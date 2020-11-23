use crate::schema::props_random_treasure_chest_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};


#[derive(Debug, Clone, Identifiable,Serialize,Deserialize, Queryable)]
#[primary_key(item_id)]
#[table_name = "props_random_treasure_chest_categories"]
pub struct PropsRandomTreasureChestCategory {
    pub item_id: i64,
    pub price: f32,
    pub is_instantly_open: i16, //1:not instantly open,2:instantly open
    pub option_values: i16,     //0:not,>0 select option_values count item
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "props_random_treasure_chest_categories"]
pub struct NewPropsRandomTreasureChestCategory {
    pub item_id: i64,
    pub price: f32,
    pub is_instantly_open: i16, //1:not instantly open,2:instantly open
    pub option_values: i16,     //0:not,>0 select option_values count item
}
