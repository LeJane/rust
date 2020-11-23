use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FrontDisplayPropsMallBuyState {
    pub item_id: i64,
    pub is_buy: i16, //0:default,1:not,2:buy
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FrontDisplayPropsMallSuperBundleBuyState {
    pub item_id: i64,
    pub purchase_limit: i16,
    pub is_buy: i16, //0:default,1:not,2:buy
}

impl BinaryEncode for FrontDisplayPropsMallBuyState {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i16(&mut encoded, self.is_buy)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPropsMallBuyState {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayPropsMallBuyState> {
        let item_id = binary_read_i64(cursor)?;
        let is_buy = binary_read_i16(cursor)?;

        let data = FrontDisplayPropsMallBuyState { item_id, is_buy };
        Ok(data)
    }
}


impl BinaryEncode for FrontDisplayPropsMallSuperBundleBuyState {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i16(&mut encoded, self.purchase_limit)?;
        binary_write_i16(&mut encoded, self.is_buy)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPropsMallSuperBundleBuyState {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayPropsMallSuperBundleBuyState> {

        let item_id = binary_read_i64(cursor)?;
        let purchase_limit = binary_read_i16(cursor)?;
        let is_buy = binary_read_i16(cursor)?;

        let data = FrontDisplayPropsMallSuperBundleBuyState {
            item_id,
            purchase_limit,
            is_buy,
        };
        Ok(data)
    }
}

