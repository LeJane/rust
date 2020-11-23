use crate::schema::props_item_metadatas;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable, Serialize,Deserialize,Debug, Clone)]
#[primary_key(item_id)]
pub struct PropsItemMetadata {
    pub item_id: i64,
    pub name: String,
    pub thumbnail: String,
    pub description: String,
    pub overlay_status: i16,
    //1:can overlay,2:can't overlay
    pub sub_item_type: i32,
    pub bag_type: i32,
    //items->101:Resources,102:speedup,103:Buff,104:Equipment,105:Material,106:Building,107:Treasure Chest,108:Action Point,109:random Treasure Chest
    pub rarity_type: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_item_metadatas"]
pub struct NewPropsItemMetadata {
    pub item_id: i64,
    pub name: String,
    pub thumbnail: String,
    pub description: String,
    pub overlay_status: i16,
    //1:can overlay,2:can't overlay
    pub sub_item_type: i32,
    pub bag_type: i32,
    pub rarity_type: i32,
}
