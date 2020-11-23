use crate::models::categories::Category;
use crate::schema::categories;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use std::io::Cursor;

impl Category {
    pub fn get_category_by_id(conn: &PgConnection, id: i64) -> Result<Self> {
        categories::table
            .filter(categories::id.eq(id))
            .get_result(conn)
            .map_err(|e| anyhow!("failed get category metadata {} : {}", id, e))
    }

    pub fn get_category_metadata_list(conn: &PgConnection) -> Result<Vec<Self>> {
        categories::table
            .load(conn)
            .map_err(|e| anyhow!("failed get category metadata list: {}", e))
    }
}

impl MetadataInstance for Category {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Category.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Category::get_category_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Category(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Category::get_category_metadata_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Category(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for Category {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i32(&mut encoded, self.type_id)?;
        binary_write_string(&mut encoded, self.type_name.as_str())?;
        binary_write_string(&mut encoded, self.system_name.as_str())?;
        binary_write_i32(&mut encoded, self.system_id)?;
        binary_write_string(&mut encoded, self.table_name.as_str())?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Category {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Category> {

        let id = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let type_id = binary_read_i32(cursor)?;
        let type_name = binary_read_string(cursor, bytes)?;
        let system_name = binary_read_string(cursor, bytes)?;
        let system_id = binary_read_i32(cursor)?;
        let table_name = binary_read_string(cursor, bytes)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Category {
            id,
            name,
            type_id,
            type_name,
            system_name,
            system_id,
            table_name,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
