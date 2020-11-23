use crate::front_models::user_players::FrontDisplayUserPlayer;
use crate::models::{players::Player, user_players::UserPlayer};
use diesel::prelude::*;


pub fn get_user_default_player_data(
    conn: &PgConnection,
    uid: i64,
) -> QueryResult<FrontDisplayUserPlayer> {
    UserPlayer::get_front_display_user_default_player_data(conn, uid)
}

pub fn get_player_data_collection_by_pid(
    conn: &PgConnection,
    uid: i64,
    pid: i64,
) -> QueryResult<FrontDisplayUserPlayer> {
    UserPlayer::get_front_display_user_player_data(conn, uid, pid)
}

pub fn create_user_default_player(conn: &PgConnection, uid: i64) -> QueryResult<()> {
    //get default player
    let player = Player::get_is_default_player(conn, 2)?;

    UserPlayer::create_user_default_player(conn, uid, player)
}
