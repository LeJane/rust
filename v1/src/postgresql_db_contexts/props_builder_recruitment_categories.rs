use crate::models::props_builder_recruitment_categories::PropsBuilderRecruitmentCategory;
use crate::schema::props_builder_recruitment_categories;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl PropsBuilderRecruitmentCategory {
    pub fn get_builder_recruitment_props_by_item_id(
        conn: &PgConnection,
        item_id: i64,
    ) -> QueryResult<Self> {
        props_builder_recruitment_categories::table
            .filter(props_builder_recruitment_categories::item_id.eq(item_id))
            .first(conn)
    }

    pub fn get_builder_recruitment_props_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        props_builder_recruitment_categories::table
            .load(conn)
    }
}


impl MetadataInstance for PropsBuilderRecruitmentCategory {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::PropsBuilderRecruitmentCategory.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = PropsBuilderRecruitmentCategory::get_builder_recruitment_props_by_item_id(conn, id)?;

        Ok(MetadataTypeEnum::PropsBuilderRecruitmentCategory(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = PropsBuilderRecruitmentCategory::get_builder_recruitment_props_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::PropsBuilderRecruitmentCategory(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for PropsBuilderRecruitmentCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i64(&mut encoded, self.recruit_time)?;
        binary_write_i64(&mut encoded, self.buff_id)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}


impl<'a> BinaryDecode<'a> for PropsBuilderRecruitmentCategory {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<PropsBuilderRecruitmentCategory> {

        let item_id = binary_read_i64(cursor)?;
        let recruit_time = binary_read_i64(cursor)?;
        let buff_id = binary_read_i64(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = PropsBuilderRecruitmentCategory {
            item_id,
            recruit_time,
            buff_id,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
