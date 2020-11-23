use crate::default_log_pre;
use crate::facades::user_assets;
use crate::utils::binary_read_helper::{binary_read_i32, binary_read_i64};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_user_asset_info(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

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

    let user_asset_data = user_assets::get_front_display_user_assets(&conn, uid)?;

    req.get_bincode(200, "Success", user_asset_data)
}

#[named]
pub async fn update_user_asset_info(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let gold_amounts = binary_read_i32(&mut cursor)?;

    let gem_amounts = binary_read_i32(&mut cursor)?;

    info!(
        "{}\tsubmit content\tuuid:{}\tgold_amounts:{}\tgem_amounts:{}",
        default_log_pre!(req.code, uid),
        uid,
        gold_amounts,
        gem_amounts,
    );

    user_assets::update_user_asset_count(&master_conn, uid, gold_amounts, gem_amounts, 0, 0, 0)?;

    req.get_bincode(200, "Success", "")
}
