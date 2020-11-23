use crate::schema::skills;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable)]
pub struct Skill {
    pub id: i64,
    pub thumbnail: String,
    pub skill_name: String,
    pub skill_description: String,
    pub model_path: String,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub level: i16,
    pub level_experience: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "skills"]
pub struct NewSkill<'a> {
    pub id: i64,
    pub thumbnail: &'a str,
    pub skill_name: &'a str,
    pub skill_description: &'a str,
    pub model_path: &'a str,
    pub cool_down: i32,
    pub attack_power: i32,
    pub mana_power: i32,
    pub level: i16,
    pub level_experience: i32,
}
