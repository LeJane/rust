use crate::front_models::purchase_orders::FrontDisplayPurchaseOrder;
use crate::models::purchase_orders::PurchaseOrder;
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub fn generate_new_order(
    conn: &PgConnection,
    obj_id: i64,
    obj_type: i16,
    uuid: i64,
    product_number: String,
    pay_platform: i16,
    price: f32,
) -> Result<FrontDisplayPurchaseOrder> {
    let exists = valid_obj_by_obj_type(conn, obj_id, obj_type, product_number.as_str())?;

    if !exists {
        return Err(anyhow!("data not found."));
    }

    let order = PurchaseOrder::new(
        conn,
        obj_id,
        obj_type,
        uuid,
        product_number.as_str(),
        pay_platform,
        price,
    )?;

    let data = FrontDisplayPurchaseOrder {
        order_no: order.order_no,
    };

    Ok(data)
}

pub fn valid_obj_by_obj_type(
    _conn: &PgConnection,
    _obj_id: i64,
    _obj_type: i16,
    _product_number: &str,
) -> QueryResult<bool> {
    unimplemented!()
}
