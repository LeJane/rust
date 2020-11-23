use crate::models::props_mall_assets::PropsMallAsset;
use crate::schema::props_mall_assets;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl PropsMallAsset {
    pub fn get_props_mall_asset_list_b_item_id(conn: &PgConnection, item_id: i64) -> QueryResult<Vec<Self>> {
        props_mall_assets::table
            .filter(props_mall_assets::item_id.eq(item_id))
            .get_results(conn)
    }

    pub fn get_props_mall_asset_by_id(conn: &PgConnection, aid: i64) -> QueryResult<Self> {
        props_mall_assets::table
            .filter(props_mall_assets::aid.eq(aid))
            .get_result(conn)
    }

    pub fn get_props_mall_asset_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        props_mall_assets::table
            .get_results(conn)
    }
}


impl MetadataInstance for PropsMallAsset {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsMallAsset.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsMallAsset::get_props_mall_asset_by_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsMallAsset(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsMallAsset::get_props_mall_asset_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsMallAsset(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}



impl BinaryEncode for PropsMallAsset {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.aid)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i64(&mut encoded, self.sub_item_id)?;
        binary_write_i32(&mut encoded, self.amounts)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsMallAsset {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsMallAsset> {
        let aid = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let sub_item_id = binary_read_i64(cursor)?;
        let amounts = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsMallAsset {
            aid,
            item_id,
            sub_item_id,
            amounts,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
