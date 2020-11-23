use crate::front_models::user_item_bags::FrontDisplayUserItemBag;
use crate::{deserialize_binary, utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayUserVip {
    pub id: i64,
    pub login_days: i32,
    pub level: i64,
    pub vip_points: i32,
    pub daily_treasure_chests_status: i16, //1:未领取,2:已领取
    pub free_everyday_treasure_chests_status: i16,
    pub special_privileges_treasure_chests_status: i16,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayBuyVipPointsData {
    pub item_bags: Vec<FrontDisplayUserItemBag>,
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayBuyVipPointsSuccess {
    pub vip_points: i32,
    pub level: i64,
}

impl BinaryEncode for FrontDisplayUserVip {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i32(&mut encoded, self.login_days)?;
        binary_write_i64(&mut encoded, self.level)?;
        binary_write_i32(&mut encoded, self.vip_points)?;
        binary_write_i16(&mut encoded, self.daily_treasure_chests_status)?;
        binary_write_i16(&mut encoded, self.free_everyday_treasure_chests_status)?;
        binary_write_i16(&mut encoded, self.special_privileges_treasure_chests_status)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserVip {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<FrontDisplayUserVip> {
        let id = binary_read_i64(cursor)?;
        let login_days = binary_read_i32(cursor)?;
        let level = binary_read_i64(cursor)?;
        let vip_points = binary_read_i32(cursor)?;
        let daily_treasure_chests_status = binary_read_i16(cursor)?;
        let free_everyday_treasure_chests_status = binary_read_i16(cursor)?;
        let special_privileges_treasure_chests_status = binary_read_i16(cursor)?;

        let data = FrontDisplayUserVip {
            id,
            login_days,
            level,
            vip_points,
            daily_treasure_chests_status,
            free_everyday_treasure_chests_status,
            special_privileges_treasure_chests_status,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayBuyVipPointsData {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        let item_bags = self.item_bags.encode()?;

        encoded.extend(item_bags);

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayBuyVipPointsData {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
    ) -> Result<FrontDisplayBuyVipPointsData> {
        let item_bags: Vec<FrontDisplayUserItemBag> = deserialize_binary(cursor, bytes)?;

        let data = FrontDisplayBuyVipPointsData { item_bags };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayBuyVipPointsSuccess {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, self.vip_points)?;
        binary_write_i64(&mut encoded, self.level)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayBuyVipPointsSuccess {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayBuyVipPointsSuccess> {
        let vip_points = binary_read_i32(cursor)?;
        let level = binary_read_i64(cursor)?;

        let data = FrontDisplayBuyVipPointsSuccess { vip_points, level };
        Ok(data)
    }
}
