use crate::schema::props_product_numbers;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};

#[derive(Queryable, Identifiable,Serialize,Deserialize, Debug, Clone)]
#[primary_key(item_id)]
pub struct PropsProductNumber {
    pub item_id: i64,
    pub product_number: String,
    pub platform_id: i16, //1:google,2:ios
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name = "props_product_numbers"]
pub struct NewPropsProductNumber<'a> {
    pub item_id: i64,
    pub product_number: &'a str,
    pub platform_id: i16,
}
