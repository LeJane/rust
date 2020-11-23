use crate::models::server_lists::ServerList;
use crate::schema::server_lists;
use crate::{
    utils::binary_read_helper::*, BinaryDecode, BinaryEncode, FrontDisplayMetaVersion,
    FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum, TableIdEnum,
};
use anyhow::Result;
use diesel::prelude::*;
use std::io::Cursor;

impl ServerList {
    pub fn get_server_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        server_lists::table
            .filter(server_lists::state.eq(0))
            .get_results(conn)
    }

    pub fn get_server_list_by_id(conn: &PgConnection, id: i64) -> QueryResult<Self> {
        server_lists::table
            .filter(server_lists::slid.eq(id))
            .filter(server_lists::state.eq(0))
            .get_result(conn)
    }
}

impl BinaryEncode for ServerList {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.slid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.country_code.as_str())?;
        binary_write_string(&mut encoded, self.area.as_str())?;
        binary_write_string(&mut encoded, self.ip.as_str())?;
        binary_write_i16(&mut encoded, self.port)?;
        binary_write_i16(&mut encoded, self.server_type)?;
        binary_write_i16(&mut encoded, self.state)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}


impl<'a> BinaryDecode<'a> for ServerList {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<ServerList> {
        let slid = binary_read_i64(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let country_code = binary_read_string(cursor, bytes)?;
        let area = binary_read_string(cursor, bytes)?;
        let ip = binary_read_string(cursor, bytes)?;
        let port = binary_read_i16(cursor)?;
        let server_type = binary_read_i16(cursor)?;
        let state = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = ServerList {
            slid,
            name,
            country_code,
            area,
            ip,
            port,
            server_type,
            state,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}

impl MetadataInstance for ServerList {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::ServerList.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let server_list = ServerList::get_server_list_by_id(conn, id)?;

        Ok(MetadataTypeEnum::ServerList(server_list))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = ServerList::get_server_list(conn)?;

        FrontDisplayMetaVersionRelation::get_front_display_meta_version_from_value(list, |data| {
            MetadataTypeEnum::ServerList(data)
        })
    }
}
