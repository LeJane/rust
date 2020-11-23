use crate::schema::props_key_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(item_id)]
#[table_name = "props_key_categories"]
pub struct PropsKeyCategory {
    pub item_id: i64,
    pub chest_item_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_key_categories"]
pub struct NewPropsKeyCategory {
    pub item_id: i64,
    pub chest_item_id: i64,
}
