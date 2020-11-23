use crate::default_log_pre;
use crate::facades::user_equipments;
use crate::utils::binary_read_helper::{binary_read_i16, binary_read_i32, binary_read_i64};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_user_equipment_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let page = binary_read_i16(&mut cursor)?;
    let uid = binary_read_i64(&mut cursor)?;

    let limit = 20;

    let offset = limit * (page as i64);

    if uid <= 0 {
        return Err(anyhow!("invalid uid."));
    }

    info!(
        "{}\tsubmit content\tpage:{}\tuuid:{}",
        default_log_pre!(req.code, uid),
        page,
        uid
    );

    let user_equipment_list = user_equipments::get_user_equipment_list(&conn, uid, limit, offset)?;

    req.get_bincode(200, "Success", user_equipment_list)
}

#[named]
pub async fn user_buy_equipment(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let eid = binary_read_i64(&mut cursor)?;
    let uid = binary_read_i64(&mut cursor)?;
    let gold_amounts = binary_read_i32(&mut cursor)?;

    if uid <= 0 || eid <= 0 {
        return Err(anyhow!("invalid param."));
    }

    if gold_amounts < 0 {
        return Err(anyhow!("invalid  golds."));
    }

    info!(
        "{}\tsubmit content\teid:{}\tuuid:{}\tgold_amounts:{}",
        default_log_pre!(req.code, uid),
        eid,
        uid,
        gold_amounts,
    );

    user_equipments::user_buy_equipment(&conn, eid, uid, gold_amounts)?;

    req.get_bincode(200, "Success", "")
}
