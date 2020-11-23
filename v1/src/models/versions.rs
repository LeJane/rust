use crate::schema::versions;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(version_id)]
pub struct Version {
    pub version_id: i64, //year*month*day*hour*minute*sec
    pub last_version_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "versions"]
pub struct NewVersion {
    pub version_id: i64, //year*month*day*hour*minute*sec
    pub last_version_id: i64,
}
