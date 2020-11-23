use crate::models::buff_metadatas::BuffMetadata;
use crate::schema::buff_metadatas;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use std::io::Cursor;

impl BuffMetadata {
    pub fn get_buff_metadata_by_id(conn: &PgConnection, buff_id: i64) -> Result<Self> {
        buff_metadatas::table
            .filter(buff_metadatas::buff_id.eq(buff_id))
            .get_result(conn)
            .map_err(|e| anyhow!("failed get buff metadata {} : {}", buff_id, e))
    }

    pub fn get_buff_metadata_list(conn: &PgConnection) -> Result<Vec<Self>> {
        buff_metadatas::table
            .load(conn)
            .map_err(|e| anyhow!("failed get buff metadata list: {}", e))
    }
}

impl MetadataInstance for BuffMetadata {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::BuffMetadata.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = BuffMetadata::get_buff_metadata_by_id(conn, id)?;

        Ok(MetadataTypeEnum::BuffMetadata(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = BuffMetadata::get_buff_metadata_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::BuffMetadata(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for BuffMetadata {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.buff_id)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_i32(&mut encoded, self.buff_category)?;
        binary_write_i32(&mut encoded, self.buff_type)?;
        binary_write_i32(&mut encoded, self.sub_buff_type)?;
        binary_write_i32(&mut encoded, self.buff_source)?;
        binary_write_i16(&mut encoded, self.is_show)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for BuffMetadata {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<BuffMetadata> {
        let buff_id = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let amounts = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let buff_category = binary_read_i32(cursor)?;
        let buff_type = binary_read_i32(cursor)?;
        let sub_buff_type = binary_read_i32(cursor)?;
        let buff_source = binary_read_i32(cursor)?;
        let is_show = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = BuffMetadata {
            buff_id,
            name,
            amounts,
            attribute_id,
            buff_category,
            buff_type,
            sub_buff_type,
            buff_source,
            is_show,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
