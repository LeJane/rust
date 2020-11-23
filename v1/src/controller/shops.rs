use crate::{
    default_log_pre,
    facades::{shop_vip_metadatas, shops, users},
    utils::binary_read_helper::binary_read_i64,
    ReqContext, ResponseResult,
};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_shop_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let version_id = binary_read_i64(&mut cursor)?;
    if version_id <= 0 {
        return Err(anyhow!("invalid version id."));
    }

    info!(
        "{}submit content\tversion_id{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let data = shops::get_shop_list(&conn, version_id)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_shop_vip_metadata_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let version_id = binary_read_i64(&mut cursor)?;
    if version_id <= 0 {
        return Err(anyhow!("invalid version id."));
    }

    info!(
        "{}submit content\tversion_id{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let data = shop_vip_metadatas::get_shop_vip_metadata_list(&conn, version_id)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn buy_shop_item_by_gems(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let sid = binary_read_i64(&mut cursor)?;
    if sid <= 0 {
        return Err(anyhow!("invalid sid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tuid:{}\tsid:{}",
        default_log_pre!(req.code, uid),
        uid,
        sid
    );

    shops::buy_shop_item_by_gems(&master_conn, uid, sid)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn buy_shop_vip_item(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let shop_vip_id = binary_read_i64(&mut cursor)?;
    if shop_vip_id <= 0 {
        return Err(anyhow!("invalid shop_vip_id."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tuid:{}\tshop_vip_id:{}",
        default_log_pre!(req.code, uid),
        uid,
        shop_vip_id
    );

    shop_vip_metadatas::buy_shop_vip_item(&master_conn, uid, shop_vip_id)?;

    req.get_bincode(200, "Success", "")
}
