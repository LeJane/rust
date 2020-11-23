use crate::schema::version_meta_relations;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(operation_id)]
pub struct VersionMetaRelation {
    pub operation_id: i64,
    pub version_id: i64,  //year*month*day*hour*minute*sec
    pub update_type: i16, //1:Incremental update,2:Full update
    pub table_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "version_meta_relations"]
pub struct NewVersionMetaRelation {
    pub operation_id: i64,
    pub version_id: i64,  //year*month*day*hour*minute*sec
    pub update_type: i16, //1:Incremental update,2:Full update
    pub table_id: i32,
}
