use crate::facades::servers;
use crate::{
    default_log_pre, utils::binary_read_helper::binary_read_i64, ReqContext, ResponseResult,
};
use function_name::named;
use tracing::info;

#[named]
pub async fn get_servers(ctx: ReqContext) -> ResponseResult {
    let diesel_conn = ctx.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(ctx.body.as_slice());

    let version = binary_read_i64(&mut cursor)?;

    info!(
        "{}\tsubmit content\tversion:{}",
        default_log_pre!(ctx.code, ""),
        version,
    );

    let data = servers::get_servers(&diesel_conn, version)?;

    ctx.get_bincode(200, "Success", data)
}
