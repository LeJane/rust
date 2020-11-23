use crate::get_guid_value;
use crate::models::user_vips::{NewUserVip, UserVip};
use crate::schema::user_vips;
use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use diesel::prelude::*;

impl UserVip {
    pub fn create_user_vip(conn: &PgConnection, uuid: i64) -> QueryResult<()> {
        let d = NewUserVip {
            id: get_guid_value(),
            uuid,
            level: 0,
            vip_points: 0,
            daily_treasure_chests_time: 0,
            free_everyday_treasure_chests_time: 0,
            special_privileges_treasure_chests_time: 0,
        };

        diesel::insert_into(user_vips::table)
            .values(d)
            .execute(conn)?;

        Ok(())
    }

    pub fn exists_user_vip(conn: &PgConnection, uuid: i64) -> QueryResult<bool> {
        use diesel::dsl::exists;

        diesel::select(exists(user_vips::table.filter(user_vips::uuid.eq(uuid)))).get_result(conn)
    }

    pub fn update_user_vip_points(
        conn: &PgConnection,
        uid: i64,
        new_vip_points: i32,
    ) -> QueryResult<()> {
        let t = Utc::now();
        diesel::update(user_vips::table)
            .set((
                user_vips::vip_points.eq(new_vip_points),
                user_vips::modify_time.eq(t.naive_utc()),
            ))
            .filter(user_vips::uuid.eq(uid))
            .execute(conn)?;

        Ok(())
    }

    pub fn update_user_vip_level(conn: &PgConnection, uid: i64, level: i64) -> QueryResult<i64> {
        let t = chrono::Utc::now();
        diesel::update(user_vips::table)
            .set((
                user_vips::level.eq(level),
                user_vips::modify_time.eq(t.naive_utc()),
            ))
            .filter(user_vips::uuid.eq(uid))
            .returning(user_vips::level)
            .get_result(conn)
    }

    /*
        @param

        1:daily_treasure_chests_time
        2:free_everyday_treasure_chests_time
        3:special_privileges_treasure_chests_time
    */
    pub fn update_user_vip_treasure_chest_time(
        conn: &PgConnection,
        uid: i64,
        time_type: i16,
        value_time: i64,
    ) -> Result<()> {
        let t = Utc::now();

        match time_type {
            1 => diesel::update(user_vips::table)
                .set((
                    user_vips::daily_treasure_chests_time.eq(value_time),
                    user_vips::modify_time.eq(t.naive_utc()),
                ))
                .filter(user_vips::uuid.eq(uid))
                .execute(conn)?,
            2 => diesel::update(user_vips::table)
                .set((
                    user_vips::free_everyday_treasure_chests_time.eq(value_time),
                    user_vips::modify_time.eq(t.naive_utc()),
                ))
                .filter(user_vips::uuid.eq(uid))
                .execute(conn)?,
            3 => diesel::update(user_vips::table)
                .set((
                    user_vips::special_privileges_treasure_chests_time.eq(value_time),
                    user_vips::modify_time.eq(t.naive_utc()),
                ))
                .filter(user_vips::uuid.eq(uid))
                .execute(conn)?,
            _ => bail!("Unknown time type."),
        };

        Ok(())
    }

    pub fn get_user_vip_data(conn: &PgConnection, uuid: i64) -> Result<Self> {
        let user_vip_data: Self = user_vips::table
            .filter(user_vips::uuid.eq(uuid))
            .get_result(conn)
            .map_err(|e| anyhow!("failed get user vip info:{:?}", e))?;

        Ok(user_vip_data)
    }
}
