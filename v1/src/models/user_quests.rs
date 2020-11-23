use crate::schema::user_quests;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, Clone)]
pub struct UserQuest {
    pub id: i64,
    pub uuid: i64,
    pub quests_id: i64,
    pub quests_finished_value: i32,
    pub is_finished: i32,      //0:not finished,1:finished
    pub is_receive_award: i16, //0:not received,1:received
    pub day_time: i64,         //daily object year*month*day
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "user_quests"]
pub struct NewUserQuest {
    pub id: i64,
    pub uuid: i64,
    pub quests_id: i64,
    pub quests_finished_value: i32,
    pub is_finished: i32,
    pub is_receive_award: i16,
}
