use crate::get_guid_value;
use crate::models::friends::{Friend, NewFriend};
use crate::schema::friends;
use diesel::prelude::*;

impl Friend {
    // state:2
    pub fn get_user_friend_list(
        conn: &PgConnection,
        uid: i64,
        state: i16,
    ) -> QueryResult<Vec<Self>> {
        friends::table
            .filter(friends::state.eq(state))
            .filter(friends::uuid_a.eq(uid))
            .load::<Friend>(conn)
    }

    // state:1
    pub fn get_user_new_friend_list(
        conn: &PgConnection,
        uid: i64,
        state: i16,
    ) -> QueryResult<Vec<Self>> {
        friends::table
            .filter(friends::state.eq(state))
            .filter(friends::uuid_b.eq(uid))
            .load::<Friend>(conn)
    }

    pub fn get_friend_by_fid(conn: &PgConnection, fid: i64) -> QueryResult<Friend> {
        friends::table.filter(friends::fid.eq(fid)).get_result(conn)
    }

    pub fn get_friend_info_by_fid(
        conn: &PgConnection,
        uuid_a: i64,
        uuid_b: i64,
    ) -> QueryResult<Friend> {
        friends::table
            .filter(friends::uuid_a.eq(uuid_a))
            .filter(friends::uuid_b.eq(uuid_b))
            .get_result(conn)
    }

    pub fn update_friend_state(conn: &PgConnection, fid: i64, state: i16) -> QueryResult<()> {
        diesel::update(friends::table)
            .set(friends::state.eq(state))
            .filter(friends::fid.eq(fid))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_friend_state_by_uid(
        conn: &PgConnection,
        uid_a: i64,
        uid_b: i64,
        state: i16,
    ) -> QueryResult<()> {
        diesel::update(friends::table)
            .set(friends::state.eq(state))
            .filter(friends::uuid_a.eq(uid_a))
            .filter(friends::uuid_b.eq(uid_b))
            .execute(conn)?;
        Ok(())
    }

    pub fn add_friend(
        conn: &PgConnection,
        uuid_a: i64,
        uuid_b: i64,
        state: i16,
    ) -> QueryResult<()> {
        let data = NewFriend {
            fid: get_guid_value() as i64,
            uuid_a,
            uuid_b,
            state,
        };

        diesel::insert_into(friends::table)
            .values(data)
            .execute(conn)?;
        Ok(())
    }

    pub fn find_friend_exists(conn: &PgConnection, uuid_a: i64, uuid_b: i64) -> QueryResult<bool> {
        use diesel::dsl::exists;
        let f = friends::table
            .filter(friends::uuid_a.eq(uuid_a))
            .filter(friends::uuid_b.eq(uuid_b));
        diesel::select(exists(f)).get_result(conn)
    }

    pub fn find_friend_state(conn: &PgConnection, uuid_a: i64, uuid_b: i64) -> QueryResult<i16> {
        friends::table
            .filter(friends::uuid_a.eq(uuid_a))
            .filter(friends::uuid_b.eq(uuid_b))
            .select(friends::state)
            .get_result(conn)
    }

    pub fn del_friend(conn: &PgConnection, fid: i64) -> QueryResult<()> {
        diesel::delete(friends::table)
            .filter(friends::fid.eq(fid))
            .execute(conn)?;
        Ok(())
    }

    pub fn del_friend_by_uuid(conn: &PgConnection, uid_a: i64, uid_b: i64) -> QueryResult<()> {
        diesel::delete(friends::table)
            .filter(friends::uuid_a.eq(uid_a))
            .filter(friends::uuid_b.eq(uid_b))
            .filter(friends::state.eq(2))
            .execute(conn)?;
        Ok(())
    }
}
