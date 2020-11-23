use crate::models::props_speed_up_categories::PropsSpeedUpCategory;
use crate::schema::props_speed_up_categories;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl PropsSpeedUpCategory {
    pub fn get_speed_up_props_by_id(conn: &PgConnection, item_id: i64) -> QueryResult<Self> {
        props_speed_up_categories::table
            .filter(props_speed_up_categories::item_id.eq(item_id))
            .first(conn)
    }

    pub fn get_speed_up_props_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        props_speed_up_categories::table
            .load(conn)
    }
}




impl MetadataInstance for PropsSpeedUpCategory {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsSpeedUpCategory.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsSpeedUpCategory::get_speed_up_props_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsSpeedUpCategory(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsSpeedUpCategory::get_speed_up_props_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsSpeedUpCategory(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsSpeedUpCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.speed_time)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsSpeedUpCategory {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsSpeedUpCategory> {

        let item_id = binary_read_i64(cursor)?;
        let speed_time = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsSpeedUpCategory {
            item_id,
            speed_time,
            attribute_id,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}


