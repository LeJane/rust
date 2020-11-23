use crate::schema::props_speed_up_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable)]
#[primary_key(item_id)]
#[table_name = "props_speed_up_categories"]
pub struct PropsSpeedUpCategory {
    pub item_id: i64,
    pub speed_time: i32,
    pub attribute_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "props_speed_up_categories"]
pub struct NewPropsSpeedUpCategory {
    pub item_id: i64,
    pub speed_time: i32,
    pub attribute_id: i32,
}
