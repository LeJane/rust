use crate::get_guid_value;
use crate::models::user_queues::{NewUserQueue, UserQueue};
use crate::schema::user_queues;
use diesel::prelude::*;

impl UserQueue {
    pub fn create_user_default_queue(conn: &PgConnection, uid: i64) -> QueryResult<()> {
        let d = NewUserQueue {
            qid: get_guid_value(),
            uuid: uid,
            building_queue: 0,
            research_queue: 0,
            training_queue: 0,
            healing_queue: 0,
            armies_queue: 0,
            scout_queue: 0,
        };

        diesel::insert_into(user_queues::table)
            .values(d)
            .execute(conn)?;

        Ok(())
    }

    pub fn update_user_building_queue(
        conn: &PgConnection,
        uid: i64,
        building_queue: i16,
    ) -> QueryResult<()> {
        diesel::update(user_queues::table)
            .set(user_queues::building_queue.eq(building_queue))
            .filter(user_queues::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }
}
