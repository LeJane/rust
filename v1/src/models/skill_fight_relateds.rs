use crate::schema::skill_fight_relateds;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable)]
pub struct SkillFightRelated {
    pub id: i64,
    pub obj_id: i64,
    pub skill_id: i64,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub probability: i16,
    pub level: i16,
    pub level_experience: i32,
    pub obj_type: i16,
    //player id/equipment id/enemy id->1,2,3
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "skill_fight_relateds"]
pub struct NewSkillFigthRelated {
    pub id: i64,
    pub obj_id: i64,
    pub skill_id: i64,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub probability: i16,
    pub level: i16,
    pub level_experience: i32,
    pub obj_type: i16,
}
