use super::versions;
use crate::{models::gem_relateds::GemRelated, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_gem_relation_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<GemRelated>(conn, version_id)
}
