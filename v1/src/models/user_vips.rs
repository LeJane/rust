use crate::schema::user_vips;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(id)]
pub struct UserVip {
    pub id: i64,
    pub uuid: i64,
    pub level: i64,
    pub vip_points: i32,
    pub daily_treasure_chests_time: i64,
    //领取的时间辍
    pub free_everyday_treasure_chests_time: i64,
    pub special_privileges_treasure_chests_time: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "user_vips"]
pub struct NewUserVip {
    pub id: i64,
    pub uuid: i64,
    pub level: i64,
    pub vip_points: i32,
    pub daily_treasure_chests_time: i64, //领取的时间辍
    pub free_everyday_treasure_chests_time: i64,
    pub special_privileges_treasure_chests_time: i64,
}
