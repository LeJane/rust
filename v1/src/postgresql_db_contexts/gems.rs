use crate::models::gems::Gem;
use crate::schema::gems;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl Gem {
    pub fn get_gem_by_gid(conn: &PgConnection, gid: i64) -> QueryResult<Self> {
        gems::table.find(gid).first(conn)
    }

    pub fn get_gem_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        gems::table.load(conn)
    }
}

impl MetadataInstance for Gem {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Gem.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Gem::get_gem_by_gid(conn, id)?;

        Ok(MetadataTypeEnum::Gem(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Gem::get_gem_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Gem(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}


impl BinaryEncode for Gem {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.gid)?;
        binary_write_string(&mut encoded, self.gem_icon.as_str())?;
        binary_write_string(&mut encoded, self.gem_selected_material.as_str())?;
        binary_write_string(&mut encoded, self.gem_link_material.as_str())?;
        binary_write_string(&mut encoded, self.model_path.as_str())?;
        binary_write_i16(&mut encoded, self.kind)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}
impl<'a> BinaryDecode<'a> for Gem {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Gem> {

        let gid = binary_read_i64(cursor)?;
        let gem_icon = binary_read_string(cursor, bytes)?;
        let gem_selected_material = binary_read_string(cursor, bytes)?;
        let gem_link_material = binary_read_string(cursor, bytes)?;
        let model_path = binary_read_string(cursor, bytes)?;
        let kind = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Gem {
            gid,
            gem_icon,
            gem_selected_material,
            gem_link_material,
            model_path,
            kind,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
