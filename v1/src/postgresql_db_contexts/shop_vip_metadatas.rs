use crate::models::shop_vip_metadatas::ShopVipMetadata;
use crate::schema::shop_vip_metadatas;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl ShopVipMetadata {
    pub fn get_shop_vip_item_by_id(conn: &PgConnection, shop_vip_id: i64) -> QueryResult<Self> {
        shop_vip_metadatas::table
            .filter(shop_vip_metadatas::shop_vip_id.eq(shop_vip_id))
            .get_result(conn)
    }

    pub fn get_shop_vip_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        shop_vip_metadatas::table.load(conn)
    }
}


impl MetadataInstance for ShopVipMetadata {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::ShopVipMetadata.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = ShopVipMetadata::get_shop_vip_item_by_id(conn, id)?;

        Ok(MetadataTypeEnum::ShopVipMetadata(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = ShopVipMetadata::get_shop_vip_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::ShopVipMetadata(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}


impl BinaryEncode for ShopVipMetadata {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.shop_vip_id)?;
        binary_write_i64(&mut encoded, self.level)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.bag_type)?;
        binary_write_i32(&mut encoded, self.sub_item_type)?;
        binary_write_i32(&mut encoded, self.cost_value)?;
        binary_write_i32(&mut encoded, self.attribute_id)?;
        binary_write_f32(&mut encoded, self.discount)?;
        binary_write_i32(&mut encoded, self.limit_amounts)?;
        binary_write_i64(&mut encoded, self.order_value)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for ShopVipMetadata {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<ShopVipMetadata> {
        let shop_vip_id = binary_read_i64(cursor)?;
        let level = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let bag_type = binary_read_i32(cursor)?;
        let sub_item_type = binary_read_i32(cursor)?;
        let cost_value = binary_read_i32(cursor)?;
        let attribute_id = binary_read_i32(cursor)?;
        let discount = binary_read_f32(cursor)?;
        let limit_amounts = binary_read_i32(cursor)?;
        let order_value = binary_read_i64(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = ShopVipMetadata {
            shop_vip_id,
            level,
            item_id,
            bag_type,
            sub_item_type,
            cost_value,
            attribute_id,
            discount,
            limit_amounts,
            order_value,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
