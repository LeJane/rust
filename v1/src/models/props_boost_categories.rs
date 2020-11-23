use crate::schema::props_boost_categories;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(item_id)]
#[table_name = "props_boost_categories"]
pub struct PropsBoostCategory {
    pub item_id: i64,
    pub boost_time: i64,
    pub boost_value: f32,
    pub buff_id: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_boost_categories"]
pub struct NewPropsBoostCategory {
    pub item_id: i64,
    pub boost_time: i64,
    pub boost_value: f32,
    pub buff_id: i64,
}
