use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayServerTime {
    pub time: i64,
}

impl BinaryEncode for FrontDisplayServerTime {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayServerTime {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayServerTime> {
        let time = binary_read_i64(cursor)?;

        let data = FrontDisplayServerTime { time };
        Ok(data)
    }
}
