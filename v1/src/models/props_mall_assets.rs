use crate::schema::props_mall_assets;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Deserialize, Serialize, Debug, Clone)]
#[primary_key(item_id)]
pub struct PropsMallAsset {
    pub aid: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_mall_assets"]
pub struct NewPropsMallAsset {
    pub aid: i64,
    pub item_id: i64,
    pub sub_item_id: i64,
    pub amounts: i32,
}
