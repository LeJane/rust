use crate::facades::versions;
use crate::{
    default_log_pre, utils::binary_read_helper::binary_read_i64, ReqContext, ResponseResult,
};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_version_update_data(req: ReqContext) -> ResponseResult {
    let diesel_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let version = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion:{}",
        default_log_pre!(req.code, ""),
        version,
    );

    let data = versions::get_meta_version_relation_by_version(&diesel_conn, version)?;

    req.get_bincode(200, "Success", data)
}
