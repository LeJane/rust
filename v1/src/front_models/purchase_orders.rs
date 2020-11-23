use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayPurchaseOrder {
    pub order_no: i64,
}

impl BinaryEncode for FrontDisplayPurchaseOrder {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.order_no)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPurchaseOrder {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayPurchaseOrder> {
        let order_no = binary_read_i64(cursor)?;

        let data = FrontDisplayPurchaseOrder { order_no };
        Ok(data)
    }
}
