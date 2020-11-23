use crate::default_log_pre;
use crate::facades::categories;
use crate::utils::binary_read_helper::binary_read_i64;
use crate::{ReqContext, ResponseResult};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_category_metadata_list(req: ReqContext) -> ResponseResult {
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let version_id = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion_id:{}",
        default_log_pre!(req.code, ""),
        version_id
    );

    let list = categories::get_category_metadata_list(&slave_conn, version_id)?;

    req.get_bincode(200, "Success", list)
}
