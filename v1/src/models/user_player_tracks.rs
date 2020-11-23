use crate::schema::user_player_tracks;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(tid)]
pub struct UserPlayerTrack {
    pub tid: i64,
    pub pid: i64,
    pub uid: i64,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "user_player_tracks"]
pub struct NewUserPlayerTrack {
    pub tid: i64,
    pub pid: i64,
    pub uid: i64,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
}
