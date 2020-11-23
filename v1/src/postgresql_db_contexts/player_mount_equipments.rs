use crate::get_guid_value;
use crate::models::player_mount_equipments::{NewPlayerMountEquipment, PlayerMountEquipment};
use crate::schema::player_mount_equipments;
use anyhow::{anyhow, Result};
use chrono::Utc;
use diesel::prelude::*;

impl PlayerMountEquipment {
    pub fn get_player_mount_equipment_list(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
    ) -> QueryResult<Vec<Self>> {
        player_mount_equipments::table
            .filter(player_mount_equipments::pid.eq(pid))
            .filter(player_mount_equipments::uid.eq(uid))
            .get_results(conn)
    }

    pub fn mount_user_player_equipment(
        conn: &PgConnection,
        pid: i64,
        uid: i64,
        equipment_id: i64,
    ) -> Result<()> {
        let equipment_used_id: i64 = match Self::get_equipment_used_id(conn,uid,equipment_id)
        {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if equipment_used_id > 0 {
            return Err(anyhow!("user equipment has mounted."));
        }

        let data = NewPlayerMountEquipment {
            id: get_guid_value() as i64,
            pid,
            uid,
            equipment_id,
        };

        diesel::insert_into(player_mount_equipments::table)
            .values(data)
            .execute(conn)?;

        Ok(())
    }

    pub fn umount_user_player_equipment(
        conn: &PgConnection,
        pid_value: i64,
        uid_value: i64,
        equipment_id_value: i64,
    ) -> Result<()> {
        diesel::delete(player_mount_equipments::table)
            .filter(player_mount_equipments::pid.eq(pid_value))
            .filter(player_mount_equipments::uid.eq(uid_value))
            .filter(player_mount_equipments::equipment_id.eq(equipment_id_value))
            .execute(conn)?;

        Ok(())
    }

    pub fn get_equipment_used_id(conn:&PgConnection,uid:i64,equipment_id:i64)->QueryResult<i64>{
        player_mount_equipments::table
            .filter(player_mount_equipments::uid.eq(uid))
            .filter(player_mount_equipments::equipment_id.eq(equipment_id))
            .select(player_mount_equipments::id)
            .get_result(conn)
    }

    pub fn switch_user_player_equipment(
        conn: &PgConnection,
        id: i64,
        uid: i64,
        equipment_id: i64,
    ) -> Result<()> {
        let equipment_used_id: i64 = match Self::get_equipment_used_id(conn,uid,equipment_id)
        {
            Ok(v) => v,
            Err(_e) => 0,
        };

        if equipment_used_id > 0 {
            return Err(anyhow!("user equipment has mounted."));
        }

        let t = Utc::now();

        diesel::update(player_mount_equipments::table)
            .set((
                player_mount_equipments::equipment_id.eq(equipment_id),
                player_mount_equipments::modify_time.eq(t.naive_utc()),
            ))
            .filter(player_mount_equipments::uid.eq(uid))
            .filter(player_mount_equipments::equipment_id.eq(id))
            .execute(conn)?;

        Ok(())
    }
}
