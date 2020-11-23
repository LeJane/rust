use crate::schema::props_tome_of_knowledge_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};


#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(item_id)]
#[table_name = "props_tome_of_knowledge_categories"]
pub struct PropsTomeOfKnowledgeCategory {
    pub item_id: i64,
    pub exp_value: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_tome_of_knowledge_categories"]
pub struct NewPropsTomeOfKnowledgeCategory {
    pub item_id: i64,
    pub exp_value: i32,
}
