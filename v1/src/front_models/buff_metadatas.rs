use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayBuffMetadata {
    pub buff_id: i64,
    pub name: String,
    pub amounts: i32,
    pub attribute_id: i32,
    pub buff_category: i32,
    pub buff_type: i32,
    pub sub_buff_type: i32,
    pub buff_source: i32,
    pub is_show: i16,
}

impl BinaryEncode for FrontDisplayBuffMetadata {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.buff_id)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_i32(&mut encoded, self.buff_category)?;
        binary_write_i32(&mut encoded, self.buff_type)?;
        binary_write_i32(&mut encoded, self.sub_buff_type)?;
        binary_write_i32(&mut encoded, self.buff_source)?;
        binary_write_i16(&mut encoded, self.is_show)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayBuffMetadata {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayBuffMetadata> {
        let buff_id = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let amounts = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let buff_category = binary_read_i32(cursor)?;
        let buff_type = binary_read_i32(cursor)?;
        let sub_buff_type = binary_read_i32(cursor)?;
        let buff_source = binary_read_i32(cursor)?;
        let is_show = binary_read_i16(cursor)?;

        let data = FrontDisplayBuffMetadata {
            buff_id,
            name,
            amounts,
            attribute_id,
            buff_category,
            buff_type,
            sub_buff_type,
            buff_source,
            is_show,
        };
        Ok(data)
    }
}
