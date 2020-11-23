use crate::schema::vip_levels;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable, Serialize,Deserialize,Queryable)]
#[primary_key(level)]
pub struct VipLevel {
    pub level: i64,
    pub vip_points_needed: i32,
    pub free_treasure_chest_item_id: i64,
    pub pay_treasure_chest_item_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "vip_levels"]
pub struct NewVipLevel {
    pub level: i64,
    pub vip_points_needed: i32,
    pub free_treasure_chest_item_id: i64,
    pub pay_treasure_chest_item_id: i64,
}
