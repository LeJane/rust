use crate::default_log_pre;
use crate::facades::friends;
use crate::utils::binary_read_helper::{binary_read_i16, binary_read_i32, binary_read_i64};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

//my friend list
#[named]
pub async fn get_user_friend_list(req: ReqContext) -> ResponseResult {
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let friend_list = friends::get_user_friend_list(&slave_conn, uid, 2)?;

    req.get_bincode(200, "Success", friend_list)
}

// get my new friend list
#[named]
pub async fn get_user_new_friend_list(req: ReqContext) -> ResponseResult {
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let friend_list = friends::get_user_new_friend_list(&slave_conn, uid, 1)?;

    req.get_bincode(200, "Success", friend_list)
}

#[named]
pub async fn search_friend_by_uid(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let search_id = binary_read_i32(&mut cursor)?;
    if search_id <= 0 {
        return Err(anyhow!("invalid search uid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tsearch's uuid:{}",
        default_log_pre!(req.code, uid),
        uid,
        search_id,
    );

    let data = friends::search_friend_by_uid(&conn, uid, search_id)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn send_friend_request(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let added_uid = binary_read_i32(&mut cursor)?;
    if added_uid <= 0 {
        return Err(anyhow!("invalid friend uuid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tadded uuid:{}",
        default_log_pre!(req.code, uid),
        uid,
        added_uid,
    );

    friends::send_friend_request(&master_conn, uid, added_uid)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn update_friend_state(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let fid = binary_read_i64(&mut cursor)?;
    if fid <= 0 {
        return Err(anyhow!("invalid fid."));
    }

    let state = binary_read_i16(&mut cursor)?;

    if state != 2 && state != 3 {
        return Err(anyhow!("invalid state."));
    }

    info!(
        "{}\tsubmit content\tfid:{}\tstate:{}",
        default_log_pre!(req.code, ""),
        fid,
        state,
    );

    friends::update_friend_state(&master_conn, fid, state)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn del_friend(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid fid."));
    }
    let deleted_id = binary_read_i64(&mut cursor)?;
    if deleted_id <= 0 {
        return Err(anyhow!("invalid deleted_id."));
    }
    info!(
        "{}\tsubmit content\tuuid:{}\tdeleted uuid:{}",
        default_log_pre!(req.code, uid),
        uid,
        deleted_id,
    );

    friends::del_friend(&master_conn, uid, deleted_id)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn get_special_user_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let my_uid = binary_read_i64(&mut cursor)?;
    if my_uid <= 0 {
        return Err(anyhow!("invalid fid."));
    }
    let dst_uid = binary_read_i64(&mut cursor)?;
    if dst_uid <= 0 {
        return Err(anyhow!("invalid dst_id."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tdst uuid:{}",
        default_log_pre!(req.code, my_uid),
        my_uid,
        dst_uid,
    );

    let data = friends::get_special_user_info(&conn, my_uid, dst_uid)?;

    req.get_bincode(200, "Success", data)
}
