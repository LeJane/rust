use crate::models::player_mount_equipments::PlayerMountEquipment;
use anyhow::Result;
use diesel::prelude::*;

pub fn get_player_mount_equipment_list(
    conn: &PgConnection,
    pid: i64,
    uid: i64,
) -> QueryResult<Vec<PlayerMountEquipment>> {
    PlayerMountEquipment::get_player_mount_equipment_list(conn, pid, uid)
}

pub fn mount_user_player_equipment(
    conn: &PgConnection,
    pid: i64,
    uid: i64,
    equipment_id: i64,
) -> Result<()> {
    PlayerMountEquipment::mount_user_player_equipment(conn, pid, uid, equipment_id)?;

    Ok(())
}

pub fn umount_user_player_equipment(
    conn: &PgConnection,
    pid: i64,
    uid: i64,
    equipment_id: i64,
) -> Result<()> {
    PlayerMountEquipment::umount_user_player_equipment(conn, pid, uid, equipment_id)?;

    Ok(())
}

pub fn switch_user_player_equipment(
    conn: &PgConnection,
    id: i64,
    uid: i64,
    equipment_id: i64,
) -> Result<()> {
    PlayerMountEquipment::switch_user_player_equipment(conn, id, uid, equipment_id)?;
    Ok(())
}
