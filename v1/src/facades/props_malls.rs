use crate::facades::versions;
use crate::front_models::{
    props_malls::FrontDisplayPropsMallBuyState, versions::FrontDisplayMetaVersion,
};
use crate::models::{props_malls::PropsMall, props_mall_assets::PropsMallAsset, user_buy_props_mall_records::UserBuyPropsMallRecord};
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use std::collections::HashMap;
use crate::front_models::props_malls::FrontDisplayPropsMallSuperBundleBuyState;

pub fn get_props_mall_metadata_list(
    conn: &PgConnection,
    version: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsMall>(conn, version)
}

pub fn get_props_mall_asset_list(
    conn: &PgConnection,
    version: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsMallAsset>(conn, version)
}


pub fn get_front_display_props_mall_first_recharge_state(
    conn: &PgConnection,
    uid: i64,
) -> Result<FrontDisplayPropsMallBuyState> {
    let mall = PropsMall::get_props_mall_by_mall_type(conn, 1081000)
        .map_err(|e| anyhow!("failed get props mall data:{}", e))?;

    let mut is_buy = 1;

    if let Ok(_v) = UserBuyPropsMallRecord::exist_user_buy_props_mall_item(
        &conn,
        uid,
        mall.item_id,
        mall.mall_type,
        false,
    ) {
        is_buy = 2;
    }

    Ok(FrontDisplayPropsMallBuyState {
        item_id: mall.item_id,
        is_buy,
    })
}

pub fn get_front_display_props_mall_super_value_bundles(
    conn: &PgConnection,
    uuid: i64,
) -> Result<Vec<FrontDisplayPropsMallSuperBundleBuyState>> {
    let malls = PropsMall::get_props_mall_list_by_mall_type(conn, 1081001)
        .map_err(|e| anyhow!("failed get props data:{}", e))?;

    let item_category_levels: HashMap<i32, i64> =
        UserBuyPropsMallRecord::get_super_value_bundle_category_levels(conn, uuid)
            .map_err(|e| anyhow!("failed get super value bundle category level:{}", e))?;

    let mut super_value_bundles = Vec::with_capacity(malls.len());

    for mall in malls.into_iter() {
        let mut level = 1;
        let mut is_buy = 1;
        if let Some(v) = item_category_levels.get(&mall.item_category) {
            level = *v;
            is_buy = 2;
        }

        if mall.level == level {
            let mut purchase_limit = mall.purchase_limit;

            if let Ok(v) = UserBuyPropsMallRecord::get_super_value_bundle_purchase_limit(conn, uuid, mall.item_id, level) {
                purchase_limit = v;
            }

            let d = FrontDisplayPropsMallSuperBundleBuyState {
                item_id: mall.item_id,
                purchase_limit,
                is_buy,
            };

            super_value_bundles.push(d);
        }
    }

    Ok(super_value_bundles)
}

pub fn get_front_display_daily_special_offers(
    conn: &PgConnection,
    uuid: i64,
) -> Result<Vec<FrontDisplayPropsMallBuyState>> {
    let malls = PropsMall::get_props_mall_list_by_mall_type(conn, 1081002)
        .map_err(|e| anyhow!("failed get props mall data:{}", e))?;

    let mut daily_special_offers = Vec::with_capacity(malls.len());

    for mall in malls.into_iter() {
        let mut is_buy = 1;

        if let Ok(exists) = UserBuyPropsMallRecord::exist_user_buy_props_mall_item(
            conn,
            uuid,
            mall.item_id,
            mall.mall_type,
            true,
        ) {
            if exists {
                is_buy = 2;
            }
        }

        let d = FrontDisplayPropsMallBuyState {
            item_id: mall.item_id,
            is_buy,
        };

        daily_special_offers.push(d)
    }

    Ok(daily_special_offers)
}

pub fn get_front_display_supply_station(
    conn: &PgConnection,
    uuid: i64,
) -> Result<Vec<FrontDisplayPropsMallBuyState>> {
    let malls = PropsMall::get_props_mall_list_by_mall_type(conn, 1081004)
        .map_err(|e| anyhow!("failed get props data:{}", e))?;

    let mut supply_stations = Vec::with_capacity(malls.len());

    for mall in malls.into_iter() {
        let mut is_buy = 1;

        if let Ok(exists) = UserBuyPropsMallRecord::exist_user_buy_props_mall_item(
            conn,
            uuid,
            mall.item_id,
            mall.mall_type,
            true,
        ) {
            if exists {
                is_buy = 2;
            }
        }

        let d = FrontDisplayPropsMallBuyState {
            item_id: mall.item_id,
            is_buy,
        };

        supply_stations.push(d);
    }

    Ok(supply_stations)
}
