use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayUserEquipment {
    pub id: i64,
    pub eid: i64,
}

impl BinaryEncode for FrontDisplayUserEquipment {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.eid)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserEquipment {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayUserEquipment> {
        let id = binary_read_i64(cursor)?;
        let eid = binary_read_i64(cursor)?;

        let data = FrontDisplayUserEquipment { id, eid };
        Ok(data)
    }
}
