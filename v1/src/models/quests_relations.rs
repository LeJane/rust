use crate::schema::quests_relations;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
pub struct QuestsRelation {
    pub id: i64,
    pub quests_id: i64,
    pub quests_next_id: i64,
    pub endpoint: i32, //1:first2:middle,3:end
    pub quests_type: i32,
    pub sub_quests_type: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "quests_relations"]
pub struct NewQuestsRelation {
    pub id: i64,
    pub quests_id: i64,
    pub quests_next_id: i64,
    pub endpoint: i32,
    pub quests_type: i32,
    pub sub_quests_type: i32,
}
