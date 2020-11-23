use crate::default_log_pre;
use crate::facades::{blacklist, users};
use crate::utils::binary_read_helper::binary_read_i64;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn add_user_to_black_list(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    let add_uid = binary_read_i64(&mut cursor)?;
    if add_uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, add_uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tadded uuid:{}",
        default_log_pre!(req.code, uid),
        uid,
        add_uid
    );

    if blacklist::find_user_black_list_exists(&slave_conn, uid, add_uid)? {
        return Err(anyhow!("user black has exists."));
    }

    blacklist::add_user_to_black_list(&master_conn, uid, add_uid)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn get_user_black_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid,
    );

    let resp_data = blacklist::get_user_black_list(&conn, uid)?;

    req.get_bincode(200, "Success", resp_data)
}

#[named]
pub async fn del_black_list(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    let target_uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, target_uid)? {
        return Err(anyhow!("black user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tdeleted uuid:{}",
        default_log_pre!(req.code, uid),
        uid,
        target_uid,
    );

    blacklist::del_user_black(&master_conn, uid, target_uid)?;

    req.get_bincode(200, "Success", "")
}
