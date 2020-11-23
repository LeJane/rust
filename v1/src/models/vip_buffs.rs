use crate::schema::vip_buffs;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable)]
#[primary_key(id)]
pub struct VipBuff {
    pub id: i64,
    pub level: i64,
    pub buff_id: i64,
    pub is_new_mark: i16, //1not 2yes
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "vip_buffs"]
pub struct NewVipBuff {
    pub id: i64,
    pub level: i64,
    pub buff_id: i64,
    pub is_new_mark: i16,
}
