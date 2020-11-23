use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[primary_key(uuid)]
pub struct User {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub login_days: i32,
    pub server_id: i32,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
    pub action_points: i32,
    pub max_action_points: i32,
    pub action_points_latest_timestamp: i64,
    pub login_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uuid: i64,
    pub uid: i32,
    pub name: &'a str,
    pub avatar: &'a str,
    pub login_days: i32,
    pub server_id: i32,
    pub action_points: i32,
    pub max_action_points: i32,
    pub action_points_latest_timestamp: i64,
}
