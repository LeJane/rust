use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone)]
pub struct FrontDisplayUserItemBag {
    pub bid: i64,
    pub item_id: i64,
    pub count: i32,
    pub order_value: i64, //asc
}

impl BinaryEncode for FrontDisplayUserItemBag {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        binary_write_i64(&mut encoded, self.bid)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.count)?;
        binary_write_i64(&mut encoded, self.order_value)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserItemBag {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayUserItemBag> {
        let bid = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let count = binary_read_i32(cursor)?;
        let order_value = binary_read_i64(cursor)?;

        let data = FrontDisplayUserItemBag {
            bid,
            item_id,
            count,
            order_value,
        };

        Ok(data)
    }
}
