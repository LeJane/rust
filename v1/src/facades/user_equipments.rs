use crate::front_models::user_equipments::FrontDisplayUserEquipment;
use crate::models::{
    equipments::Equipment, user_assets::UserAsset, user_equipments::UserEquipment,
};
use anyhow::{Error, Result};
use diesel::prelude::*;

pub fn get_user_equipment_list(
    conn: &PgConnection,
    uid: i64,
    limit: i64,
    offset: i64,
) -> QueryResult<Vec<FrontDisplayUserEquipment>> {
    UserEquipment::get_front_display_user_equipment_list(conn, uid, limit, offset)
}

pub fn create_user_default_equipments(conn: &PgConnection, uid: i64) -> QueryResult<()> {
    //get default equipment id
    let eid_list = Equipment::get_default_equipment_id_list(conn)?;

    UserEquipment::create_user_default_equipments(conn, uid, eid_list)?;

    Ok(())
}

pub fn user_buy_equipment(
    conn: &PgConnection,
    eid: i64,
    uid: i64,
    gold_amounts: i32,
) -> Result<()> {
    conn.transaction::<(), Error, _>(|| {
        UserAsset::update_user_asset_count(&conn, uid, -gold_amounts, 0, 0, 0, 0)?;

        UserEquipment::user_buy_equipment(conn, eid, uid)?;

        Ok(())
    })?;

    Ok(())
}
