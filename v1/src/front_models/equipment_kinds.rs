use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayEquipmentKind {
    pub kid: i64,
    pub name: String,
    pub kind: i16,
}

impl BinaryEncode for FrontDisplayEquipmentKind {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.kid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i16(&mut encoded, self.kind)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayEquipmentKind {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayEquipmentKind> {
        let kid = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let kind = binary_read_i16(cursor)?;

        let data = FrontDisplayEquipmentKind { kid, name, kind };
        Ok(data)
    }
}
