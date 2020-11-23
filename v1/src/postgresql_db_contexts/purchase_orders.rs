use crate::get_guid_value;
use crate::models::purchase_orders::{NewPurchaseOrder, PurchaseOrder};
use crate::schema::purchase_orders;
use diesel::prelude::*;

impl PurchaseOrder {
    pub fn new(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
        uuid: i64,
        product_number: &str,
        pay_platform: i16,
        price: f32,
    ) -> QueryResult<Self> {
        let data = NewPurchaseOrder {
            oid: get_guid_value(),
            obj_id,
            obj_type,
            uuid,
            hash: "",
            product_number,
            pay_platform,
            order_no: get_guid_value(),
            status: 0,
            pay_time: 0,
            price,
            request_receipt_data: "",
            response_receipt_data: "",
            deleted_time: 0,
        };

        diesel::insert_into(purchase_orders::table)
            .values(data)
            .get_result(conn)
    }

    pub fn check_order_info_by_order_no(conn: &PgConnection, hash: &str) -> QueryResult<bool> {
        use diesel::dsl::exists;
        let f = purchase_orders::table.filter(purchase_orders::hash.eq(hash));

        diesel::select(exists(f)).get_result(conn)
    }
    pub fn get_order_info_by_order_no(conn: &PgConnection, order_no: i64) -> QueryResult<Self> {
        purchase_orders::table
            .filter(purchase_orders::order_no.eq(order_no))
            .get_result(conn)
    }
}
