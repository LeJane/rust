use crate::models::players::Player;
use crate::schema::players;
use crate::{ utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;
use diesel::prelude::*;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl Player {
    pub fn get_is_default_player(conn: &PgConnection, is_default: i16) -> QueryResult<Self> {
        players::table
            .filter(players::is_default.eq(is_default))
            .get_result(conn)
    }

    pub fn get_player_by_id(conn: &PgConnection,pid:i64) -> QueryResult<Self> {
        players::table
            .filter(players::pid.eq(pid))
            .get_result(conn)
    }
    pub fn get_player_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        players::table
            .get_results(conn)
    }
}

impl MetadataInstance for Player {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Player.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Player::get_player_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Player(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Player::get_player_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Player(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for Player {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.pid)?;
        binary_write_string(&mut encoded, self.player_name.as_str())?;
        binary_write_string(&mut encoded, self.model_path.as_str())?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_i32(&mut encoded, self.max_hp)?;
        binary_write_i32(&mut encoded, self.attack_power)?;
        binary_write_f32(&mut encoded, self.move_speed)?;
        binary_write_i32(&mut encoded, self.max_mana)?;
        binary_write_i32(&mut encoded, self.defense)?;
        binary_write_f32(&mut encoded, self.animation_hit_delay)?;
        binary_write_string(&mut encoded, self.spawn_style_class.as_str())?;
        binary_write_i16(&mut encoded, self.level)?;
        binary_write_i16(&mut encoded, self.star_level)?;
        binary_write_i32(&mut encoded, self.level_experience)?;
        binary_write_i16(&mut encoded, self.is_default)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Player {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Player> {

        let pid = binary_read_i64(cursor)?;
        let player_name = binary_read_string(cursor, bytes)?;
        let model_path = binary_read_string(cursor, bytes)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let max_hp = binary_read_i32(cursor)?;
        let attack_power = binary_read_i32(cursor)?;
        let move_speed = binary_read_f32(cursor)?;
        let max_mana = binary_read_i32(cursor)?;
        let defense = binary_read_i32(cursor)?;
        let animation_hit_delay = binary_read_f32(cursor)?;
        let spawn_style_class = binary_read_string(cursor, bytes)?;
        let level = binary_read_i16(cursor)?;
        let star_level = binary_read_i16(cursor)?;
        let level_experience = binary_read_i32(cursor)?;
        let is_default = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Player {
            pid,
            player_name,
            model_path,
            thumbnail,
            max_hp,
            attack_power,
            move_speed,
            max_mana,
            defense,
            animation_hit_delay,
            spawn_style_class,
            level,
            star_level,
            level_experience,
            is_default,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
