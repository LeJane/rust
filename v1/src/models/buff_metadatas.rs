use crate::schema::buff_metadatas;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(buff_id)]
pub struct BuffMetadata {
    pub buff_id: i64,
    pub name: String,
    pub amounts: i32,
    pub attribute_id: i32,
    pub buff_category: i32,
    pub buff_type: i32,
    pub sub_buff_type: i32,
    pub buff_source: i32,
    pub is_show: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "buff_metadatas"]
pub struct NewBuffMetadata<'a> {
    pub buff_id: i64,
    pub name: &'a str,
    pub amounts: i32,
    pub attribute_id: i32,
    pub buff_category: i32,
    pub buff_type: i32,
    pub sub_buff_type: i32,
    pub buff_source: i32,
    pub is_show: i16,
}
