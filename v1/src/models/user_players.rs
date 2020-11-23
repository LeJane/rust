use crate::schema::user_players;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct UserPlayer {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub is_default: i16, //1:unactived,2:actived
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_players"]
pub struct NewUserPlayer {
    pub id: i64,
    pub pid: i64,
    pub uid: i64,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
    pub is_default: i16,
}
