use crate::schema::equipments;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(eid)]
pub struct Equipment {
    pub eid: i64,
    pub kid: i64,
    pub name: String,
    pub thumbnail: String,
    pub price: i32,
    pub hp: i32,
    pub multiplier: f32,
    pub kind: i16,
    //1:weapons,2:armors,3:shields
    pub is_default: i16,
    //1:No,2:Yes
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "equipments"]
pub struct NewEquipment<'a> {
    pub eid: i64,
    pub kid: i64,
    pub name: &'a str,
    pub thumbnail: &'a str,
    pub price: i32,
    pub hp: i32,
    pub multiplier: f32,
    pub kind: i16,
    pub is_default: i16, //1:No,2:Yes
}
