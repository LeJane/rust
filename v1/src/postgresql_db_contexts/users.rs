use crate::front_models::users::FrontDisplayChatUser;
use crate::get_guid_value;
use crate::models::users::{NewUser, User};
use crate::schema::users;
use anyhow::Result;
use chrono::{Datelike, Utc};
use diesel::prelude::*;
use rand::Rng;

impl User {
    pub fn create_user(conn: &PgConnection, uid: i32, server_number: i32) -> Result<Self> {
        let uuid = get_guid_value();

        let name = format!("Governor{}", uid);
        let mut rng = rand::thread_rng();
        let rand_value: u64 = rng.gen_range(1, 3);
        let avatar = format!("game://{}", rand_value);

        let user = NewUser {
            uuid: uuid as i64,
            uid: uid as i32,
            name: &name,
            avatar: &avatar,
            login_days: 1,
            server_id: server_number,
            action_points: 1000,
            max_action_points: 1000,
            action_points_latest_timestamp: 0,
        };

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(conn)?;

        Ok(user)
    }

    pub fn get_user_info(conn: &PgConnection, uuid: i64) -> QueryResult<User> {
        users::table.filter(users::uuid.eq(uuid)).first(conn)
    }

    pub fn get_user_info_by_uid(conn: &PgConnection, uid: i32) -> QueryResult<User> {
        users::table.filter(users::uid.eq(uid)).first(conn)
    }

    pub fn update_user_action_points(
        conn: &PgConnection,
        uid: i64,
        action_points: i32,
    ) -> Result<()> {
        diesel::update(users::table)
            .set(users::action_points.eq(users::action_points + action_points))
            .filter(users::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }

    pub fn update_user_action_points_latest_timestamp(
        conn: &PgConnection,
        uid: i64,
        t: i64,
    ) -> Result<()> {
        diesel::update(users::table)
            .set(users::action_points_latest_timestamp.eq(t))
            .filter(users::uuid.eq(uid))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_user_name(conn: &PgConnection, uid: i64, name: &str) -> Result<()> {
        diesel::update(users::table)
            .set(users::name.eq(name))
            .filter(users::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }
    pub fn update_user_login_time_and_login_day(conn: &PgConnection, uid: i64) -> Result<()> {
        let user_info = Self::get_user_info(conn, uid)?;

        let t = Utc::now();

        if user_info.login_time.day() < t.day() {
            diesel::update(users::table)
                .set((
                    users::login_time.eq(t.naive_utc()),
                    users::login_days.eq(users::login_days + 1),
                ))
                .filter(users::uuid.eq(uid))
                .execute(conn)?;
        } else {
            diesel::update(users::table)
                .set(users::login_time.eq(t.naive_utc()))
                .filter(users::uuid.eq(uid))
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn get_front_display_chat_user_info(
        conn: &PgConnection,
        uuid: i64,
    ) -> QueryResult<FrontDisplayChatUser> {
        users::table
            .filter(users::uuid.eq(uuid))
            .select((
                users::uuid,
                users::uid,
                users::name,
                users::avatar,
                users::server_id,
                users::action_points,
            ))
            .first(conn)
    }

    pub fn find_user_exists(conn: &PgConnection, uid: i64) -> QueryResult<bool> {
        use diesel::dsl::exists;
        diesel::select(exists(users::table.filter(users::uuid.eq(uid)))).get_result(conn)
    }
}
