use crate::default_log_pre;
use crate::facades::user_players;
use crate::utils::binary_read_helper::binary_read_i64;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_user_default_player_data(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let user_player_data = user_players::get_user_default_player_data(&conn, uid as i64)?;

    req.get_bincode(200, "Success", user_player_data)
}

#[named]
pub async fn get_player_collection_data(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    let pid = binary_read_i64(&mut cursor)?;

    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if pid <= 0 {
        return Err(anyhow!("invalid player id."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}\tpid:{}",
        default_log_pre!(req.code, uid),
        uid,
        pid
    );

    let data = user_players::get_player_data_collection_by_pid(&conn, uid as i64, pid as i64)?;

    req.get_bincode(200, "Success", data)
}
