use crate::models::quests_assets::QuestsAsset;
use crate::schema::quests_assets;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl QuestsAsset {
    pub fn get_quest_asset_list_by_quests_id(conn: &PgConnection, quests_id: i64) -> QueryResult<Vec<Self>> {
        quests_assets::table
            .filter(quests_assets::quests_id.eq(quests_id))
            .load(conn)
    }

    pub fn get_quest_asset_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        quests_assets::table
            .load(conn)
    }
    pub fn get_quest_asset_list_by_id(conn: &PgConnection, id: i64) -> QueryResult<Self> {
        quests_assets::table
            .filter(quests_assets::id.eq(id))
            .first(conn)
    }
}




impl MetadataInstance for QuestsAsset {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::QuestsAsset.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = QuestsAsset::get_quest_asset_list_by_id(conn, id)?;

        Ok(MetadataTypeEnum::QuestsAsset(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = QuestsAsset::get_quest_asset_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::QuestsAsset(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for QuestsAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.quests_id)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for QuestsAsset {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<QuestsAsset> {

        let id = binary_read_i64(cursor)?;
        let quests_id = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let amounts = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = QuestsAsset {
            id,
            quests_id,
            item_id,
            amounts,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
