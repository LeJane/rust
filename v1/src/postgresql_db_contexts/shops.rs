use crate::models::shops::Shop;
use crate::schema::shops;
use crate::{
    utils::binary_read_helper::*, BinaryDecode, BinaryEncode, FrontDisplayMetaVersion,
    FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum, TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl Shop {
    pub fn get_shop_by_id(conn: &PgConnection, sid: i64) -> QueryResult<Self> {
        shops::table.filter(shops::sid.eq(sid)).get_result(conn)
    }

    pub fn get_shop_vip_points_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        shops::table.filter(shops::sub_item_type.eq(303)).load(conn)
    }

    pub fn get_shop_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        shops::table.load(conn)
    }
}

impl BinaryEncode for Shop {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.sid)?;
        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.bag_type)?;
        binary_write_i32(&mut encoded, self.sub_item_type)?;
        binary_write_i64(&mut encoded, self.order_value)?;
        binary_write_i32(&mut encoded, self.gems_needed)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Shop {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Shop> {
        let sid = binary_read_i64(cursor)?;
        let item_id = binary_read_i64(cursor)?;
        let bag_type = binary_read_i32(cursor)?;
        let sub_item_type = binary_read_i32(cursor)?;
        let order_value = binary_read_i64(cursor)?;
        let gems_needed = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Shop {
            sid,
            item_id,
            bag_type,
            sub_item_type,
            order_value,
            gems_needed,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

impl MetadataInstance for Shop {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Shop.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let shop = Shop::get_shop_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Shop(shop))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let shop_list = Shop::get_shop_list(conn)?;

        FrontDisplayMetaVersionRelation::get_front_display_meta_version_from_value(
            shop_list,
            |data| MetadataTypeEnum::Shop(data),
        )
    }
}
