pub mod helper;
use chrono::{Datelike, Utc};
use helper::{build_header_req, get_tcp_conn};
use std::io::Cursor;
use v1::front_models::props_malls::{
    FrontDisplayPropsMallBuyState,
    FrontDisplayPropsMallSuperBundleBuyState,
};
use v1::{deserialize_binary, utils::binary_read_helper::*};

#[tokio::test]
async fn mall_get_first_recharge_gift_data() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        binary_write_i64(&mut body, 902267455082729342).unwrap();
        binary_write_i16(&mut body, 1).unwrap();
        let req_ctx = build_header_req(1034, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayPropsMallBuyState =
                deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn mall_get_super_value_bundle_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        binary_write_i64(&mut body, 902267455082729342).unwrap();
        binary_write_i16(&mut body, 1).unwrap();
        let req_ctx = build_header_req(1039, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let list: Vec<FrontDisplayPropsMallSuperBundleBuyState> =
            deserialize_binary(&mut cursor, body).unwrap();
        let res = serde_json::to_string(&list).expect("failed json encode.");

        println!("Content:{}", res);
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn mall_get_daily_special_offer_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        binary_write_i64(&mut body, 902267455082729342).unwrap();
        binary_write_i16(&mut body, 1).unwrap();
        let req_ctx = build_header_req(1040, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let list: Vec<FrontDisplayPropsMallBuyState> =
            deserialize_binary(&mut cursor, body).unwrap();
        let res = serde_json::to_string(&list).expect("failed json encode.");

        println!("Content:{}", res);
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn mall_get_supply_station_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        binary_write_i16(&mut body, 1).unwrap();
        let req_ctx = build_header_req(1036, body);
        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let list: Vec<FrontDisplayPropsMallBuyState> =
            deserialize_binary(&mut cursor, body).unwrap();
        let res = serde_json::to_string(&list).expect("failed json encode.");

        println!("Content:{}", res);
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn read_binary_data() {
    let body = vec![214, 7, 0, 0];
    let mut cursor = Cursor::new(body.as_slice());
    let length = binary_read_i32(&mut cursor).unwrap();

    println!("body:{:?}", format!("{:?}", body));

    dbg!("length={}", length);
}

#[tokio::test]
async fn write_binary_data() {
    let mut body = vec![];
    // let mut cursor = Cursor::new(body.as_slice());
    binary_write_i64(&mut body, 25003).unwrap();

    println!("body:{:?}", format!("{:?}", body));
}

#[tokio::test]
async fn get_week() {
    let t = Utc::now();

    dbg!((t.year() * 100) as u32 + t.iso_week().week());
}
