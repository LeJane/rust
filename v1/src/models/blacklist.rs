use crate::schema::blacklists;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(bid)]
pub struct Blacklist {
    pub bid: i64,
    pub uuid_a: i64,
    pub uuid_b: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "blacklists"]
pub struct NewBlacklist {
    pub bid: i64,
    pub uuid_a: i64,
    pub uuid_b: i64,
}
