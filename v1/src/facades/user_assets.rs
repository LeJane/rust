use crate::{front_models::user_assets::FrontDisplayUserAsset, models::user_assets::UserAsset};
use anyhow::{bail, Result};
use diesel::prelude::*;

pub fn get_user_assets(conn: &PgConnection, uid: i64) -> QueryResult<UserAsset> {
    UserAsset::get_user_assets(conn, uid)
}

pub fn get_front_display_user_assets(
    conn: &PgConnection,
    uid: i64,
) -> QueryResult<FrontDisplayUserAsset> {
    UserAsset::get_front_display_user_assets(conn, uid)
}

pub fn update_user_asset_count(
    conn: &PgConnection,
    uid: i64,
    gold_amounts: i32,
    gem_amounts: i32,
    food_amounts: i32,
    wood_amounts: i32,
    stone_amounts: i32,
) -> Result<()> {
    let asset = get_user_assets(conn, uid)?;

    if gold_amounts > 0 && (gold_amounts + asset.gold_amounts) < 0 {
        bail!("user gold amounts not enough.")
    }

    if gem_amounts > 0 && (gem_amounts + asset.gem_amounts) < 0 {
        bail!("user gem amounts not enough.")
    }

    if food_amounts > 0 && (food_amounts + asset.food_amounts) < 0 {
        bail!("user food amounts not enough.")
    }

    if wood_amounts > 0 && (wood_amounts + asset.wood_amounts) < 0 {
        bail!("user wood amounts not enough.")
    }

    if stone_amounts > 0 && (stone_amounts + asset.stone_amounts) < 0 {
        bail!("user stone amounts not enough.")
    }

    UserAsset::update_user_asset_count(
        conn,
        uid,
        gold_amounts,
        gem_amounts,
        food_amounts,
        wood_amounts,
        stone_amounts,
    )?;

    Ok(())
}
