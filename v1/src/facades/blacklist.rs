use crate::front_models::blacklist::FrontDisplayBlacklist;
use crate::models::{blacklist::Blacklist, users::User};
use diesel::prelude::*;

pub fn get_user_black_list(
    conn: &PgConnection,
    uid: i64,
) -> QueryResult<Vec<FrontDisplayBlacklist>> {
    let black_lists = Blacklist::get_black_list(conn, uid)?;

    let mut front_display_black_list = Vec::new();

    for list in black_lists.into_iter() {
        let user = User::get_front_display_chat_user_info(conn, list.uuid_b)?;

        let data = FrontDisplayBlacklist {
            bid: list.bid,
            user,
        };

        front_display_black_list.push(data);
    }

    Ok(front_display_black_list)
}

pub fn add_user_to_black_list(conn: &PgConnection, uid: i64, add_uid: i64) -> QueryResult<()> {
    Blacklist::add_user_to_black_list(conn, uid, add_uid)
}

pub fn find_user_black_list_exists(
    conn: &PgConnection,
    uid: i64,
    black_uid: i64,
) -> QueryResult<bool> {
    Blacklist::find_user_black_list_exists(conn, uid, black_uid)
}

pub fn del_user_black(conn: &PgConnection, uuid_a: i64, uuid_b: i64) -> QueryResult<()> {
    Blacklist::del_user_black(conn, uuid_a, uuid_b)
}
