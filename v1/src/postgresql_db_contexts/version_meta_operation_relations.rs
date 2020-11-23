use crate::models::version_meta_operation_relations::VersionMetaOperationRelation;
use crate::schema::version_meta_operation_relations;
use diesel::prelude::*;

impl VersionMetaOperationRelation {
    pub fn get_version_meta_relation_operation_list(
        conn: &PgConnection,
        operation_id: i64,
    ) -> QueryResult<Vec<Self>> {
        version_meta_operation_relations::table
            .filter(version_meta_operation_relations::operation_id.eq(operation_id))
            .load(conn)
    }
}
