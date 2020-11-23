use crate::models::servers::Server;
use crate::schema::servers;
use diesel::prelude::*;
use crate::{ utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

use anyhow::Result;
use std::io::Cursor;

impl Server {
    pub fn get_server_socket_latest(conn: &PgConnection) -> QueryResult<Server> {
        servers::table
            .order_by(servers::person_count.desc())
            .first(conn)
    }

    pub fn get_server_by_id(conn: &PgConnection,sid:i64) -> QueryResult<Server> {
        servers::table
            .filter(servers::sid.eq(sid))
            .order_by(servers::person_count.desc())
            .get_result(conn)
    }
    pub fn get_server_list(conn: &PgConnection) -> QueryResult<Vec<Server>> {
        servers::table
            .order_by(servers::person_count.desc())
            .get_results(conn)
    }

    pub fn update_server_person_count(
        conn: &PgConnection,
        sid: i64,
        value: i32,
    ) -> QueryResult<()> {
        diesel::update(servers::table.filter(servers::sid.eq(sid)))
            .set(servers::person_count.eq(value))
            .execute(conn)?;

        Ok(())
    }
}


impl MetadataInstance for Server {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Server.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Server::get_server_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Server(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Server::get_server_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Server(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}


impl BinaryEncode for Server {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.sid)?;
        binary_write_i32(&mut encoded, self.server_number)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_i32(&mut encoded, self.person_count)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Server {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Server> {

        let sid = binary_read_i64(cursor)?;
        let server_number = binary_read_i32(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let person_count = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Server {
            sid,
            server_number,
            name,
            person_count,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
