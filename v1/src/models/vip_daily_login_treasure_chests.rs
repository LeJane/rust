use crate::schema::vip_daily_login_treasure_chests;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable)]
#[primary_key(id)]
pub struct VipDailyLoginTreasureChest {
    pub id: i64,
    pub continuous_login_days: i32,
    pub today_vip_points: i32,
    pub tomorrow_vip_points: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "vip_daily_login_treasure_chests"]
pub struct NewVipDailyLoginTreasureChest {
    pub id: i64,
    pub continuous_login_days: i32,
    pub today_vip_points: i32,
    pub tomorrow_vip_points: i32,
}
