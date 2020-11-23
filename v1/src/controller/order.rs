use crate::default_log_pre;
use crate::facades::{orders, users};
use crate::utils::binary_read_helper::*;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn generate_new_order(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let body: &[u8] = req.body.as_ref();

    let mut cursor = std::io::Cursor::new(body);

    let uuid = binary_read_i64(&mut cursor)?;
    if uuid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uuid)? {
        return Err(anyhow!("user not exists."));
    }

    let obj_id = binary_read_i64(&mut cursor)?;
    if obj_id == 0 {
        return Err(anyhow!("invalid id."));
    }

    let obj_type = binary_read_i16(&mut cursor)?;
    if obj_type == 0 {
        return Err(anyhow!("invalid obj type."));
    }

    let product_number = binary_read_string(&mut cursor, &req.body)?;

    if product_number.is_empty() {
        return Err(anyhow!("invalid product_number."));
    }

    let pay_platform = binary_read_i16(&mut cursor)?;
    if pay_platform == 0 {
        return Err(anyhow!("invalid pay platform."));
    }

    let price = binary_read_f32(&mut cursor)?;
    if price == 0.0 {
        return Err(anyhow!("invalid price."));
    }

    info!("{}\tsubmit content\tuuid:{}\tobj_id:{}\tobj_type:{}\tproduct_number:{}\tpay_platform:{}\tprice:{}",
          default_log_pre!(req.code,uuid),
          uuid,
          obj_id,
          obj_type,
          product_number,
          pay_platform,
          price,
    );

    let data = orders::generate_new_order(
        &master_conn,
        obj_id,
        obj_type,
        uuid,
        product_number,
        pay_platform,
        price,
    )?;

    req.get_bincode(200, "Success", data)
}
