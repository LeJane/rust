use crate::schema::shop_vip_metadatas;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Deserialize, Serialize, Debug, Clone)]
#[primary_key(shop_vip_id)]
pub struct ShopVipMetadata {
    pub shop_vip_id: i64,
    pub level: i64,
    pub item_id: i64,
    pub bag_type: i32,
    pub sub_item_type: i32,
    pub cost_value: i32,
    pub attribute_id: i32, //1:gem,2:food,3:wood
    pub discount: f32,
    pub limit_amounts: i32,
    pub order_value: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "shop_vip_metadatas"]
pub struct NewShopVipMetadata {
    pub shop_vip_id: i64,
    pub level: i64,
    pub item_id: i64,
    pub bag_type: i32,
    pub sub_item_type: i32,
    pub cost_value: i32,
    pub attribute_id: i32,
    pub discount: f32,
    pub limit_amounts: i32,
    pub order_value: i64,
}
