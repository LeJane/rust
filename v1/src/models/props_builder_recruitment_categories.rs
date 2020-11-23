use crate::schema::props_builder_recruitment_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(item_id)]
#[table_name = "props_builder_recruitment_categories"]
pub struct PropsBuilderRecruitmentCategory {
    pub item_id: i64,
    pub recruit_time: i64,
    pub buff_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_builder_recruitment_categories"]
pub struct NewPropsBuilderRecruitmentCategory {
    pub item_id: i64,
    pub recruit_time: i64,
    pub buff_id: i64,
}
