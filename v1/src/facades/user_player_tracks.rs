use crate::front_models::user_player_tracks::FrontDisplayUserPlayerTrack;
use crate::models::user_player_tracks::UserPlayerTrack;
use diesel::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn insert_or_update_player_track(
    conn: &PgConnection,
    pid: i64,
    uid: i64,
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    location_x: f32,
    location_y: f32,
    location_z: f32,
) -> QueryResult<()> {
    UserPlayerTrack::insert_or_update_player_track(
        conn, pid, uid, rotation_x, rotation_y, rotation_z, location_x, location_y, location_z,
    )
}

pub fn get_front_display_player_track(
    conn: &PgConnection,
    pid: i64,
    uid: i64,
) -> QueryResult<FrontDisplayUserPlayerTrack> {
    UserPlayerTrack::get_front_display_player_track(conn, pid, uid)
}
