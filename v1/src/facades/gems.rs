use super::versions;
use crate::{models::gems::Gem, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_gem_list(conn: &PgConnection, version_id: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Gem>(conn, version_id)
}
