use crate::front_models::user_player_tracks::FrontDisplayUserPlayerTrack;
use crate::get_guid_value;
use crate::models::user_player_tracks::{NewUserPlayerTrack, UserPlayerTrack};
use crate::schema::user_player_tracks;
use diesel::prelude::*;

impl UserPlayerTrack {
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
        let data = NewUserPlayerTrack {
            tid: get_guid_value() as i64,
            pid,
            uid,
            rotation_x,
            rotation_y,
            rotation_z,
            location_x,
            location_y,
            location_z,
        };

        match user_player_tracks::table
            .filter(user_player_tracks::pid.eq(pid))
            .filter(user_player_tracks::uid.eq(uid))
            .select(user_player_tracks::tid)
            .first::<i64>(conn)
        {
            Ok(v) => {
                diesel::update(user_player_tracks::table)
                    .set((
                        user_player_tracks::rotation_x.eq(rotation_x),
                        user_player_tracks::rotation_y.eq(rotation_y),
                        user_player_tracks::rotation_z.eq(rotation_z),
                        user_player_tracks::location_x.eq(location_x),
                        user_player_tracks::location_y.eq(location_y),
                        user_player_tracks::location_z.eq(location_z),
                    ))
                    .filter(user_player_tracks::tid.eq(v))
                    .execute(conn)?;
            }
            Err(ref e) if e == &diesel::NotFound => {
                diesel::insert_into(user_player_tracks::table)
                    .values(data)
                    .execute(conn)?;
            }
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn get_player_track(conn: &PgConnection, pid: i64, uid: i64) -> QueryResult<Self> {
        user_player_tracks::table
            .filter(user_player_tracks::pid.eq(pid))
            .filter(user_player_tracks::uid.eq(uid))
            .get_result(conn)
    }
    pub fn get_front_display_player_track(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
    ) -> QueryResult<FrontDisplayUserPlayerTrack> {
        user_player_tracks::table
            .filter(user_player_tracks::pid.eq(pid))
            .filter(user_player_tracks::uid.eq(uid))
            .select((
                user_player_tracks::tid,
                user_player_tracks::rotation_x,
                user_player_tracks::rotation_y,
                user_player_tracks::rotation_z,
                user_player_tracks::location_x,
                user_player_tracks::location_y,
                user_player_tracks::location_z,
            ))
            .get_result(conn)
    }
}
