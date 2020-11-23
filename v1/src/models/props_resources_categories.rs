use crate::schema::props_resources_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize, Identifiable, Queryable)]
#[primary_key(item_id)]
#[table_name = "props_resources_categories"]
pub struct PropsResourcesCategory {
    pub item_id: i64,
    pub rss_value: i32,
    pub attribute_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "props_resources_categories"]
pub struct NewPropsResourcesCategory {
    pub item_id: i64,
    pub rss_value: i32,
    pub attribute_id: i32,
}
