use crate::schema::user_queues;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(qid)]
pub struct UserQueue {
    pub qid: i64,
    pub uuid: i64,
    pub building_queue: i16,
    pub research_queue: i16,
    pub training_queue: i16,
    pub healing_queue: i16,
    pub armies_queue: i16,
    pub scout_queue: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "user_queues"]
pub struct NewUserQueue {
    pub qid: i64,
    pub uuid: i64,
    pub building_queue: i16,
    pub research_queue: i16,
    pub training_queue: i16,
    pub healing_queue: i16,
    pub armies_queue: i16,
    pub scout_queue: i16,
}
