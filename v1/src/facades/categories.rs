use super::versions;
use crate::{models::categories::Category, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::*;

pub fn get_category_metadata_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Category>(conn, version_id)
}
