use super::versions;
use crate::{models::buff_metadatas::BuffMetadata, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::*;

pub fn get_buff_metadata_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<BuffMetadata>(conn, version_id)
}
