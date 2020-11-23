use crate::schema::user_equipments;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct UserEquipment {
    pub id: i64,
    pub eid: i64,
    pub uid: i64,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "user_equipments"]
pub struct NewUserEquipment {
    pub id: i64,
    pub eid: i64,
    pub uid: i64,
}
