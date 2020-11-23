use crate::models::quests_attribute_assets::QuestsAttributeAsset;
use crate::schema::quests_attribute_assets;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl QuestsAttributeAsset {
    pub fn get_quest_attribute_asset_list_by_quests_id(
        conn: &PgConnection,
        quests_id: i64,
    ) -> QueryResult<Vec<Self>> {
        quests_attribute_assets::table
            .filter(quests_attribute_assets::quests_id.eq(quests_id))
            .load(conn)
    }

    pub fn get_quest_attribute_asset_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        quests_attribute_assets::table
            .load(conn)
    }
    pub fn get_quest_attribute_asset_list_by_id(conn: &PgConnection, id: i64) -> QueryResult<Self> {
        quests_attribute_assets::table
            .filter(quests_attribute_assets::id.eq(id))
            .first(conn)
    }
}




impl MetadataInstance for QuestsAttributeAsset {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::QuestsAttributeAsset.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = QuestsAttributeAsset::get_quest_attribute_asset_list_by_id(conn, id)?;

        Ok(MetadataTypeEnum::QuestsAttributeAsset(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = QuestsAttributeAsset::get_quest_attribute_asset_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::QuestsAttributeAsset(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for QuestsAttributeAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.quests_id)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for QuestsAttributeAsset {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<QuestsAttributeAsset> {

        let id = binary_read_i64(cursor)?;
        let quests_id = binary_read_i64(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let amounts = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = QuestsAttributeAsset {
            id,
            quests_id,
            attribute_id,
            amounts,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
