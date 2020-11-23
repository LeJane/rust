use crate::schema::categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable, Associations)]
#[table_name = "categories"]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub type_id: i32,
    pub type_name: String,
    pub system_name: String,
    pub system_id: i32,
    pub table_name: String,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}


#[derive(Debug, Default, Insertable)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub id: i64,
    pub name: &'a str,
    pub type_id: i32,
    pub type_name: &'a str,
    pub system_name: &'a str,
    pub system_id: i32,
    pub table_name: &'a str,
}
