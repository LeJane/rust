use crate::schema::version_meta_operation_relations;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct VersionMetaOperationRelation {
    pub id: i64,
    pub operation_id: i64,
    pub version_id: i64,
    pub action_id: i64,   //table's primary key id
    pub action_type: i32, //0:default,1:insert,2:update,3:del
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "version_meta_operation_relations"]
pub struct NewVersionMetaOperationRelation {
    pub id: i64,
    pub operation_id: i64,
    pub version_id: i64,
    pub action_id: i64,
    pub action_type: i32,
}
