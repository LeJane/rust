use crate::schema::enemys;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(eid)]
pub struct Enemy {
    pub eid: i64,
    pub enemy_name: String,
    pub model_path: String,
    pub thumbnail: String,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: String,
    pub bp_enemy: String,
    pub ap_enemy: String,
    pub skm_enemy: String,
    pub enemy_die: String,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "enemys"]
pub struct NewEnemy<'a> {
    pub eid: i64,
    pub enemy_name: &'a str,
    pub model_path: &'a str,
    pub thumbnail: &'a str,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub animation_hit_delay: f32,
    pub spawn_style_class: &'a str,
    pub bp_enemy: &'a str,
    pub ap_enemy: &'a str,
    pub skm_enemy: &'a str,
    pub enemy_die: &'a str,
}
