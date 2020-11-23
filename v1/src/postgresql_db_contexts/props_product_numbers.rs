use crate::models::props_product_numbers::PropsProductNumber;
use crate::schema::props_product_numbers;
use diesel::prelude::*;

use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl PropsProductNumber {
    pub fn get_props_product_number(
        conn: &PgConnection,
        item_id: i64,
        platform_id: i16,
    ) -> QueryResult<Self> {
        props_product_numbers::table
            .filter(props_product_numbers::item_id.eq(item_id))
            .filter(props_product_numbers::platform_id.eq(platform_id))
            .get_result(conn)
    }

    pub fn get_props_product_number_by_id(
        conn: &PgConnection,
        item_id: i64,
    ) -> QueryResult<Self> {
        props_product_numbers::table
            .filter(props_product_numbers::item_id.eq(item_id))
            .get_result(conn)
    }

    pub fn get_props_product_number_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        props_product_numbers::table
            .load(conn)
    }
}


impl MetadataInstance for PropsProductNumber {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsProductNumber.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsProductNumber::get_props_product_number_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsProductNumber(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsProductNumber::get_props_product_number_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsProductNumber(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsProductNumber {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_string(&mut encoded, self.product_number.as_str())?;
        binary_write_i16(&mut encoded, self.platform_id)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsProductNumber {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsProductNumber> {

        let item_id = binary_read_i64(cursor)?;
        let product_number = binary_read_string(cursor, bytes)?;
        let platform_id = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsProductNumber {
            item_id,
            product_number,
            platform_id,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

