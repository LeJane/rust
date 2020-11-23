use crate::schema::props_fixed_treasure_chest_category_assets;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, Clone,Serialize,Deserialize, Identifiable, Queryable)]
#[primary_key(aid)]
pub struct PropsFixedTreasureChestCategoryAsset {
    pub aid: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[table_name = "props_fixed_treasure_chest_category_assets"]
pub struct NewPropsFixedTreasureChestCategoryAsset {
    pub aid: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
}
