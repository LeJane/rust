use crate::models::version_meta_relations::MetaVersionRelation;
use crate::schema::meta_version_relations;
use diesel::prelude::*;

impl MetaVersionRelation {
    pub fn get_meta_version_relation_list(
        conn: &PgConnection,
        version_id: i64,
    ) -> QueryResult<Vec<Self>> {
        meta_version_relations::table
            .filter(meta_version_relations::version_id.eq(version_id))
            .load(conn)
    }
}
