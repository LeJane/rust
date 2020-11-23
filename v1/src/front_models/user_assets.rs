use crate::{
    utils::binary_read_helper::{
        binary_read_i32, binary_read_i64, binary_write_i32, binary_write_i64,
    },
    BinaryDecode, BinaryEncode,
};
use anyhow::Result;
use std::io::Cursor;

#[derive(Debug, Default, Queryable)]
pub struct FrontDisplayUserAsset {
    pub uuid: i64,
    pub gold_amounts: i32,
    pub gem_amounts: i32,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub stone_amounts: i32,
}

impl BinaryEncode for FrontDisplayUserAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i32(&mut encoded, self.gold_amounts)?;
        binary_write_i32(&mut encoded, self.gem_amounts)?;
        binary_write_i32(&mut encoded, self.food_amounts)?;
        binary_write_i32(&mut encoded, self.wood_amounts)?;
        binary_write_i32(&mut encoded, self.stone_amounts)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserAsset {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayUserAsset> {
        let uuid = binary_read_i64(cursor)?;
        let gold_amounts = binary_read_i32(cursor)?;
        let gem_amounts = binary_read_i32(cursor)?;
        let food_amounts = binary_read_i32(cursor)?;
        let wood_amounts = binary_read_i32(cursor)?;
        let stone_amounts = binary_read_i32(cursor)?;

        let data = FrontDisplayUserAsset {
            uuid,
            gold_amounts,
            gem_amounts,
            food_amounts,
            wood_amounts,
            stone_amounts,
        };
        Ok(data)
    }
}
