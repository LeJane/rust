use crate::models::props_action_points_categories::PropsActionPointsCategory;
use crate::schema::props_action_points_categories;
use diesel::prelude::*;
use crate::{ utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};



impl PropsActionPointsCategory {
    pub fn get_action_points_props_by_item_id(conn: &PgConnection, item_id: i64) -> QueryResult<Self> {
        props_action_points_categories::table
            .filter(props_action_points_categories::item_id.eq(item_id))
            .first(conn)
    }

    pub fn get_action_points_props_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        props_action_points_categories::table
            .load(conn)
    }
}



impl MetadataInstance for PropsActionPointsCategory {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsActionPointsCategory.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsActionPointsCategory::get_action_points_props_by_item_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsActionPointsCategory(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsActionPointsCategory::get_action_points_props_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsActionPointsCategory(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsActionPointsCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.ap_value)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsActionPointsCategory {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsActionPointsCategory> {

        let item_id = binary_read_i64(cursor)?;
        let ap_value = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsActionPointsCategory {
            item_id,
            ap_value,
            attribute_id,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
