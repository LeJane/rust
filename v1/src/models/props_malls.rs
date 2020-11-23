use crate::schema::props_malls;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[primary_key(item_id)]
pub struct PropsMall {
    pub item_id: i64,
    pub next_item_id: i64,
    pub level: i64,
    pub item_category: i32,
    pub price: f32,
    pub purchase_limit: i16,
    pub small_icon: String,
    pub gem_amounts: i32,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub first_buy_handsel: i32,
    pub late_buy_handsel: i32,
    pub valid_period_day: i16,
    pub mall_type: i32, //
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_malls"]
pub struct NewPropsMall<'a> {
    pub item_id: i64,
    pub next_item_id: i64,
    pub level: i64,
    pub item_category: i32,
    pub price: f32,
    pub purchase_limit: i16,
    pub small_icon: &'a str,
    pub gem_amounts: i32,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub first_buy_handsel: i32,
    pub late_buy_handsel: i32,
    pub valid_period_day: i16,
    pub mall_type: i32,
}
