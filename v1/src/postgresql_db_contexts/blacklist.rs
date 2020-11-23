use crate::get_guid_value;
use crate::models::blacklist::{Blacklist, NewBlacklist};
use crate::schema::blacklists;
use diesel::prelude::*;

impl Blacklist {
    pub fn get_black_list(conn: &PgConnection, uid: i64) -> QueryResult<Vec<Self>> {
        blacklists::table
            .filter(blacklists::uuid_a.eq(uid))
            .load::<Blacklist>(conn)
    }

    pub fn add_user_to_black_list(conn: &PgConnection, uid: i64, add_uid: i64) -> QueryResult<()> {
        let data = NewBlacklist {
            bid: get_guid_value() as i64,
            uuid_a: uid,
            uuid_b: add_uid,
        };

        diesel::insert_into(blacklists::table)
            .values(data)
            .execute(conn)?;

        Ok(())
    }

    pub fn find_user_black_list_exists(
        conn: &PgConnection,
        uid: i64,
        black_uid: i64,
    ) -> QueryResult<bool> {
        use diesel::dsl::exists;

        diesel::select(exists(
            blacklists::table
                .filter(blacklists::uuid_a.eq(uid))
                .filter(blacklists::uuid_b.eq(black_uid)),
        ))
        .get_result(conn)
    }

    pub fn find_user_black_list_bid(
        conn: &PgConnection,
        uid: i64,
        black_uid: i64,
    ) -> QueryResult<i64> {
        blacklists::table
            .filter(blacklists::uuid_a.eq(uid))
            .filter(blacklists::uuid_b.eq(black_uid))
            .select(blacklists::bid)
            .get_result(conn)
    }

    pub fn del_user_black(conn: &PgConnection, uuid_a: i64, uuid_b: i64) -> QueryResult<()> {
        diesel::delete(blacklists::table)
            .filter(blacklists::uuid_a.eq(uuid_a))
            .filter(blacklists::uuid_b.eq(uuid_b))
            .execute(conn)?;

        Ok(())
    }
}
