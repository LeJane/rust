use crate::schema::gem_relateds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable)]
#[primary_key(grid)]
pub struct GemRelated {
    pub grid: i64,
    pub obj_id: i64, //player id/equipment id
    pub gid: i64,
    pub obj_type: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "gem_relateds"]
pub struct NewGemRelated {
    pub grid: i64,
    pub obj_id: i64,
    pub gid: i64,
    pub obj_type: i16,
}
