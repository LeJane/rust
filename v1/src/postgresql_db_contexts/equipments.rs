use crate::models::equipments::Equipment;

use crate::schema::equipments;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl Equipment {
    pub fn get_equipment_list(conn: &PgConnection) -> QueryResult<Vec<Equipment>> {
        equipments::table.get_results(conn)
    }

    pub fn get_equipment_list_by_kid_page(
        conn: &PgConnection,
        kid: i64,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<Self>> {
        equipments::table
            .filter(equipments::kid.eq(kid))
            .limit(limit)
            .offset(offset)
            .get_results(conn)
    }

    pub fn get_equipment_data_by_id(conn: &PgConnection, eid: i64) -> QueryResult<Equipment> {
        equipments::table.find(eid).first(conn)
    }

    pub fn get_default_equipment_id_list(conn: &PgConnection) -> QueryResult<Vec<i64>> {
        equipments::table
            .filter(equipments::is_default.eq(2))
            .select(equipments::eid)
            .get_results(conn)
    }
}

impl MetadataInstance for Equipment {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Equipment.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Equipment::get_equipment_data_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Equipment(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Equipment::get_equipment_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Equipment(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for Equipment {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.eid)?;
        binary_write_i64(&mut encoded, self.kid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_i32(&mut encoded, self.price)?;
        binary_write_i32(&mut encoded, self.hp)?;
        binary_write_f32(&mut encoded, self.multiplier)?;
        binary_write_i16(&mut encoded, self.kind)?;
        binary_write_i16(&mut encoded, self.is_default)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Equipment {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Equipment> {
        let eid = binary_read_i64(cursor)?;
        let kid = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let price = binary_read_i32(cursor)?;
        let hp = binary_read_i32(cursor)?;
        let multiplier = binary_read_f32(cursor)?;
        let kind = binary_read_i16(cursor)?;
        let is_default = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Equipment {
            eid,
            kid,
            name,
            thumbnail,
            price,
            hp,
            multiplier,
            kind,
            is_default,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
