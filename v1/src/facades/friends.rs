use crate::front_models::{
    friends::FrontDisplayFriend, friends::FrontDisplayGetSpecialUserFriendInfo,
    users::FrontDisplayChatUserAndFriendState,
};
use crate::models::{blacklist::Blacklist, friends::Friend, users::User};
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub fn get_user_friend_list(
    conn: &PgConnection,
    uid: i64,
    state: i16,
) -> QueryResult<Vec<FrontDisplayFriend>> {
    let friends_info = Friend::get_user_friend_list(conn, uid, state)?;

    let mut data = Vec::new();

    for friend in friends_info.into_iter() {
        if let Ok(exists) = Blacklist::find_user_black_list_exists(conn, uid, friend.uuid_b) {
            if exists {
                continue;
            }
        }

        let user_info = User::get_front_display_chat_user_info(conn, friend.uuid_b)?;
        let front_user_chat_info = FrontDisplayFriend {
            fid: friend.fid,
            user: user_info,
            state: friend.state,
        };
        data.push(front_user_chat_info);
    }

    Ok(data)
}

pub fn get_user_new_friend_list(
    conn: &PgConnection,
    uid: i64,
    state: i16,
) -> QueryResult<Vec<FrontDisplayFriend>> {
    let friends_info = Friend::get_user_new_friend_list(conn, uid, state)?;

    let mut data = Vec::new();

    for friend in friends_info.into_iter() {
        let user_info = User::get_front_display_chat_user_info(conn, friend.uuid_a)?;

        let front_user_chat_info = FrontDisplayFriend {
            fid: friend.fid,
            user: user_info,
            state: friend.state,
        };
        data.push(front_user_chat_info);
    }

    Ok(data)
}

pub fn search_friend_by_uid(
    conn: &PgConnection,
    uid: i64,
    search_id: i32,
) -> Result<FrontDisplayChatUserAndFriendState> {
    let user_info = User::get_user_info_by_uid(conn, search_id)?;

    let mut state = 0i16;

    let mut find_peer_state = false;

    if let Ok(exists) = Friend::find_friend_exists(conn, uid, user_info.uuid) {
        if exists {
            let friend_state = Friend::find_friend_state(conn, uid, user_info.uuid)?;

            if friend_state != 2 {
                state = 1
            } else {
                state = 3;
            }
        } else {
            find_peer_state = true;
        }
    }

    if find_peer_state {
        if let Ok(exists) = Friend::find_friend_exists(conn, user_info.uuid, uid) {
            if exists {
                let friend_state = Friend::find_friend_state(conn, user_info.uuid, uid)?;

                if friend_state != 2 {
                    state = 2
                } else {
                    state = 3;
                }
            }
        }
    }

    let data = FrontDisplayChatUserAndFriendState {
        uuid: user_info.uuid,
        uid: user_info.uid,
        name: user_info.name,
        avatar: user_info.avatar,
        server_id: user_info.server_id,
        action_points: user_info.action_points,
        friend_state: state,
    };

    Ok(data)
}

pub fn send_friend_request(conn: &PgConnection, uid: i64, added_uid: i32) -> Result<()> {
    let user_info = User::get_user_info_by_uid(conn, added_uid)?;

    if Friend::find_friend_exists(conn, uid, user_info.uuid)? {
        return Err(anyhow!("friend request has exists."));
    }

    Friend::add_friend(conn, uid, user_info.uuid, 1)?;

    Ok(())
}

pub fn update_friend_state(conn: &PgConnection, fid: i64, state: i16) -> Result<()> {
    let friend_info = Friend::get_friend_by_fid(conn, fid)?;

    if friend_info.state != 1 {
        return Err(anyhow!("invalid friend info."));
    }

    if state == 2 {
        Friend::update_friend_state(conn, fid, state)?;
        if Friend::find_friend_exists(conn, friend_info.uuid_b, friend_info.uuid_a)? {
            Friend::update_friend_state_by_uid(
                conn,
                friend_info.uuid_b,
                friend_info.uuid_a,
                state,
            )?;
        } else {
            Friend::add_friend(conn, friend_info.uuid_b, friend_info.uuid_a, 2)?;
        }
    } else {
        Friend::del_friend(conn, fid)?;
    }

    Ok(())
}

pub fn del_friend(conn: &PgConnection, uid: i64, deleted_id: i64) -> Result<()> {
    Friend::del_friend_by_uuid(conn, uid, deleted_id)?;

    if let Ok(exists) = Blacklist::find_user_black_list_exists(conn, uid, deleted_id) {
        if exists {
            //
            Blacklist::del_user_black(conn, uid, deleted_id)?;
        }
    }

    Friend::del_friend_by_uuid(conn, deleted_id, uid)?;

    if let Ok(exists) = Blacklist::find_user_black_list_exists(conn, deleted_id, uid) {
        if exists {
            //
            Blacklist::del_user_black(conn, deleted_id, uid)?;
        }
    }

    Ok(())
}

pub fn get_special_user_info(
    conn: &PgConnection,
    my_uid: i64,
    dst_uid: i64,
) -> Result<FrontDisplayGetSpecialUserFriendInfo> {
    //get user info
    let user_info = User::get_front_display_chat_user_info(conn, dst_uid)
        .map_err(|e| anyhow!("failed get user {} info:{}", dst_uid, e))?;

    let mut friend_state = 0;
    let mut fid = 0;
    let mut bid = 0;
    let mut blacklist_state = 0;
    let mut find_peer_friend_state = false;

    if let Ok(exists) = Friend::find_friend_exists(conn, my_uid, dst_uid) {
        if exists {
            let friend_info = Friend::get_friend_info_by_fid(conn, my_uid, dst_uid)?;

            if friend_info.state != 2 {
                friend_state = 1;
                find_peer_friend_state = true;
            } else {
                friend_state = 3;
            }

            fid = friend_info.fid;
        } else {
            find_peer_friend_state = true;
        }
    }

    if find_peer_friend_state {
        if let Ok(exists) = Friend::find_friend_exists(conn, dst_uid, my_uid) {
            if exists {
                let friend_info = Friend::get_friend_info_by_fid(conn, dst_uid, my_uid)?;

                if friend_info.state != 2 {
                    friend_state = 2;
                } else {
                    friend_state = 3;
                }

                fid = friend_info.fid;
            }
        }
    }

    if let Ok(exists) = Blacklist::find_user_black_list_exists(conn, my_uid, dst_uid) {
        if exists {
            blacklist_state = 1;
            bid = Blacklist::find_user_black_list_bid(conn, my_uid, dst_uid)?;
        }
    }

    let data = FrontDisplayGetSpecialUserFriendInfo {
        fid,
        bid,
        user: user_info,
        friend_state,
        blacklist_state,
    };

    Ok(data)
}
