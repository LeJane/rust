use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayVipDailyLoginTreasureChest {
    pub continuous_login_days: i32,
    pub today_vip_points: i32,
    pub tomorrow_vip_points: i32,
    pub level: i64,
}

impl BinaryEncode for FrontDisplayVipDailyLoginTreasureChest {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, self.continuous_login_days)?;
        binary_write_i32(&mut encoded, self.today_vip_points)?;
        binary_write_i32(&mut encoded, self.tomorrow_vip_points)?;
        binary_write_i64(&mut encoded, self.level)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayVipDailyLoginTreasureChest {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayVipDailyLoginTreasureChest> {
        let continuous_login_days = binary_read_i32(cursor)?;
        let today_vip_points = binary_read_i32(cursor)?;
        let tomorrow_vip_points = binary_read_i32(cursor)?;
        let level = binary_read_i64(cursor)?;

        let data = FrontDisplayVipDailyLoginTreasureChest {
            continuous_login_days,
            today_vip_points,
            tomorrow_vip_points,
            level,
        };
        Ok(data)
    }
}
