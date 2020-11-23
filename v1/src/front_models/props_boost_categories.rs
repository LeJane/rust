use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FrontDisplayPropsBoostCategory {
    pub item_id: i64,
    pub boost_time: i64,
    pub boost_value: f32,
    pub buff_id: i64,
}

impl BinaryEncode for FrontDisplayPropsBoostCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i64(&mut encoded, self.boost_time)?;
        binary_write_f32(&mut encoded, self.boost_value)?;
        binary_write_i64(&mut encoded, self.buff_id)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPropsBoostCategory {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayPropsBoostCategory> {
        let item_id = binary_read_i64(cursor)?;
        let boost_time = binary_read_i64(cursor)?;
        let boost_value = binary_read_f32(cursor)?;
        let buff_id = binary_read_i64(cursor)?;
        let data = FrontDisplayPropsBoostCategory {
            item_id,
            boost_time,
            boost_value,
            buff_id,
        };
        Ok(data)
    }
}
