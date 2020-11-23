use crate::schema::props_random_treasure_chest_category_assets;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
pub struct PropsRandomTreasureChestCategoryAsset {
    pub id: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
    pub probability_value: f32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_random_treasure_chest_category_assets"]
pub struct NewPropsRandomTreasureChestCategoryAsset {
    pub id: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
    pub probability_value: f32,
}
