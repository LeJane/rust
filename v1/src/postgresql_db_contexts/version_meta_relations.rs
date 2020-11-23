use crate::models::version_meta_relations::VersionMetaRelation;
use crate::schema::version_meta_relations;
use diesel::prelude::*;

impl VersionMetaRelation {
    pub fn get_version_meta_relation_table_id_by_version(
        conn: &PgConnection,
        version: i64,
    ) -> QueryResult<Vec<i32>> {
        version_meta_relations::table
            .filter(version_meta_relations::version_id.eq(version))
            .select(version_meta_relations::table_id)
            .get_results(conn)
    }

    pub fn get_version_meta_relation_by_version_id(
        conn: &PgConnection,
        version_id: i64,
        table_id: i32,
    ) -> QueryResult<Self> {
        version_meta_relations::table
            .filter(version_meta_relations::version_id.eq(version_id))
            .filter(version_meta_relations::table_id.eq(table_id))
            .get_result(conn)
    }
}
