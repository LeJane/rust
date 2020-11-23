use crate::schema::purchase_orders;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(oid)]
pub struct PurchaseOrder {
    pub oid: i64,
    pub obj_id: i64,
    pub obj_type: i16,
    //1->first recharge,2->super bundle,3->gem store,4->supply station
    pub uuid: i64,
    pub hash: String,
    pub product_number: String,
    pub pay_platform: i16,
    pub order_no: i64,
    pub status: i16,
    //order status：-1=failed 0=default，1=success,
    pub pay_time: i64,
    pub price: f32,
    pub request_receipt_data: String,
    pub response_receipt_data: String,
    pub deleted_time: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "purchase_orders"]
pub struct NewPurchaseOrder<'a> {
    pub oid: i64,
    pub obj_id: i64,
    pub obj_type: i16,
    pub uuid: i64,
    pub hash: &'a str,
    pub product_number: &'a str,
    pub pay_platform: i16,
    pub order_no: i64,
    pub status: i16,
    pub pay_time: i64,
    pub price: f32,
    pub request_receipt_data: &'a str,
    pub response_receipt_data: &'a str,
    pub deleted_time: i64,
}
