use crate::default_log_pre;
use crate::facades::player_mount_equipments;
use crate::utils::binary_read_helper::binary_read_i64;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn mount_user_player_equipment(req: ReqContext) -> ResponseResult {
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

    let equipment_id = binary_read_i64(&mut cursor)?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    info!(
        "{}\tsubmit content\tpid:{}\tuid:{}\tequipment id:{}",
        default_log_pre!(req.code, uid),
        pid,
        uid,
        equipment_id,
    );

    player_mount_equipments::mount_user_player_equipment(&master_conn, pid, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn umount_user_player_equipment(req: ReqContext) -> ResponseResult {
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

    let equipment_id = binary_read_i64(&mut cursor)?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    info!(
        "{}\tsubmit content\tpid:{}\tuid:{}\tequipment id:{}",
        default_log_pre!(req.code, uid),
        pid,
        uid,
        equipment_id,
    );

    player_mount_equipments::umount_user_player_equipment(&master_conn, pid, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn switch_user_player_equipment(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let id = binary_read_i64(&mut cursor)?;
    if id <= 0 {
        return Err(anyhow!("invalid id."));
    }

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    let equipment_id = binary_read_i64(&mut cursor)?;
    if equipment_id <= 0 {
        return Err(anyhow!("invalid equipment_id."));
    }

    info!(
        "{}\tsubmit content\tid:{}\tuid:{}\tequipment id:{}",
        default_log_pre!(req.code, uid),
        id,
        uid,
        equipment_id,
    );

    player_mount_equipments::switch_user_player_equipment(&master_conn, id, uid, equipment_id)?;

    req.get_bincode(200, "Success", "")
}
