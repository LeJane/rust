use crate::models::props_tome_of_knowledge_categories::PropsTomeOfKnowledgeCategory;
use crate::schema::props_tome_of_knowledge_categories;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl PropsTomeOfKnowledgeCategory {
    pub fn get_tome_of_knowledge_props_by_item_id(
        conn: &PgConnection,
        item_id: i64,
    ) -> QueryResult<Self> {
        props_tome_of_knowledge_categories::table
            .filter(props_tome_of_knowledge_categories::item_id.eq(item_id))
            .first(conn)
    }

    pub fn get_tome_of_knowledge_props_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        props_tome_of_knowledge_categories::table
            .load(conn)
    }
}




impl MetadataInstance for PropsTomeOfKnowledgeCategory {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsTomeOfKnowledgeCategory.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsTomeOfKnowledgeCategory::get_tome_of_knowledge_props_by_item_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsTomeOfKnowledgeCategory(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsTomeOfKnowledgeCategory::get_tome_of_knowledge_props_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsTomeOfKnowledgeCategory(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsTomeOfKnowledgeCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i32(&mut encoded, self.exp_value)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}


impl<'a> BinaryDecode<'a> for PropsTomeOfKnowledgeCategory {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsTomeOfKnowledgeCategory> {

        let item_id = binary_read_i64(cursor)?;
        let exp_value = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsTomeOfKnowledgeCategory {
            item_id,
            exp_value,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
