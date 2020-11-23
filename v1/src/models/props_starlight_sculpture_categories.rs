use crate::schema::props_starlight_sculpture_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};


#[derive(Queryable, Identifiable, Serialize,Deserialize,Debug, Clone)]
#[primary_key(item_id)]
#[table_name = "props_starlight_sculpture_categories"]
pub struct PropsStarlightSculptureCategory {
    pub item_id: i64,
    pub exp_value: i32,         //经验值
    pub probability_value: f32, //百分比
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_starlight_sculpture_categories"]
pub struct NewPropsStarlightSculptureCategory {
    pub item_id: i64,
    pub exp_value: i32,         //经验值
    pub probability_value: f32, //百分比
}
