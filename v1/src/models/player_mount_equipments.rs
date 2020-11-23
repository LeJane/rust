use crate::schema::player_mount_equipments;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize, Identifiable, Queryable)]
pub struct PlayerMountEquipment {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub equipment_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "player_mount_equipments"]
pub struct NewPlayerMountEquipment {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub equipment_id: i64,
}
