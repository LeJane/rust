use crate::schema::shops;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Deserialize, Serialize, Debug, Clone)]
#[primary_key(sid)]
pub struct Shop {
    pub sid: i64,
    pub item_id: i64,
    pub bag_type: i32,
    pub sub_item_type: i32,
    pub order_value: i64,
    pub gems_needed: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "shops"]
pub struct NewShop {
    pub sid: i64,
    pub item_id: i64,
    pub bag_type: i32,
    pub sub_item_type: i32,
    pub order_value: i64,
    pub gems_needed: i32,
}
