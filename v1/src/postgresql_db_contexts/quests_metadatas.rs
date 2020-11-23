use crate::models::quests_metadatas::QuestsMetadata;
use crate::schema::quests_metadatas;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl QuestsMetadata {
    pub fn get_quests_metadata(conn: &PgConnection, quests_id: i64) -> QueryResult<Self> {
        quests_metadatas::table
            .filter(quests_metadatas::quests_id.eq(quests_id))
            .first(conn)
    }
    pub fn get_quests_metadata_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        quests_metadatas::table
            .load(conn)
    }
}




impl MetadataInstance for QuestsMetadata {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::QuestsMetadata.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = QuestsMetadata::get_quests_metadata(conn, id)?;

        Ok(MetadataTypeEnum::QuestsMetadata(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = QuestsMetadata::get_quests_metadata_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::QuestsMetadata(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for QuestsMetadata {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.quests_id)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_string(&mut encoded, self.description.as_str())?;
        binary_write_i32(&mut encoded, self.quests_value)?;
        binary_write_i32(&mut encoded, self.quests_type)?;
        binary_write_i32(&mut encoded, self.sub_quests_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for QuestsMetadata {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<QuestsMetadata> {

        let quests_id = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let description = binary_read_string(cursor, bytes)?;
        let quests_value = binary_read_i32(cursor)?;
        let quests_type = binary_read_i32(cursor)?;
        let sub_quests_type = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = QuestsMetadata {
            quests_id,
            name,
            thumbnail,
            description,
            quests_value,
            quests_type,
            sub_quests_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

