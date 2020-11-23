use crate::schema::user_item_bags;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(bid)]
pub struct UserItemBag {
    pub bid: i64,
    pub uuid: i64,
    pub item_id: i64,        //item_id
    pub overlay_status: i16, //1:can overlay,2:can't overlay
    pub bag_type: i32,
    pub count: i32,
    pub order_value: i64, //asc
    pub sub_item_type: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_item_bags"]
pub struct NewUserItemBag {
    pub bid: i64,
    pub uuid: i64,
    pub item_id: i64,
    pub overlay_status: i16, //1:can overlay,2:can't overlay
    pub bag_type: i32,
    pub count: i32,
    pub order_value: i64,
    pub sub_item_type: i32,
}
