use crate::front_models::users::{FrontDisplayUser, FrontDisplayUserActionPoint};
use crate::models::players::Player;
use crate::models::{
    equipments::Equipment, servers::Server, system_configs::SystemConfig, user_assets::UserAsset,
    user_buffs::UserBuff, user_counters::UserCounter, user_equipments::UserEquipment,
    user_players::UserPlayer, user_queues::UserQueue, users::User,
};
use anyhow::{anyhow, bail, Context, Error, Result};
use chrono::Utc;
use diesel::prelude::*;

pub fn get_update_user_action_points(
    conn: &PgConnection,
    uid: i64,
) -> Result<FrontDisplayUserActionPoint> {
    let user_data = User::get_user_info(conn, uid)?;
    //
    let action_point_recovery = SystemConfig::get_system_config(conn, "Action Point Recovery")
        .context("Action Point Recovery system config not found.")?;

    let one_point_every_sec = action_point_recovery.value.parse::<i8>()? as i32;

    //get action points limit
    let added_max_action_points_limit = UserBuff::get_buff_amounts_sum(conn, 1, 2, 3, 4)?;

    //get recover speed
    let recover_speed = UserBuff::get_buff_amounts_sum(conn, 1, 4, 4, 5)?;

    let last_recovery_speed = one_point_every_sec - ((one_point_every_sec * recover_speed) / 100);

    let max_action_points = user_data.max_action_points + added_max_action_points_limit;

    let reduce_action_point = max_action_points - user_data.action_points;

    let t = Utc::now();

    let mut recover_time = (last_recovery_speed * reduce_action_point) as i64;

    let mut action_points = user_data.action_points;

    if reduce_action_point > 0 && user_data.action_points_latest_timestamp > 0 {
        let mut time_diff = t.timestamp() - user_data.action_points_latest_timestamp;

        if time_diff < 0 {
            time_diff = 0;
        }

        let total_update_ap = (time_diff as i32) / last_recovery_speed;

        let mut result_ap = user_data.action_points + total_update_ap;

        if result_ap > max_action_points {
            result_ap = max_action_points;
        }

        //update ap
        action_points = result_ap;

        User::update_user_action_points(conn, uid, result_ap)
            .map_err(|e| anyhow!("failed update user {} action points:{}", uid, e))?;

        if result_ap >= max_action_points {
            recover_time = 0;
        }
    }

    let time_diff = (t.timestamp() - user_data.action_points_latest_timestamp) as i32;

    if time_diff >= last_recovery_speed {
        User::update_user_action_points_latest_timestamp(conn, uid, t.timestamp())?;
    }

    let data = FrontDisplayUserActionPoint {
        uuid: uid,
        action_points,
        max_action_points,
        one_point_every_sec: last_recovery_speed as i8,
        recover_speed,
        recover_time,
    };

    Ok(data)
}

pub fn create_user(conn: &PgConnection) -> Result<FrontDisplayUser> {
    let server = Server::get_server_socket_latest(conn)
        .map_err(|e| anyhow!("Failed to get server socket:{}", e))?;

    let eid_list = Equipment::get_default_equipment_id_list(conn)?;

    let default_player = Player::get_is_default_player(conn, 2)?;

    let user = match conn.transaction::<User, Error, _>(|| {
        let uid_counter = UserCounter::insert_and_get_id(conn)?;

        let user = User::create_user(conn, uid_counter, server.server_number)?;

        Server::update_server_person_count(conn, server.sid, server.person_count + 1)?;

        UserAsset::create_user_assets(conn, user.uuid)?;

        //create user default player
        UserPlayer::create_user_default_player(conn, user.uuid, default_player)?;

        //create user default equipment
        UserEquipment::create_user_default_equipments(conn, user.uuid, eid_list)?;

        //create user default queues
        UserQueue::create_user_default_queue(conn, user.uuid)?;

        Ok(user)
    }) {
        Ok(v) => v,
        Err(e) => {
            return Err(anyhow!("failed create user info:{}", e));
        }
    };

    let return_user_info = FrontDisplayUser {
        uuid: user.uuid,
        uid: user.uid,
        name: user.name,
        avatar: user.avatar,
        server_id: user.server_id,
        action_points: user.action_points,
        gold_amounts: 0,
        gem_amounts: 0,
        food_amounts: 0,
        wood_amounts: 0,
        stone_amounts: 0,
    };

    Ok(return_user_info)
}

pub fn get_user_info(conn: &PgConnection, uuid: i64) -> Result<User> {
    User::get_user_info(conn, uuid).map_err(|e| anyhow!("failed get user {} info {}", uuid, e))
}

pub fn get_user_base_info(conn: &PgConnection, uuid: i64) -> Result<FrontDisplayUser> {
    let user = User::get_user_info(conn, uuid)
        .map_err(|e| anyhow!("failed get user {} info {}", uuid, e))?;

    let user_asset = UserAsset::get_user_assets(conn, uuid)
        .map_err(|e| anyhow!("failed get user {} asset info {}", uuid, e))?;

    let d = FrontDisplayUser {
        uuid,
        uid: user.uid,
        name: user.name,
        avatar: user.avatar,
        server_id: user.server_id,
        action_points: user.action_points,
        gold_amounts: user_asset.gold_amounts,
        gem_amounts: user_asset.gem_amounts,
        food_amounts: user_asset.food_amounts,
        wood_amounts: user_asset.wood_amounts,
        stone_amounts: user_asset.stone_amounts,
    };

    Ok(d)
}

pub fn update_user_action_points(conn: &PgConnection, uid: i64, action_points: i32) -> Result<()> {
    let old_action_points = get_update_user_action_points(conn, uid)?;

    if action_points < 0 && (old_action_points.action_points + action_points) < 0 {
        bail!("user action force not enough.");
    }

    User::update_user_action_points(conn, uid, action_points)?;

    Ok(())
}

pub fn update_user_name(conn: &PgConnection, uid: i64, name: &str) -> Result<()> {
    User::update_user_name(conn, uid, name)
}

pub fn update_user_login_time_and_login_day(conn: &PgConnection, uid: i64) -> Result<()> {
    User::update_user_login_time_and_login_day(conn, uid)
}

pub fn find_user_exists(conn: &PgConnection, uid: i64) -> QueryResult<bool> {
    User::find_user_exists(conn, uid)
}
