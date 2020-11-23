use crate::{
    default_log_pre,
    facades::{props_malls, users},
    utils::binary_read_helper::binary_read_i64,
    ReqContext, ResponseResult,
};

use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_props_mall_metadata_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let version = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion:{}",
        default_log_pre!(req.code, ""),
        version,
    );

    let data = props_malls::get_props_mall_metadata_list(&conn, version)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_props_mall_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_malls::get_props_mall_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_first_recharge_gift_data(req: ReqContext) -> ResponseResult {
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
        "{}\tsubmit content\tuid:{}",
        default_log_pre!(req.code, uid),
        uid,
    );

    let data = props_malls::get_front_display_props_mall_first_recharge_state(&conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_super_value_bundle_list(req: ReqContext) -> ResponseResult {
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
        uid,
    );

    let data = props_malls::get_front_display_props_mall_super_value_bundles(&slave_conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_daily_special_offer_list(req: ReqContext) -> ResponseResult {
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
        uid,
    );

    let data = props_malls::get_front_display_daily_special_offers(&slave_conn, uid)?;

    req.get_bincode(200, "Success", data)
}

#[named]
pub async fn get_supply_station_list(req: ReqContext) -> ResponseResult {
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
        "{}\tsubmit content uid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let data = props_malls::get_front_display_supply_station(&conn, uid)?;

    req.get_bincode(200, "Success", data)
}

//
// #[named]
// pub async fn buy_first_recharge_gift_data(req: ReqContext) -> ResponseResult {
//     let slave_conn = req.db_conn(false)?;
//
//     //check order_no
//     let mut cursor = std::io::Cursor::new(req.body.as_slice());
//
//     let uuid = binary_read_i64(&mut cursor)?;
//     let hash = binary_read_string(&mut cursor, &req.body)?;
//
//     ensure!(
//         PurchaseOrder::check_order_info_by_order_no(&slave_conn, hash.as_str())?,
//         "hash has exists."
//     );
//
//     let order_no = binary_read_i64(&mut cursor)?;
//
//     let order_info = PurchaseOrder::get_order_info_by_order_no(&slave_conn, order_no)?;
//     if order_info.uuid != uuid {
//         bail!("invalid user uuid.");
//     }
//
//     //receipt
//     let receipt = binary_read_string(&mut cursor, &req.body)?;
//     if receipt.is_empty() {
//         bail!("invalid receipt data.");
//     }
//
//     info!(
//         "{}\tsubmit content\tuuid:{}\thash:{}\torder_no:{}\treceipt:{}",
//         default_log_pre!(req.code, uuid),
//         uuid,
//         hash,
//         order_no,
//         receipt,
//     );
//
//     let receipt_slice = ReqClient::new("", false).verify(receipt).await?;
//
//     let _receipt_info: Receipt = serde_json::from_slice(&receipt_slice)?;
//
