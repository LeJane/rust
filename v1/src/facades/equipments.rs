use super::versions;
use crate::{
    models::equipments::Equipment, models::user_equipments::UserEquipment, FrontDisplayMetaVersion,
};
use crate::front_models::equipments::FrontDisplayEquipment;
use anyhow::Result;
use diesel::prelude::*;

pub fn get_equipment_list(conn: &PgConnection, version_id: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Equipment>(conn, version_id)
}

pub fn get_equipment_data_by_id(conn: &PgConnection, eid: i64) -> QueryResult<Equipment> {
    Equipment::get_equipment_data_by_id(conn, eid)
}

pub fn get_shop_equipment_data_by_kid(
    conn: &PgConnection,
    kid: i64,
    uid: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<FrontDisplayEquipment>> {
    let equipment_list = Equipment::get_equipment_list_by_kid_page(conn, kid, limit, offset)?;

    let mut filter_equipments = Vec::new();

    for equipment in equipment_list.into_iter() {
        if let Ok(exists) = UserEquipment::find_user_equipment_exists(conn, uid, equipment.eid) {
            if exists {
                continue;
            }
        }

        let d=FrontDisplayEquipment{
            eid:equipment.eid,
        };

        filter_equipments.push(d);
    }

    Ok(filter_equipments)
}
