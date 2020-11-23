use crate::facades::{user_item_bags,versions};
use crate::get_iso_week;
use crate::{
    models::{
        props_item_metadatas::PropsItemMetadata, shop_vip_metadatas::ShopVipMetadata,
        user_assets::UserAsset, user_buy_shop_vip_records::UserBuyShopVipRecord, user_vips::UserVip,
    },
    FrontDisplayMetaVersion
};
use anyhow::{anyhow, bail, Error, Result};
use diesel::prelude::*;

pub fn get_shop_vip_metadata_list(conn:&PgConnection,version:i64)->Result<FrontDisplayMetaVersion>{
    versions::get_meta_version_relation_type_data::<ShopVipMetadata>(conn,version)
}

pub fn buy_shop_vip_item(conn: &PgConnection, uuid: i64, shop_vip_id: i64) -> Result<()> {
    let iso_week = get_iso_week();
    let shop_vip_data = ShopVipMetadata::get_shop_vip_item_by_id(conn, shop_vip_id)
        .map_err(|e| anyhow!("failed get shop vip data:{}", e))?;

    let (mut exists_user_buy_vip_record, mut exists_user_buy_vip_record_id) = (false, 0);

    match UserBuyShopVipRecord::get_user_buy_shop_vip_by_week(conn, uuid, shop_vip_id, iso_week) {
        Ok(v) => {
            exists_user_buy_vip_record = true;
            exists_user_buy_vip_record_id = v.id;
            if v.amounts >= shop_vip_data.limit_amounts {
                bail!("purchase limit reached.")
            }
        }
        Err(e) => {
            if e != diesel::NotFound {
                bail!("server internal error:{}", e)
            }
        }
    }

    let user_vip_data = UserVip::get_user_vip_data(conn, uuid)
        .map_err(|e| anyhow!("failed get user vip data:{}", e))?;

    if user_vip_data.level < shop_vip_data.level {
        bail!("user vip level not enough.")
    }

    let user_asset = UserAsset::get_user_assets(conn, uuid)?;

    let (mut gem_amounts, mut food_amounts, mut wood_amounts) = (0, 0, 0);

    match shop_vip_data.attribute_id {
        1 => {
            gem_amounts = shop_vip_data.cost_value;
            if user_asset.gem_amounts < shop_vip_data.cost_value {
                bail!("gem amounts not enough.")
            }
        }
        2 => {
            food_amounts = shop_vip_data.cost_value;
            if user_asset.food_amounts < shop_vip_data.cost_value {
                bail!("food amounts not enough.")
            }
        }
        3 => {
            wood_amounts = shop_vip_data.cost_value;
            if user_asset.wood_amounts < shop_vip_data.cost_value {
                bail!("wood amounts not enough.")
            }
        }
        _ => bail!("Unknown cost type."),
    }

    let item_metadata = PropsItemMetadata::get_item_metadata_by_id(conn, shop_vip_data.item_id)
        .map_err(|e| anyhow!("failed get item metadata data:{}", e))?;

    conn.transaction::<(), Error, _>(|| {
        user_item_bags::check_item_bag_exists_item_and_update(conn, &item_metadata, uuid)
            .map_err(|e| anyhow!("failed item into user bag:{}", e))?;

        UserAsset::update_user_asset_count(
            conn,
            uuid,
            0,
            -gem_amounts,
            -food_amounts,
            -wood_amounts,
            0,
        )
        .map_err(|e| anyhow!("failed update user asset:{}", e))?;

        if exists_user_buy_vip_record {
            UserBuyShopVipRecord::update_user_buy_shop_vip_amounts(
                conn,
                exists_user_buy_vip_record_id,
                1,
            )
            .map_err(|e| anyhow!("failed update user buy shop vip record:{}", e))?;
        } else {
            UserBuyShopVipRecord::add_user_buy_shop_vip_record(conn, uuid, shop_vip_id, iso_week)
                .map_err(|e| anyhow!("failed add user buy shop vip record:{}", e))?;
        }

        Ok(())
    })
    .map_err(|e| anyhow!("{}", e))
}
