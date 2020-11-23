use crate::models::quests_relations::QuestsRelation;
use crate::schema::quests_relations;
use anyhow::{anyhow, Result};
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use std::io::Cursor;
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl QuestsRelation {
    pub fn get_main_side_quests_relations(conn: &PgConnection) -> Result<Vec<Self>> {
        quests_relations::table
            .filter(quests_relations::quests_type.eq_any(vec![1, 2]))
            .filter(quests_relations::endpoint.eq(1))
            .load(conn)
            .map_err(|e| anyhow!("failed get main and side quests:{}", e))
    }

    pub fn get_daily_object_quests(conn: &PgConnection) -> Result<Vec<Self>> {
        quests_relations::table
            .filter(quests_relations::quests_type.eq(1))
            .filter(quests_relations::endpoint.eq(1))
            .load(conn)
            .map_err(|e| anyhow!("failed get daily object quests:{}", e))
    }

    pub fn get_quests_relation_by_quests_id(
        conn: &PgConnection,
        quests_id: i64,
    ) -> QueryResult<Self> {
        quests_relations::table
            .filter(quests_relations::quests_id.eq(quests_id))
            .first(conn)
    }

    pub fn get_quests_relation_by_id(
        conn: &PgConnection,
        id: i64,
    ) -> QueryResult<Self> {
        quests_relations::table
            .filter(quests_relations::id.eq(id))
            .first(conn)
    }

    pub fn get_quests_relation_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        quests_relations::table
            .load(conn)
    }
}


impl MetadataInstance for QuestsRelation {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::QuestsRelation.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = QuestsRelation::get_quests_relation_by_id(conn, id)?;

        Ok(MetadataTypeEnum::QuestsRelation(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = QuestsRelation::get_quests_relation_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::QuestsRelation(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for QuestsRelation {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.quests_id)?;
        binary_write_i64(&mut encoded, self.quests_next_id)?;
        binary_write_i32(&mut encoded, self.endpoint)?;
        binary_write_i32(&mut encoded, self.quests_type)?;
        binary_write_i32(&mut encoded, self.sub_quests_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for QuestsRelation {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<QuestsRelation> {

        let id = binary_read_i64(cursor)?;
        let quests_id = binary_read_i64(cursor)?;
        let quests_next_id = binary_read_i64(cursor)?;
        let endpoint = binary_read_i32(cursor)?;
        let quests_type = binary_read_i32(cursor)?;
        let sub_quests_type = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = QuestsRelation {
            id,
            quests_id,
            quests_next_id,
            endpoint,
            quests_type,
            sub_quests_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}


