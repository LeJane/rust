use crate::schema::user_assets;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[primary_key(asid)]
pub struct UserAsset {
    pub asid: i64,
    pub uid: i64,
    pub gold_amounts: i32,
    pub gem_amounts: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub stone_amounts: i32,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_assets"]
pub struct NewUserAsset {
    pub asid: i64,
    pub uid: i64,
    pub gold_amounts: i32,
    pub gem_amounts: i32,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub stone_amounts: i32,
}
