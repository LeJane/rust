use crate::schema::user_buy_shop_vip_records;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(id)]
pub struct UserBuyShopVipRecord {
    pub id: i64,
    pub uuid: i64,
    pub shop_vip_id: i64,
    pub amounts: i32,
    pub week_time: i32, //week_time=year*100+一年中的第几个周
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_buy_shop_vip_records"]
pub struct NewUserBuyShopVipRecords {
    pub id: i64,
    pub uuid: i64,
    pub shop_vip_id: i64,
    pub amounts: i32,
    pub week_time: i32, //week_time=year*100+一年中的第几个周
}
