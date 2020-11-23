use crate::models::gem_relateds::GemRelated;
use crate::schema::gem_relateds;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl GemRelated {
    pub fn get_gem_related_list_obj_id(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
    ) -> QueryResult<Vec<Self>> {
        gem_relateds::table
            .filter(gem_relateds::obj_id.eq(obj_id))
            .filter(gem_relateds::obj_type.eq(obj_type))
            .get_results::<GemRelated>(conn)
    }

    pub fn get_gem_related_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        gem_relateds::table.get_results(conn)
    }

    pub fn get_gem_related_list_by_id(conn: &PgConnection, grid: i64) -> QueryResult<Self> {
        gem_relateds::table
            .filter(gem_relateds::grid.eq(grid))
            .get_result(conn)
    }
}

impl MetadataInstance for GemRelated {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::GemRelated.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = GemRelated::get_gem_related_list_by_id(conn, id)?;

        Ok(MetadataTypeEnum::GemRelated(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = GemRelated::get_gem_related_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::GemRelated(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for GemRelated {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.grid)?;
        binary_write_i64(&mut encoded, self.obj_id)?;
        binary_write_i64(&mut encoded, self.gid)?;
        binary_write_i16(&mut encoded, self.obj_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for GemRelated {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<GemRelated> {
        let grid = binary_read_i64(cursor)?;
        let obj_id = binary_read_i64(cursor)?;
        let gid = binary_read_i64(cursor)?;
        let obj_type = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = GemRelated {
            grid,
            obj_id,
            gid,
            obj_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
