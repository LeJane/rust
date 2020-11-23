use crate::schema::props_action_points_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone, Queryable,Serialize,Deserialize, Identifiable)]
#[primary_key(item_id)]
#[table_name = "props_action_points_categories"]
pub struct PropsActionPointsCategory {
    pub item_id: i64,
    pub ap_value: i32,
    pub attribute_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "props_action_points_categories"]
pub struct NewPropsActionPointsCategory {
    pub item_id: i64,
    pub ap_value: i32,
    pub attribute_id: i32,
}
