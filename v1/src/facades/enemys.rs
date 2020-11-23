use super::versions;
use crate::{models::enemys::Enemy, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::*;

pub fn get_enemy_data_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Enemy>(conn, version_id)
}
