use crate::front_models::user_quests::FrontDisplayUserQuest;
use crate::models::user_quests::UserQuest;
use crate::schema::user_quests;
use diesel::prelude::*;

impl UserQuest {
    pub fn get_user_quests(conn: &PgConnection, uuid: i64, quests_id: i64) -> QueryResult<Self> {
        user_quests::table
            .filter(user_quests::uuid.eq(uuid))
            .filter(user_quests::quests_id.eq(quests_id))
            .first(conn)
    }

    pub fn get_daily_object_user_quests(
        conn: &PgConnection,
        uuid: i64,
        quests_id: i64,
        day_time: i64,
    ) -> QueryResult<Self> {
        user_quests::table
            .filter(user_quests::uuid.eq(uuid))
            .filter(user_quests::quests_id.eq(quests_id))
            .filter(user_quests::day_time.eq(day_time))
            .first(conn)
    }

    pub fn get_front_display_user_quests_default(uuid: i64) -> FrontDisplayUserQuest {
        let mut user_quests: FrontDisplayUserQuest = Default::default();

        user_quests.uuid = uuid;

        user_quests
    }
}
