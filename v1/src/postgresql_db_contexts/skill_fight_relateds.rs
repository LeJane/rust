use crate::models::skill_fight_relateds::SkillFightRelated;
use crate::schema::skill_fight_relateds;
use diesel::prelude::*;
use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

use crate::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, MetadataInstance, MetadataTypeEnum,
    TableIdEnum,
};


impl SkillFightRelated {
    pub fn get_skill_related_list_by_obj_id(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
    ) -> QueryResult<Vec<Self>> {
        skill_fight_relateds::table
            .filter(skill_fight_relateds::obj_id.eq(obj_id))
            .filter(skill_fight_relateds::obj_type.eq(obj_type))
            .get_results(conn)
    }

    pub fn get_skill_related_by_id(
        conn: &PgConnection,
        id: i64,
    ) -> QueryResult<Self> {
        skill_fight_relateds::table
            .filter(skill_fight_relateds::id.eq(id))
            .get_result(conn)
    }
    pub fn get_skill_related_list(
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        skill_fight_relateds::table
            .get_results(conn)
    }
}



impl MetadataInstance for SkillFightRelated {
    fn get_table_id() -> Result<i32> {
        Ok(TableIdEnum::SkillFightRelated.to_i32())
    }

    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum> {
        let data = SkillFightRelated::get_skill_related_by_id(conn, id)?;

        Ok(MetadataTypeEnum::SkillFightRelated(data))
    }

    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion> {
        let list = SkillFightRelated::get_skill_related_list(conn)?;
        let table_id = Self::get_table_id()?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = list
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: MetadataTypeEnum::SkillFightRelated(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for SkillFightRelated {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.id)?;
        binary_write_i64(&mut encoded, self.obj_id)?;
        binary_write_i64(&mut encoded, self.skill_id)?;
        binary_write_i32(&mut encoded, self.cool_down)?;
        binary_write_i32(&mut encoded, self.attack_power)?;
        binary_write_i32(&mut encoded, self.mana_power)?;
        binary_write_i16(&mut encoded, self.probability)?;
        binary_write_i16(&mut encoded, self.level)?;
        binary_write_i32(&mut encoded, self.level_experience)?;
        binary_write_i16(&mut encoded, self.obj_type)?;
        binary_write_time(&mut encoded, self.modify_time)?;
        binary_write_time(&mut encoded, self.created_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for SkillFightRelated {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<SkillFightRelated> {

        let id = binary_read_i64(cursor)?;
        let obj_id = binary_read_i64(cursor)?;
        let skill_id = binary_read_i64(cursor)?;
        let cool_down = binary_read_i32(cursor)?;
        let attack_power = binary_read_i32(cursor)?;
        let mana_power = binary_read_i32(cursor)?;
        let probability = binary_read_i16(cursor)?;
        let level = binary_read_i16(cursor)?;
        let level_experience = binary_read_i32(cursor)?;
        let obj_type = binary_read_i16(cursor)?;
        let modify_time = binary_read_time(cursor, bytes)?;
        let created_time = binary_read_time(cursor, bytes)?;

        let data = SkillFightRelated {
            id,
            obj_id,
            skill_id,
            cool_down,
            attack_power,
            mana_power,
            probability,
            level,
            level_experience,
            obj_type,
            modify_time,
            created_time,
        };
        Ok(data)
    }
}
