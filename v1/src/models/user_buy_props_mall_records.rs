use crate::schema::user_buy_props_mall_records;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(rid)]
pub struct UserBuyPropsMallRecord {
    pub rid: i64,
    pub item_id: i64,
    pub uuid: i64,
    pub level: i64,
    pub item_category: i32,
    pub purchase_limit: i16,
    pub expire_time: i64,
    pub latest_receive_time: i64,
    pub mall_type: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_buy_props_mall_records"]
pub struct NewUserBuyPropsMallRecord {
    pub rid: i64,
    pub item_id: i64,
    pub uuid: i64,
    pub level: i64,
    pub item_category: i32,
    pub purchase_limit: i16,
    pub expire_time: i64,
    pub latest_receive_time: i64,
    pub mall_type: i32,
}
