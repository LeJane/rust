use crate::schema::quests_metadatas;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(quests_id)]
pub struct QuestsMetadata {
    pub quests_id: i64,
    pub name: String,
    pub thumbnail: String,
    pub description: String,
    pub quests_value: i32,
    pub quests_type: i32,     //main/side/daily object
    pub sub_quests_type: i32, //barbarians/build
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "quests_metadatas"]
pub struct NewQuestsMetadata<'a> {
    pub quests_id: i64,
    pub name: &'a str,
    pub thumbnail: &'a str,
    pub description: &'a str,
    pub quests_value: i32,
    pub quests_type: i32,
    pub sub_quests_type: i32,
}
