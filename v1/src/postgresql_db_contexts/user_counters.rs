use crate::models::user_counters::UserCounter;
use crate::schema::user_counters;
use diesel::prelude::*;

impl UserCounter {
    pub fn insert_and_get_id(conn: &PgConnection) -> QueryResult<i32> {
        diesel::insert_into(user_counters::table)
            .default_values()
            .returning(user_counters::id)
            .get_result(conn)
    }
}
