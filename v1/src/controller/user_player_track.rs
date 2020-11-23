use crate::default_log_pre;
use crate::facades::user_player_tracks;
use crate::utils::binary_read_helper::{binary_read_f32, binary_read_i64};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn insert_or_update_player_track(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let pid = binary_read_i64(&mut cursor)?;
    if pid <= 0 {
        return Err(anyhow!("invalid pid."));
    }

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let rotation_x = binary_read_f32(&mut cursor)?;
    let rotation_y = binary_read_f32(&mut cursor)?;
    let rotation_z = binary_read_f32(&mut cursor)?;
    let location_x = binary_read_f32(&mut cursor)?;
    let location_y = binary_read_f32(&mut cursor)?;
    let location_z = binary_read_f32(&mut cursor)?;

    info!("{}\tsubmit content\tpid:{}\tuid:{}\trotation_x:{}\trotation_y:{}\trotation_z:{}\tlocation_x:{}\tlocation_y:{}\tlocation_z:{}",
          default_log_pre!(req.code, uid),
          pid,
          uid,
          rotation_x,
          rotation_y,
          rotation_z,
          location_x,
          location_y,
          location_z,
    );

    user_player_tracks::insert_or_update_player_track(
        &master_conn,
        pid,
        uid,
        rotation_x,
        rotation_y,
        rotation_z,
        location_x,
        location_y,
        location_z,
    )?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn get_player_track(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let pid = binary_read_i64(&mut cursor)?;
    if pid <= 0 {
        return Err(anyhow!("invalid player id."));
    }
    let uid = binary_read_i64(&mut cursor)?;

    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tpid:{}\tuid:{}",
        default_log_pre!(req.code, uid),
        pid,
        uid
    );

    let track = user_player_tracks::get_front_display_player_track(&conn, pid, uid)?;

    req.get_bincode(200, "Success", track)
}
