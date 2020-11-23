use crate::models::props_fixed_treasure_chest_categories::PropsFixedTreasureChestCategory;
use crate::schema::props_fixed_treasure_chest_categories;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};



impl PropsFixedTreasureChestCategory {
    pub fn get_fixed_treasure_chest_data_by_id(
        conn: &PgConnection,
        item_id: i64,
    ) -> QueryResult<Self> {
        props_fixed_treasure_chest_categories::table
            .filter(props_fixed_treasure_chest_categories::item_id.eq(item_id))
            .get_result(conn)
    }

    pub fn get_fixed_treasure_chest_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        props_fixed_treasure_chest_categories::table
            .load(conn)
    }
}



impl MetadataInstance for PropsFixedTreasureChestCategory {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsFixedTreasureChestCategory.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsFixedTreasureChestCategory::get_fixed_treasure_chest_data_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsFixedTreasureChestCategory(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsFixedTreasureChestCategory::get_fixed_treasure_chest_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsFixedTreasureChestCategory(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsFixedTreasureChestCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_f32(&mut encoded, self.price)?;
        binary_write_i16(&mut encoded, self.is_instantly_open)?;
        binary_write_i16(&mut encoded, self.option_values)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsFixedTreasureChestCategory {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsFixedTreasureChestCategory> {

        let item_id = binary_read_i64(cursor)?;
        let price = binary_read_f32(cursor)?;
        let is_instantly_open = binary_read_i16(cursor)?;
        let option_values = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsFixedTreasureChestCategory {
            item_id,
            price,
            is_instantly_open,
            option_values,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

