use crate::schema::props_random_treasure_chest_category_attribute_assets;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
pub struct PropsRandomTreasureChestCategoryAttributeAsset {
    pub id: i64,
    pub item_id: i64,
    pub attribute_id: i32,
    pub amounts: i32,
    pub probability_value: f32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_random_treasure_chest_category_attribute_assets"]
pub struct NewPropsRandomTreasureChestCategoryAttributeAsset {
    pub id: i64,
    pub item_id: i64,
    pub attribute_id: i32,
    pub amounts: i32,
    pub probability_value: f32,
}
