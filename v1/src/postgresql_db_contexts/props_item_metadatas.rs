use crate::models::props_item_metadatas::PropsItemMetadata;
use crate::schema::props_item_metadatas;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl PropsItemMetadata {
    pub fn get_item_metadata_by_id(conn: &PgConnection, item_id: i64) -> QueryResult<Self> {
        props_item_metadatas::table
            .filter(props_item_metadatas::item_id.eq(item_id))
            .get_result(conn)
    }

    pub fn get_item_metadata_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        props_item_metadatas::table
            .get_results(conn)
    }
}



impl MetadataInstance for PropsItemMetadata {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsItemMetadata.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsItemMetadata::get_item_metadata_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsItemMetadata(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsItemMetadata::get_item_metadata_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsItemMetadata(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsItemMetadata {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_string(&mut encoded, self.description.as_str())?;
        binary_write_i16(&mut encoded, self.overlay_status)?;
        binary_write_i32(&mut encoded, self.sub_item_type)?;
        binary_write_i32(&mut encoded, self.bag_type)?;
        binary_write_i32(&mut encoded, self.rarity_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsItemMetadata {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsItemMetadata> {

        let item_id = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let description = binary_read_string(cursor, bytes)?;
        let overlay_status = binary_read_i16(cursor)?;
        let sub_item_type = binary_read_i32(cursor)?;
        let bag_type = binary_read_i32(cursor)?;
        let rarity_type = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsItemMetadata {
            item_id,
            name,
            thumbnail,
            description,
            overlay_status,
            sub_item_type,
            bag_type,
            rarity_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
