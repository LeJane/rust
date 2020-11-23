use crate::schema::equipment_kinds;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable)]
#[primary_key(kid)]
pub struct EquipmentKind {
    pub kid: i64,
    pub name: String,
    pub kind: i16, //1:weapons,2:armors,3:shields
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "equipment_kinds"]
pub struct NewEquipmentKind<'a> {
    pub kid: i64,
    pub name: &'a str,
    pub kind: i16,
}
