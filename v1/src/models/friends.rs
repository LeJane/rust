use crate::schema::friends;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(fid)]
pub struct Friend {
    pub fid: i64,
    pub uuid_a: i64,
    pub uuid_b: i64,
    pub state: i16, //1:send request,2:pass
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "friends"]
pub struct NewFriend {
    pub fid: i64,
    pub uuid_a: i64,
    pub uuid_b: i64,
    pub state: i16, //1:send request 2:pass
}
