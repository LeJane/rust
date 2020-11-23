use crate::schema::quests_attribute_assets;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
pub struct QuestsAttributeAsset {
    pub id: i64,
    pub quests_id: i64,
    pub attribute_id: i32,
    pub amounts: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "quests_attribute_assets"]
pub struct NewQuestsAttributeAsset {
    pub id: i64,
    pub quests_id: i64,
    pub attribute_id: i32,
    pub amounts: i32,
}
