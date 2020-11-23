use crate::schema::gems;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable)]
#[primary_key(gid)]
pub struct Gem {
    pub gid: i64,
    pub gem_icon: String,
    pub gem_selected_material: String,
    pub gem_link_material: String,
    pub model_path: String,
    pub kind: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "gems"]
pub struct NewGem<'a> {
    pub gid: i64,
    pub gem_icon: &'a str,
    pub gem_selected_material: &'a str,
    pub gem_link_material: &'a str,
    pub model_path: &'a str,
    pub kind: i16,
}
