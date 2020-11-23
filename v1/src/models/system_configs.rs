use crate::schema::system_configs;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(scid)]
pub struct SystemConfig {
    pub scid: i64,
    pub key: String,
    pub value: String,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "system_configs"]
pub struct NewSystemConfig<'a> {
    pub scid: i64,
    pub key: &'a str,
    pub value: &'a str,
}
