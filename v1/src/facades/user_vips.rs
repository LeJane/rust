use crate::front_models::{
    user_vips::{
        FrontDisplayBuyVipPointsData, FrontDisplayBuyVipPointsSuccess, FrontDisplayUserVip,
    },
    vip_daily_login_treasure_chests::FrontDisplayVipDailyLoginTreasureChest,
};
use crate::models::{
    props_resources_categories::PropsResourcesCategory, shops::Shop, user_assets::UserAsset,
    user_buffs::UserBuff, user_item_bags::UserItemBag, user_queues::UserQueue, user_vips::UserVip,
    users::User, vip_buffs::VipBuff, vip_daily_login_treasure_chests::VipDailyLoginTreasureChest,
    vip_levels::VipLevel,
};

use crate::facades::props_fixed_treasure_chest_categories;
use crate::models::buff_metadatas::BuffMetadata;

use anyhow::{anyhow, Error, Result};
use chrono::{Datelike, Utc};
use diesel::prelude::*;

pub fn receive_vip_exclusive_free_treasure_chest(
    conn: &PgConnection,
    uid: i64,
    item_id: i64,
) -> Result<()> {
    let user_vip = get_user_vip_data(&conn, uid)?;

    let t = Utc::now();
    let ftc = chrono::NaiveDateTime::from_timestamp(user_vip.free_everyday_treasure_chests_time, 0);

    if t.day() == ftc.day() {
        return Err(anyhow!("today has receive."));
    }

    let t = Utc::now();

    conn.transaction::<(), Error, _>(|| {
        props_fixed_treasure_chest_categories::get_treasure_chest_data_and_into_item_bags(
            conn, uid, item_id,
        )?;

        UserVip::update_user_vip_treasure_chest_time(conn, uid, 2, t.timestamp())?;
        Ok(())
    })
}

pub fn buy_vip_points_by_gem_or_item(
    conn: &PgConnection,
    uid: i64,
    id: i64,
    buy_type: i16,
) -> Result<FrontDisplayBuyVipPointsSuccess> {
    let user_vip_data = get_user_vip_data(conn, uid)?;

    let mut new_vip_points = user_vip_data.vip_points;

    match buy_type {
        1 => {
            //item bag
            let item_bag_data = UserItemBag::get_user_item_bag_by_id(conn, id)
                .map_err(|e| anyhow!("failed get item bag info:{}", e))?;

            if item_bag_data.count <= 0 {
                return Err(anyhow!("vip points item count is zero"));
            }

            let shop_data = Shop::get_shop_by_id(conn, id)?;

            let resources_data =
                PropsResourcesCategory::get_resources_props_by_item_id(conn, shop_data.item_id)?;

            new_vip_points += resources_data.rss_value;

            conn.transaction::<(), Error, _>(|| {
                UserVip::update_user_vip_points(conn, uid, new_vip_points)?;

                if item_bag_data.overlay_status == 1 {
                    UserItemBag::update_user_item_bag_count_by_bid(conn, id, -1)?;
                } else {
                    UserItemBag::delete_user_item_bag_by_id(conn, id)?;
                }

                Ok(())
            })
            .map_err(|e| anyhow!("{}", e))?;
        }
        2 => {
            //gems
            // check user gems enough
            let user_asset = UserAsset::get_user_assets(conn, uid)?;

            let shop_data = Shop::get_shop_by_id(conn, id)?;

            if user_asset.gem_amounts < shop_data.gems_needed {
                return Err(anyhow!("user gems asset not enough"));
            }

            let resources_data =
                PropsResourcesCategory::get_resources_props_by_item_id(conn, shop_data.item_id)?;

            new_vip_points += resources_data.rss_value;

            conn.transaction::<(), Error, _>(|| {
                UserVip::update_user_vip_points(conn, uid, new_vip_points)?;

                UserAsset::update_user_asset_count(conn, uid, 0, -shop_data.gems_needed, 0, 0, 0)?;

                Ok(())
            })
            .map_err(|e| anyhow!("{}", e))?;
        }
        _ => {
            return Err(anyhow!("Unknown buy type."));
        }
    }

    //check vip point change
    let (_level_change, level) =
        check_user_vip_points_level_change(conn, &user_vip_data, uid, new_vip_points)?;

    Ok(FrontDisplayBuyVipPointsSuccess {
        vip_points: new_vip_points,
        level,
    })
}

pub fn get_front_display_buy_vip_points_data(
    conn: &PgConnection,
    uid: i64,
) -> Result<FrontDisplayBuyVipPointsData> {
    let mut item_bags = Vec::new();

    if let Ok(v) = UserItemBag::get_front_display_user_item_bag_list(conn, uid) {
        item_bags = v;
    }

    let data = FrontDisplayBuyVipPointsData { item_bags };

    Ok(data)
}

pub fn get_vip_points_and_update_user_vip_points(
    conn: &PgConnection,
    uid: i64,
) -> Result<FrontDisplayVipDailyLoginTreasureChest> {
    let user_vip = get_user_vip_data(conn, uid)?;

    let t = Utc::now();
    let dtc = chrono::NaiveDateTime::from_timestamp(user_vip.daily_treasure_chests_time, 0);

    if t.day() == dtc.day() {
        return Err(anyhow!("today has receive."));
    }

    //check level >=max_level
    let max_level = VipLevel::get_max_vip_level(conn)?;

    if user_vip.level >= max_level {
        return Err(anyhow!("max level can't receive."));
    }

    let user_info = User::get_user_info(conn, uid)?;

    let vip_points =
        VipDailyLoginTreasureChest::get_front_daily_login_treasure_chest_by_continuous_login_days(
            conn,
            user_info.login_days,
        )?;

    let new_vip_points = user_vip.vip_points + (vip_points.today_vip_points as i32);

    //check vip level
    UserVip::update_user_vip_points(conn, uid, new_vip_points)?;

    let (_level_change, res_level) =
        check_user_vip_points_level_change(conn, &user_vip, uid, new_vip_points)?;

    Ok(FrontDisplayVipDailyLoginTreasureChest {
        continuous_login_days: vip_points.continuous_login_days,
        today_vip_points: vip_points.today_vip_points,
        tomorrow_vip_points: vip_points.tomorrow_vip_points,
        level: res_level,
    })
}

pub fn check_user_vip_points_level_change(
    conn: &PgConnection,
    user_vip: &UserVip,
    uid: i64,
    new_vip_points: i32,
) -> Result<(bool, i64)> {
    let vip_level_info = VipLevel::get_vip_level_by_level(conn, user_vip.level)?;

    let mut res_level = user_vip.level;
    let mut is_change = false;

    let max_level = VipLevel::get_max_vip_level(conn)?;

    if new_vip_points >= vip_level_info.vip_points_needed && vip_level_info.level < max_level {
        //level change
        conn.transaction::<(), Error, _>(|| {
            //update user vip level
            res_level = UserVip::update_user_vip_level(conn, uid, user_vip.level + 1)?;

            //add user exclusive buff to economic buff table
            let buffs = VipBuff::get_vip_buffs(conn, vip_level_info.level)?;

            for buff in buffs.into_iter() {
                let buff_meta = BuffMetadata::get_buff_metadata_by_id(conn, buff.buff_id)?;

                if buff_meta.sub_buff_type == 2 {
                    UserQueue::update_user_building_queue(conn, uid, buff_meta.amounts as i16)?;
                } else {
                    UserBuff::add_user_buff(
                        conn,
                        uid,
                        1,
                        buff_meta.buff_id,
                        buff_meta.amounts,
                        buff_meta.buff_category,
                        buff_meta.buff_type,
                        buff_meta.sub_buff_type,
                        buff_meta.buff_source,
                        buff_meta.is_show,
                    )?;
                }
            }

            Ok(())
        })
        .map_err(|e| anyhow!("{}", e))?;

        is_change = true;
    }

    Ok((is_change, res_level))
}

pub fn get_user_vip_data(conn: &PgConnection, uuid: i64) -> Result<UserVip> {
    UserVip::get_user_vip_data(conn, uuid)
}

pub fn get_user_vip_front_data(conn: &PgConnection, uuid: i64) -> Result<FrontDisplayUserVip> {
    let user_info =
        User::get_user_info(conn, uuid).map_err(|e| anyhow!("failed get user info:{:?}", e))?;

    let user_vip_data = match UserVip::get_user_vip_data(conn, uuid) {
        Ok(v) => v,
        Err(e) => {
            if e.to_string() == diesel::NotFound.to_string() {
                UserVip::create_user_vip(conn, uuid)
                    .map_err(|e| anyhow!("failed create user vip data:{:?}", e))?
            }
            return Err(anyhow!("failed get user vip data:{:?}", e));
        }
    };

    let t = Utc::now();

    let mut daily_treasure_chests_status = 1;
    let mut free_everyday_treasure_chests_status = 1;
    let mut special_privileges_treasure_chests_status = 1;

    let dtc = chrono::NaiveDateTime::from_timestamp(user_vip_data.daily_treasure_chests_time, 0);
    let ftc =
        chrono::NaiveDateTime::from_timestamp(user_vip_data.free_everyday_treasure_chests_time, 0);
    let spec = chrono::NaiveDateTime::from_timestamp(
        user_vip_data.special_privileges_treasure_chests_time,
        0,
    );

    if dtc.day() == t.day() {
        daily_treasure_chests_status = 2;
    }

    if ftc.day() == t.day() {
        free_everyday_treasure_chests_status = 2;
    }

    if spec.day() == t.day() {
        special_privileges_treasure_chests_status = 2;
    }

    let data = FrontDisplayUserVip {
        id: user_vip_data.id,
        login_days: user_info.login_days,
        level: user_vip_data.level,
        vip_points: user_vip_data.vip_points,
        daily_treasure_chests_status,
        free_everyday_treasure_chests_status,
        special_privileges_treasure_chests_status,
    };

    Ok(data)
}
