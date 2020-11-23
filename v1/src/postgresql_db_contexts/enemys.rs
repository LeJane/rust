use crate::models::enemys::Enemy;
use crate::schema::enemys;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl Enemy {
    pub fn get_enemy_data_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        enemys::table.get_results(conn)
    }

    pub fn get_enemy_data_by_id(conn: &PgConnection, eid: i64) -> QueryResult<Self> {
        enemys::table.filter(enemys::eid.eq(eid)).get_result(conn)
    }
}

impl MetadataInstance for Enemy {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Enemy.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Enemy::get_enemy_data_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Enemy(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Enemy::get_enemy_data_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Enemy(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for Enemy {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.eid)?;
        binary_write_string(&mut encoded, self.enemy_name.as_str())?;
        binary_write_string(&mut encoded, self.model_path.as_str())?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_i32(&mut encoded, self.max_hp)?;
        binary_write_i32(&mut encoded, self.attack_power)?;
        binary_write_f32(&mut encoded, self.move_speed)?;
        binary_write_i32(&mut encoded, self.max_mana)?;
        binary_write_i32(&mut encoded, self.defense)?;
        binary_write_f32(&mut encoded, self.animation_hit_delay)?;
        binary_write_string(&mut encoded, self.spawn_style_class.as_str())?;
        binary_write_string(&mut encoded, self.bp_enemy.as_str())?;
        binary_write_string(&mut encoded, self.ap_enemy.as_str())?;
        binary_write_string(&mut encoded, self.skm_enemy.as_str())?;
        binary_write_string(&mut encoded, self.enemy_die.as_str())?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Enemy {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Enemy> {
        let eid = binary_read_i64(cursor)?;
        let enemy_name = binary_read_string(cursor, bytes)?;
        let model_path = binary_read_string(cursor, bytes)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let max_hp = binary_read_i32(cursor)?;
        let attack_power = binary_read_i32(cursor)?;
        let move_speed = binary_read_f32(cursor)?;
        let max_mana = binary_read_i32(cursor)?;
        let defense = binary_read_i32(cursor)?;
        let animation_hit_delay = binary_read_f32(cursor)?;
        let spawn_style_class = binary_read_string(cursor, bytes)?;
        let bp_enemy = binary_read_string(cursor, bytes)?;
        let ap_enemy = binary_read_string(cursor, bytes)?;
        let skm_enemy = binary_read_string(cursor, bytes)?;
        let enemy_die = binary_read_string(cursor, bytes)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Enemy {
            eid,
            enemy_name,
            model_path,
            thumbnail,
            max_hp,
            attack_power,
            move_speed,
            max_mana,
            defense,
            animation_hit_delay,
            spawn_style_class,
            bp_enemy,
            ap_enemy,
            skm_enemy,
            enemy_die,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
