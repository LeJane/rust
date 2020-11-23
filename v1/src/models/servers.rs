use crate::schema::servers;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize, Identifiable, Queryable)]
#[primary_key(sid)]
pub struct Server {
    pub sid: i64,
    pub server_number: i32,
    pub name: String,
    pub person_count: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Default)]
#[table_name = "servers"]
pub struct NewServer<'a> {
    pub sid: i64,
    pub server_number: i32,
    pub name: &'a str,
    pub person_count: i32,
}
