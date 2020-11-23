use crate::models::skills::Skill;
use crate::schema::skills;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;
use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};

impl Skill {
    pub fn get_skill_by_id(conn: &PgConnection, skill_id: i64) -> QueryResult<Self> {
        skills::table
            .filter(skills::id.eq(skill_id))
            .get_result(conn)
    }

    pub fn get_skill_list(conn: &PgConnection) -> QueryResult<Vec<Self>> {
        skills::table.load(conn)
    }
}




impl MetadataInstance for Skill {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::Skill.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = Skill::get_skill_by_id(conn, id)?;

        Ok(MetadataTypeEnum::Skill(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = Skill::get_skill_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::Skill(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for Skill {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_string(&mut encoded, self.thumbnail.as_str())?;
        binary_write_string(&mut encoded, self.skill_name.as_str())?;
        binary_write_string(&mut encoded, self.skill_description.as_str())?;
        binary_write_string(&mut encoded, self.model_path.as_str())?;
        binary_write_i32(&mut encoded, self.cool_down)?;
        binary_write_i32(&mut encoded, self.attack_power)?;
        binary_write_i32(&mut encoded, self.mana_power)?;
        binary_write_i16(&mut encoded, self.level)?;
        binary_write_i32(&mut encoded, self.level_experience)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for Skill {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Skill> {

        let id = binary_read_i64(cursor)?;
        let thumbnail = binary_read_string(cursor, bytes)?;
        let skill_name = binary_read_string(cursor, bytes)?;
        let skill_description = binary_read_string(cursor, bytes)?;
        let model_path = binary_read_string(cursor, bytes)?;
        let cool_down = binary_read_i32(cursor)?;
        let attack_power = binary_read_i32(cursor)?;
        let mana_power = binary_read_i32(cursor)?;
        let level = binary_read_i16(cursor)?;
        let level_experience = binary_read_i32(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = Skill {
            id,
            thumbnail,
            skill_name,
            skill_description,
            model_path,
            cool_down,
            attack_power,
            mana_power,
            level,
            level_experience,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
