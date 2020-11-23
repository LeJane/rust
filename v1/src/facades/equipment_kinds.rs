use super::versions;
use crate::{models::equipment_kinds::EquipmentKind, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_equipment_kind_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<EquipmentKind>(conn, version_id)
}
