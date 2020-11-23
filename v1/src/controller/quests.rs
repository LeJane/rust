use crate::default_log_pre;
use crate::facades::quests;
use crate::{utils::binary_read_helper::binary_read_i64, ReqContext, ResponseResult};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_quests_metadata_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = quests::get_quests_metadata_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_quest_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = quests::get_quest_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_quest_attribute_asset_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = quests::get_quest_attribute_asset_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}

#[named]
pub async fn get_quests_relation_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = quests::get_quests_relation_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}
