use crate::default_log_pre;
use crate::facades::equipments;
use crate::utils::binary_read_helper::{binary_read_i16, binary_read_i64};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_equipment_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let equipment_kind_list = equipments::get_equipment_list(&conn, version_id)?;

    req.get_bincode(200, "Success", equipment_kind_list)
}

#[named]
pub async fn get_shop_equipment_list_by_kid(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let page = binary_read_i16(&mut cursor)?;
    let kid = binary_read_i64(&mut cursor)?;
    let uid = binary_read_i64(&mut cursor)?;

    let limit = 20;

    let offset = limit * (page as i64);

    if kid <= 0 {
        return Err(anyhow!("invalid equipment kind id."));
    }

    if uid <= 0 {
        return Err(anyhow!("invalid uid."));
    }

    info!(
        "{}\tsubmit content\tpage:{}\tkid:{}\tuuid:{}",
        default_log_pre!(req.code, uid),
        page,
        kid,
        uid
    );

    let equipment_shop_list =
        equipments::get_shop_equipment_data_by_kid(&conn, kid, uid, limit, offset)?;

    req.get_bincode(200, "Success", equipment_shop_list)
}
