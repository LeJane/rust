use crate::{facades::versions, models::servers::Server, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::PgConnection;

pub fn get_servers(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Server>(conn, version)
}
