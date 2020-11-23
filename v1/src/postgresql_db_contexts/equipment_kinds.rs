use crate::models::equipment_kinds::EquipmentKind;
use crate::schema::equipment_kinds;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl EquipmentKind {
    pub fn get_equipment_kind_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        equipment_kinds::table.load(conn)
    }

    pub fn get_equipment_kind_by_id(conn: &PgConnection, kid: i64) -> QueryResult<Self> {
        equipment_kinds::table
            .filter(equipment_kinds::kid.eq(kid))
            .get_result(conn)
    }
}

impl MetadataInstance for EquipmentKind {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::EquipmentKind.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = EquipmentKind::get_equipment_kind_by_id(conn, id)?;

        Ok(MetadataTypeEnum::EquipmentKind(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = EquipmentKind::get_equipment_kind_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::EquipmentKind(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for EquipmentKind {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.kid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i16(&mut encoded, self.kind)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for EquipmentKind {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<EquipmentKind> {
        let kid = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let kind = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = EquipmentKind {
            kid,
            name,
            kind,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
