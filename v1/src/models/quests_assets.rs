use crate::schema::quests_assets;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
pub struct QuestsAsset {
    pub id: i64,
    pub quests_id: i64,
    pub item_id: i64,
    pub amounts: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "quests_assets"]
pub struct NewQuestsAsset {
    pub id: i64,
    pub quests_id: i64,
    pub item_id: i64,
    pub amounts: i32,
}
