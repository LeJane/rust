use crate::default_log_pre;
use crate::facades::{user_vips, users};
use crate::utils::binary_read_helper::*;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_user_vip_data(req: ReqContext) -> ResponseResult {
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

    let data = user_vips::get_user_vip_front_data(&master_conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn receive_vip_exlusive_free_treasure_chest(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let item_id = binary_read_i64(&mut cursor)?;
    if item_id <= 0 {
        return Err(anyhow!("invalid treasure chest item id."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\titem_id:{}",
        default_log_pre!(req.code, uid),
        uid,
        item_id,
    );

    user_vips::receive_vip_exclusive_free_treasure_chest(&master_conn, uid, item_id)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn receive_vip_daily_login_treasure_chest(req: ReqContext) -> ResponseResult {
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

    let data = user_vips::get_vip_points_and_update_user_vip_points(&master_conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_buy_vip_points_data_list(req: ReqContext) -> ResponseResult {
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let data = user_vips::get_front_display_buy_vip_points_data(&slave_conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn buy_vip_points_by_gem_or_item(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let id = binary_read_i64(&mut cursor)?;
    if id <= 0 {
        return Err(anyhow!("invalid id."));
    }

    //1:item,2:gem

    let buy_type = binary_read_i16(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tid:{}\tbuy_type:{}",
        default_log_pre!(req.code, uid),
        uid,
        id,
        buy_type
    );

    let data = user_vips::buy_vip_points_by_gem_or_item(&master_conn, uid, id, buy_type)?;

    req.get_bincode(200, "Success", data)
}
