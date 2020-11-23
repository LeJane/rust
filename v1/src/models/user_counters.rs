use crate::schema::user_counters;

#[derive(Clone, Debug, Identifiable, Queryable)]
#[primary_key(id)]
pub struct UserCounter {
    pub id: i32,
}
