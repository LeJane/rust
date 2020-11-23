use crate::models::vip_daily_login_treasure_chests::VipDailyLoginTreasureChest;
use crate::schema::vip_daily_login_treasure_chests;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl VipDailyLoginTreasureChest {
    pub fn get_front_daily_login_treasure_chest_by_continuous_login_days(
        conn: &PgConnection,
        continuous_login_days: i32,
    ) -> QueryResult<Self> {
        use diesel::dsl::max;

        let mut max_day = 8;

        if let Ok(Some(v)) = vip_daily_login_treasure_chests::table
            .select(max(vip_daily_login_treasure_chests::continuous_login_days))
            .first(conn)
        {
            max_day = v;
        }

        if continuous_login_days > max_day {
            vip_daily_login_treasure_chests::table
                .filter(vip_daily_login_treasure_chests::continuous_login_days.eq(-1))
                .first(conn)
        } else {
            vip_daily_login_treasure_chests::table
                .filter(
                    vip_daily_login_treasure_chests::continuous_login_days
                        .eq(continuous_login_days),
                )
                .first(conn)
        }
    }

    pub fn get_daily_login_treasure_chest_by_id(conn:&PgConnection,id:i64)->QueryResult<Self>{
        vip_daily_login_treasure_chests::table
            .filter(vip_daily_login_treasure_chests::id.eq(id))
            .first(conn)
    }

    pub fn get_daily_login_treasure_chest_list(conn:&PgConnection)->QueryResult<Vec<Self>>{
        vip_daily_login_treasure_chests::table
            .load(conn)
    }
}



impl MetadataInstance for VipDailyLoginTreasureChest {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::VipDailyLoginTreasureChest.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = VipDailyLoginTreasureChest::get_daily_login_treasure_chest_by_id(conn, id)?;

        Ok(MetadataTypeEnum::VipDailyLoginTreasureChest(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = VipDailyLoginTreasureChest::get_daily_login_treasure_chest_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::VipDailyLoginTreasureChest(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for VipDailyLoginTreasureChest {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i32(&mut encoded, self.continuous_login_days)?;
        binary_write_i32(&mut encoded, self.today_vip_points)?;
        binary_write_i32(&mut encoded, self.tomorrow_vip_points)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for VipDailyLoginTreasureChest {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<VipDailyLoginTreasureChest> {

        let id = binary_read_i64(cursor)?;
        let continuous_login_days = binary_read_i32(cursor)?;
        let today_vip_points = binary_read_i32(cursor)?;
        let tomorrow_vip_points = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = VipDailyLoginTreasureChest {
            id,
            continuous_login_days,
            today_vip_points,
            tomorrow_vip_points,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
