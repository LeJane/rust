use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayUserPlayer {
    pub id: i64,
    pub pid: i64,
    pub max_hp: i32,
    pub attack_power: i32,
    pub move_speed: f32,
    pub max_mana: i32,
    pub defense: i32,
    pub level: i16,
    pub star_level: i16,
    pub level_experience: i32,
}

impl BinaryEncode for FrontDisplayUserPlayer {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.pid)?;
        binary_write_i32(&mut encoded, self.max_hp)?;
        binary_write_i32(&mut encoded, self.attack_power)?;
        binary_write_f32(&mut encoded, self.move_speed)?;
        binary_write_i32(&mut encoded, self.max_mana)?;
        binary_write_i32(&mut encoded, self.defense)?;
        binary_write_i16(&mut encoded, self.level)?;
        binary_write_i16(&mut encoded, self.star_level)?;
        binary_write_i32(&mut encoded, self.level_experience)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserPlayer {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayUserPlayer> {
        let id = binary_read_i64(cursor)?;
        let pid = binary_read_i64(cursor)?;
        let max_hp = binary_read_i32(cursor)?;
        let attack_power = binary_read_i32(cursor)?;
        let move_speed = binary_read_f32(cursor)?;
        let max_mana = binary_read_i32(cursor)?;
        let defense = binary_read_i32(cursor)?;
        let level = binary_read_i16(cursor)?;
        let star_level = binary_read_i16(cursor)?;
        let level_experience = binary_read_i32(cursor)?;

        let data = FrontDisplayUserPlayer {
            id,
            pid,
            max_hp,
            attack_power,
            move_speed,
            max_mana,
            defense,
            level,
            star_level,
            level_experience,
        };
        Ok(data)
    }
}
