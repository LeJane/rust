use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Default, Debug, Clone)]
pub struct FrontDisplayUserQuest {
    pub uuid: i64,
    pub quests_id: i64,
    pub quests_finished_value: i32,
    pub is_finished: i32,
    pub is_receive_award: i16,
    pub day_time: i64, //daily object year*month*day
}
