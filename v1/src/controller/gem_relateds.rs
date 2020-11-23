use crate::default_log_pre;
use crate::facades::gem_relateds;
use crate::{utils::binary_read_helper::binary_read_i64, ReqContext, ResponseResult};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_gem_relation_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = gem_relateds::get_gem_relation_list(&conn, version_id)?;

    req.get_bincode(200, "Success", list)
}
