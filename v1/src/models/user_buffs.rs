use crate::schema::user_buffs;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(bid)]
pub struct UserBuff {
    pub bid: i64,
    pub obj_id: i64,   //uuid/allience/kingdom
    pub obj_type: i16, //1,2,3
    pub buff_id: i64,
    pub buff_amounts: i32,
    pub buff_category: i32,
    pub buff_type: i32,
    pub sub_buff_type: i32,
    pub buff_source: i32,
    pub is_show: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "user_buffs"]
pub struct NewUserBuff {
    pub bid: i64,
    pub obj_id: i64,   //uuid/allience/kingdom
    pub obj_type: i16, //1,2,3
    pub buff_id: i64,
    pub buff_amounts: i32,
    pub buff_category: i32,
    pub buff_type: i32,
    pub sub_buff_type: i32,
    pub buff_source: i32,
    pub is_show: i16,
}
