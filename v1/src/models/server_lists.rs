use crate::schema::server_lists;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable)]
#[primary_key(slid)]
pub struct ServerList {
    pub slid: i64,
    pub name: String,
    pub country_code: String,
    pub area: String,
    pub ip: String,
    pub port: i16,
    pub server_type: i16, //1:chat,2:api
    pub state: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "server_lists"]
pub struct NewServerList<'a> {
    pub slid: i64,
    pub name: &'a str,
    pub country_code: &'a str,
    pub area: &'a str,
    pub ip: &'a str,
    pub port: i16,
    pub server_type: i16, //1:chat,2:api
    pub state: i16,
}
