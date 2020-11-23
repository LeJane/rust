use crate::default_log_pre;
use crate::facades::props_items;
use crate::{utils::binary_read_helper::binary_read_i64, ReqContext, ResponseResult};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_item_metadata_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_item_metadata_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_action_points_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_action_points_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_boost_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_boost_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_builder_recruitment_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_builder_recruitment_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_fixed_treasure_chest_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_fixed_treasure_chest_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_fixed_treasure_chest_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_fixed_treasure_chest_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_key_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_key_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_props_product_number_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_props_product_number_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_random_treasure_chest_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_random_treasure_chest_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_random_treasure_chest_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_random_treasure_chest_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_random_treasure_chest_attribute_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_random_treasure_chest_attribute_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_resources_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_resources_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_speed_up_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_speed_up_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}


#[named]
pub async fn get_starlight_sculpture_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_starlight_sculpture_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}



#[named]
pub async fn get_tome_of_knowledge_props_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = props_items::get_tome_of_knowledge_props_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

