use crate::models::version_meta_relations::MetaVersion;
use crate::schema::meta_versions;
use diesel::prelude::*;

impl MetaVersion {
    pub fn get_meta_version_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        meta_versions::table.load(conn)
    }
}
