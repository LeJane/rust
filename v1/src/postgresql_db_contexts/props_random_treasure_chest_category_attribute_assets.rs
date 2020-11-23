use crate::models::props_random_treasure_chest_category_attribute_assets::PropsRandomTreasureChestCategoryAttributeAsset;
use crate::schema::props_random_treasure_chest_category_attribute_assets;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl PropsRandomTreasureChestCategoryAttributeAsset {
    pub fn get_random_treasure_chest_attribute_asset_list_by_item_id(
        conn: &PgConnection,
        item_id: i64,
    ) -> QueryResult<Vec<Self>> {
        props_random_treasure_chest_category_attribute_assets::table
            .filter(props_random_treasure_chest_category_attribute_assets::item_id.eq(item_id))
            .load(conn)
    }

    pub fn get_random_treasure_chest_attribute_asset_by_id(
        conn: &PgConnection,
        id: i64,
    ) -> QueryResult<Self> {
        props_random_treasure_chest_category_attribute_assets::table
            .filter(props_random_treasure_chest_category_attribute_assets::id.eq(id))
            .first(conn)
    }
    pub fn get_random_treasure_chest_attribute_asset_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        props_random_treasure_chest_category_attribute_assets::table
            .load(conn)
    }
}




impl MetadataInstance for PropsRandomTreasureChestCategoryAttributeAsset {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsRandomTreasureChestCategoryAttributeAsset.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsRandomTreasureChestCategoryAttributeAsset::get_random_treasure_chest_attribute_asset_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsRandomTreasureChestCategoryAttributeAsset(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsRandomTreasureChestCategoryAttributeAsset::get_random_treasure_chest_attribute_asset_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsRandomTreasureChestCategoryAttributeAsset(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsRandomTreasureChestCategoryAttributeAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_f32(&mut encoded, self.probability_value)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsRandomTreasureChestCategoryAttributeAsset {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsRandomTreasureChestCategoryAttributeAsset> {

        let id = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let amounts = binary_read_i32(cursor)?;
        let probability_value = binary_read_f32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsRandomTreasureChestCategoryAttributeAsset {
            id,
            item_id,
            attribute_id,
            amounts,
            probability_value,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
