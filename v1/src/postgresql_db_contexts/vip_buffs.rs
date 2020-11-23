use crate::models::vip_buffs::VipBuff;
use crate::schema::vip_buffs;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl VipBuff {
    pub fn get_vip_buffs(conn: &PgConnection, level: i64) -> QueryResult<Vec<Self>> {
        let vip_buff_list: Vec<VipBuff> = vip_buffs::table
            .filter(vip_buffs::level.eq(level))
            .load(conn)?;

        Ok(vip_buff_list)
    }

    pub fn get_vip_buff_by_id(conn: &PgConnection, id: i64) -> QueryResult<Self> {
        vip_buffs::table
            .filter(vip_buffs::id.eq(id))
            .first(conn)
    }

    pub fn get_vip_buff_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        vip_buffs::table
            .load(conn)
    }
}


impl MetadataInstance for VipBuff {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::VipBuff.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = VipBuff::get_vip_buff_by_id(conn, id)?;

        Ok(MetadataTypeEnum::VipBuff(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = VipBuff::get_vip_buff_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::VipBuff(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for VipBuff {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.level)?;
        binary_write_i64(&mut encoded, self.buff_id)?;
        binary_write_i16(&mut encoded, self.is_new_mark)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for VipBuff {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<VipBuff> {

        let id = binary_read_i64(cursor)?;
        let level = binary_read_i64(cursor)?;
        let buff_id = binary_read_i64(cursor)?;
        let is_new_mark = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = VipBuff {
            id,
            level,
            buff_id,
            is_new_mark,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

