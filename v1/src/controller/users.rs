use crate::default_log_pre;
use crate::facades::users;
use crate::utils::binary_read_helper::{binary_read_i32, binary_read_i64, binary_read_string};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::{error, info};

#[named]
pub async fn create_user(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    info!("{}", default_log_pre!(req.code, ""));

    let return_user_info = users::create_user(&master_conn)?;

    req.get_bincode(200, "Success", return_user_info)
}

#[named]
pub async fn get_user_base_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;
    let master_conn = req.db_conn(true)?;

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

    let user_base_data = users::get_user_base_info(&conn, uid)?;

    //update login time and login day
    if let Err(e) = users::update_user_login_time_and_login_day(&master_conn, uid) {
        error!(
            "{}\t update user login time or login day error:{}",
            default_log_pre!(req.code, uid),
            e,
        )
    }

    req.get_bincode(200, "Success", user_base_data)
}

#[named]
pub async fn update_user_action_force(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let action_points = binary_read_i32(&mut cursor)?;

    info!(
        "{}\tsubmit content\tuuid:{}\taction_points:{}",
        default_log_pre!(req.code, uid),
        uid,
        action_points,
    );

    users::update_user_action_points(&master_conn, uid, action_points)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn update_user_name(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let name = binary_read_string(&mut cursor, req.body.as_slice())?;

    info!(
        "{}\tsubmit content\tuuid:{}\tname:{}",
        default_log_pre!(req.code, uid),
        uid,
        name
    );

    users::update_user_name(&master_conn, uid, &name)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn get_user_action_points(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

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

    let data = users::get_update_user_action_points(&master_conn, uid)?;

    req.get_bincode(200, "Success", data)
}
