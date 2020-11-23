use crate::front_models::versions::{FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation};

use crate::models::props_malls::PropsMall;
use crate::schema::props_malls;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{MetadataInstance, MetadataTypeEnum, TableIdEnum};
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use std::io::Cursor;

impl PropsMall {
    pub fn get_props_mall_list(conn: &PgConnection) -> Result<Vec<Self>> {
        props_malls::table
            .get_results(conn)
            .map_err(|e| anyhow!("failed get props data:{}", e))
    }

    pub fn get_single_props_mall_data(conn: &PgConnection, item_id: i64) -> Result<Self> {
        let t = props_malls::table.filter(props_malls::item_id.eq(item_id));

        t.get_result(conn)
            .map_err(|e| anyhow!("failed get props data:{}", e))
    }

    pub fn get_props_mall_by_item_id(conn: &PgConnection, item_id: i64) -> Result<Self> {
        props_malls::table
            .filter(props_malls::item_id.eq(item_id))
            .get_result(conn)
            .map_err(|e| anyhow!("failed get props data:{}", e))
    }

    //first recharge gift package
    pub fn get_props_mall_by_mall_type(conn: &PgConnection, mall_type: i32) -> Result<Self> {
        props_malls::table
            .filter(props_malls::mall_type.eq(mall_type))
            .get_result(conn)
            .map_err(|e| anyhow!("failed get props data:{}", e))
    }

    pub fn get_props_mall_list_by_mall_type(
        conn: &PgConnection,
        mall_type: i32,
    ) -> Result<Vec<Self>> {
        props_malls::table
            .filter(props_malls::mall_type.eq(mall_type))
            .get_results(conn)
            .map_err(|e| anyhow!("failed get props data:{}", e))
    }
}

impl MetadataInstance for PropsMall {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsMall.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let props_mall = PropsMall::get_single_props_mall_data(conn, id)?;

        Ok(MetadataTypeEnum::PropsMall(props_mall))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsMall::get_props_mall_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsMall(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsMall {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i64(&mut encoded, self.next_item_id)?;
        binary_write_i64(&mut encoded, self.level)?;
        binary_write_i32(&mut encoded, self.item_category)?;
        binary_write_f32(&mut encoded, self.price)?;
        binary_write_i16(&mut encoded, self.purchase_limit)?;
        binary_write_string(&mut encoded, self.small_icon.as_str())?;
        binary_write_i32(&mut encoded, self.gem_amounts)?;
        binary_write_i32(&mut encoded, self.food_amounts)?;
        binary_write_i32(&mut encoded, self.wood_amounts)?;
        binary_write_i32(&mut encoded, self.first_buy_handsel)?;
        binary_write_i32(&mut encoded, self.late_buy_handsel)?;
        binary_write_i16(&mut encoded, self.valid_period_day)?;
        binary_write_i32(&mut encoded, self.mall_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for PropsMall {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsMall> {
        let item_id = binary_read_i64(cursor)?;
        let next_item_id = binary_read_i64(cursor)?;
        let level = binary_read_i64(cursor)?;
        let item_category = binary_read_i32(cursor)?;
        let price = binary_read_f32(cursor)?;
        let purchase_limit = binary_read_i16(cursor)?;
        let small_icon = binary_read_string(cursor, bytes)?;
        let gem_amounts = binary_read_i32(cursor)?;
        let food_amounts = binary_read_i32(cursor)?;
        let wood_amounts = binary_read_i32(cursor)?;
        let first_buy_handsel = binary_read_i32(cursor)?;
        let late_buy_handsel = binary_read_i32(cursor)?;
        let valid_period_day = binary_read_i16(cursor)?;
        let mall_type = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsMall {
            item_id,
            next_item_id,
            level,
            item_category,
            price,
            purchase_limit,
            small_icon,
            gem_amounts,
            food_amounts,
            wood_amounts,
            first_buy_handsel,
            late_buy_handsel,
            valid_period_day,
            mall_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
