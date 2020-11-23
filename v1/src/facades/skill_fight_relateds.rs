use super::versions;
use crate::{models::skill_fight_relateds::SkillFightRelated, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_skill_relation_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<SkillFightRelated>(conn, version_id)
}
