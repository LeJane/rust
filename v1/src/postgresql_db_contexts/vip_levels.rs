use crate::models::vip_levels::VipLevel;
use crate::schema::vip_levels;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl VipLevel {
    pub fn get_vip_level_by_level(conn: &PgConnection, level: i64) -> QueryResult<Self> {
        vip_levels::table
            .filter(vip_levels::level.eq(level))
            .first(conn)
    }

    pub fn get_max_vip_level(conn: &PgConnection) -> QueryResult<i64> {
        use diesel::dsl::max;

        let mut max_level: i64 = 17;

        if let Ok(Some(v)) = vip_levels::table.select(max(vip_levels::level)).first(conn) {
            max_level = v;
        }

        Ok(max_level)
    }

    pub fn get_vip_level_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        vip_levels::table.load(conn)
    }
}




impl MetadataInstance for VipLevel {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::VipLevel.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = VipLevel::get_vip_level_by_level(conn, id)?;

        Ok(MetadataTypeEnum::VipLevel(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = VipLevel::get_vip_level_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::VipLevel(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for VipLevel {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.level)?;
        binary_write_i32(&mut encoded, self.vip_points_needed)?;
        binary_write_i64(&mut encoded, self.free_treasure_chest_item_id)?;
        binary_write_i64(&mut encoded, self.pay_treasure_chest_item_id)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for VipLevel {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<VipLevel> {

        let level = binary_read_i64(cursor)?;
        let vip_points_needed = binary_read_i32(cursor)?;
        let free_treasure_chest_item_id = binary_read_i64(cursor)?;
        let pay_treasure_chest_item_id = binary_read_i64(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = VipLevel {
            level,
            vip_points_needed,
            free_treasure_chest_item_id,
            pay_treasure_chest_item_id,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
