pub mod helper;

use byteorder::{LittleEndian, WriteBytesExt};
use diesel::prelude::*;
use helper::{build_header_req, get_tcp_conn};
use std::io::{Cursor};
use v1::{deserialize_binary, FrontDisplayMetaVersion, get_guid_value};
use v1::models::{
    gems::Gem,
    skills::Skill,
};
use v1::front_models::{
    versions::FrontDisplayVersion,
    users::FrontDisplayUser,
    user_equipments::FrontDisplayUserEquipment,
    friends::FrontDisplayGetSpecialUserFriendInfo,
    users::FrontDisplayUserActionPoint,
    user_item_bags::FrontDisplayUserItemBag,
    user_vips::FrontDisplayUserVip,
    user_vips::FrontDisplayBuyVipPointsData,
    vip_daily_login_treasure_chests::FrontDisplayVipDailyLoginTreasureChest,
};
use v1::schema::{gems, skills};
use v1::utils::binary_read_helper::*;
use v1::utils::diesel_db::get_diesel_pool;

#[tokio::test]
async fn get_server_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();
        let req_ctx = build_header_req(1000, body);
        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let server_lists: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&server_lists).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn create_user() {
    let req = || -> Vec<u8> {
        let body = vec![];
        let req_ctx = build_header_req(1001, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let user_info: FrontDisplayUser = deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&user_info).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_user_equipment_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i16::<LittleEndian>(0).unwrap();
        body.write_i64::<LittleEndian>(6457364932904658384).unwrap();
        let req_ctx = build_header_req(1002, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let equipment_data: Vec<FrontDisplayUserEquipment> = deserialize_binary(&mut cursor, body).unwrap();
        let res = serde_json::to_string(&equipment_data).expect("failed json encode.");

        println!("Content:{}", res);
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_skills_gems_data_list() {
    let conn = get_diesel_pool().get().unwrap();

    //skills
    let skill_list = skills::table.load::<Skill>(&conn).unwrap();
    let gem_list = gems::table.load::<Gem>(&conn).unwrap();

    let skill_str = serde_json::to_string(&skill_list).unwrap();
    let gem_str = serde_json::to_string(&gem_list).unwrap();

    dbg!(skill_str, gem_str);
}

#[tokio::test]
async fn get_friend_special_user_info() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        body.write_i64::<LittleEndian>(3857456648315068608).unwrap();
        let req_ctx = build_header_req(1038, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let friend_info: FrontDisplayGetSpecialUserFriendInfo =
                deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&friend_info).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn generate_skills_gems_data_list() {
    let conn = get_diesel_pool().get().unwrap();

    let gem_list = gems::table
        .filter(gems::gid.eq_any(vec![
            9169467747588979885,
            1194675840908803999,
            3230165357958183206,
            2401662050616474945,
            8932747538749248592,
            4106114089184243024,
            6428662726616975724,
            1209330646817269805,
            8374855459150100114,
            8672005677990751628,
        ]))
        .load::<Gem>(&conn)
        .unwrap();

    let mut sqls = vec![];

    for gem in gem_list.into_iter() {
        let sql = format!(
            "update gems set gem_icon={:?},gem_selected_material={:?},gem_link_material={:?} where gid={}",
            gem.gem_icon, gem.gem_selected_material, gem.gem_link_material, gem.gid,
        );

        sqls.push(sql);
    }

    dbg!(sqls);
}

#[tokio::test]
async fn get_user_action_point() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        let req_ctx = build_header_req(1041, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            println!("{}", item_length);
            let action_point_info: FrontDisplayUserActionPoint =
                deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&action_point_info).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_user_item_bag_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        let req_ctx = build_header_req(1042, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_bag_list: Vec<FrontDisplayUserItemBag> =
            deserialize_binary(&mut cursor, body).unwrap();

        let res = serde_json::to_string(&item_bag_list).expect("failed json encode.");

        println!("Content:{}", res);
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_user_vip_data() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        body.write_i16::<LittleEndian>(1).unwrap();
        let req_ctx = build_header_req(1043, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            println!("{}", item_length);
            let data: FrontDisplayUserVip = deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_user_base_info() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        let req_ctx = build_header_req(1008, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            println!("{}", item_length);
            let data: FrontDisplayUser = deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn receive_vip_exclusive_free_treasure_chest() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        body.write_i64::<LittleEndian>(902267455082729342).unwrap(); //etcid
        let req_ctx = build_header_req(1044, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        println!("No Content");
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn receive_vip_daily_login_treasure_chest() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        let req_ctx = build_header_req(1045, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            println!("{}", item_length);
            let data: FrontDisplayVipDailyLoginTreasureChest =
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
async fn get_buy_vip_points_data_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(902267455082729342).unwrap();
        let req_ctx = build_header_req(1046, body);
        req_ctx
    };
    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);

        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            println!("{}", item_length);
            let data: FrontDisplayBuyVipPointsData = deserialize_binary(&mut cursor, body).unwrap();

            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn get_user_buy_mall_data_list() {
    use v1::get_master_diesel_pool;
    use v1::schema::user_buy_props_mall_records;

    let conn = get_master_diesel_pool().get().unwrap();
    let uuid = 333;
    let mut item_category_slice = vec![];

    let res = user_buy_props_mall_records::table
        .filter(user_buy_props_mall_records::uuid.eq(uuid))
        .filter(user_buy_props_mall_records::expire_time.gt(1234567880))
        .filter(user_buy_props_mall_records::mall_type.eq(2))
        .select((
            user_buy_props_mall_records::item_category,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("max(level) AS max"),
        ))
        .group_by(user_buy_props_mall_records::item_category)
        .load::<(i32, i64)>(&conn)
        .unwrap()
        .into_iter()
        .map(|v| {
            item_category_slice.push(v.0);
            v
        })
        .collect::<std::collections::HashMap<i32, i64>>();

    dbg!(res, item_category_slice);
}


#[tokio::test]
async fn insert_into_version_meta_relation() {
    let v = vec![2043,
                 2042,
                 2006,
                 2007,
                 2028,
                 2030,
                 2029,
                 2031,
                 2032,
                 2033,
                 2034,
                 2035,
                 2036,
                 2037,
                 2038,
                 2039,
                 2040,
                 2041,
                 2044,
                 2045,
                 2046,
                 2047,
                 2048,
                 2049,
                 2050,
                 2051,
                 2052,
                 2053,
                 2054,
                 2055,
                 2056,
                 2057,
                 2058,
                 2059,
                 2060,
                 2061,
                 2062,
                 2063];

    for vv in v.into_iter() {
        println!("insert into version_meta_relations values({},20201120113045,2,{});", get_guid_value(), vv);
    }
}


#[tokio::test]
async fn metadata_get_equipment_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();
        let req_ctx = build_header_req(1053, body);
        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}


#[tokio::test]
async fn metadata_get_buff_metadata_list() {
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();
        let req_ctx = build_header_req(1054, body);
        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_gem_relation_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1055, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_gem_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1056, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_player_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1057, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_item_metadata_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1058, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_action_points_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1059, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_boost_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1060, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_builder_recruitment_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1061, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_fixed_treasure_chest_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1062, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_fixed_treasure_chest_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1063, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_key_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1064, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_props_product_number_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1065, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_random_treasure_chest_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1066, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_random_treasure_chest_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1067, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_random_treasure_chest_attribute_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1068, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_resources_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1069, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_speed_up_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1070, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_starlight_sculpture_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1071, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_tome_of_knowledge_props_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1072, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_props_mall_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1073, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_quests_metadata_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1074, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_quest_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1075, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_quest_attribute_asset_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1076, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_quests_relation_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1077, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_servers()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1078, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_skill_relation_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1079, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_skill_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1080, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_version_update_data()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1081, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_vip_buff_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1082, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_daily_login_treasure_chest_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1083, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_vip_level_list()

{
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1084, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn metadata_get_category_metadata_list(){
    let req = || -> Vec<u8> {
        let mut body = vec![];
        body.write_i64::<LittleEndian>(20201120113045).unwrap();

        let req_ctx = build_header_req(1085, body);

        req_ctx
    };

    let res = |body: &[u8]| {
        let mut cursor = Cursor::new(body);
        binary_read_msg(&mut cursor, body);
        let item_length = binary_read_i32(&mut cursor).unwrap();
        if item_length > 0 {
            let data: FrontDisplayMetaVersion = deserialize_binary(&mut cursor, body).unwrap();
            let res = serde_json::to_string(&data).expect("failed json encode.");

            println!("Content:{}", res);
        } else {
            println!("No Content");
        }
    };

    get_tcp_conn(req, res).await;
}

#[tokio::test]
async fn time_format(){
    let t=chrono::Utc::now();


    dbg!(t.format("%Y.%m.%d-%H.%M.%S").to_string().as_str());
}