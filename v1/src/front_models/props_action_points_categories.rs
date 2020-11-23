use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Queryable, Deserialize, Serialize)]
pub struct FrontDisplayPropsActionPointsCategory {
    pub item_id: i64,
    pub ap_value: i32,
    pub attribute_id: i32,
}

impl BinaryEncode for FrontDisplayPropsActionPointsCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.ap_value)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPropsActionPointsCategory {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayPropsActionPointsCategory> {
        let item_id = binary_read_i64(cursor)?;
        let ap_value = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;

        let data = FrontDisplayPropsActionPointsCategory {
            item_id,
            ap_value,
            attribute_id,
        };
        Ok(data)
    }
}
