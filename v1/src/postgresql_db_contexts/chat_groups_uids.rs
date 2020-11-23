use crate::get_guid_value;
use crate::models::chat_groups_uids::{ChatGroupsUid, NewChatGroupUid};
use crate::schema::chat_groups_uids;
use diesel::prelude::*;

impl ChatGroupsUid {
    pub fn get_groups_chat_list_by_uid(conn: &PgConnection, uid: i64) -> QueryResult<Vec<Self>> {
        chat_groups_uids::table
            .filter(chat_groups_uids::uuid.eq(uid))
            .load(conn)
    }

    pub fn get_groups_chat_uid_list_by_gid(
        conn: &PgConnection,
        gid: i64,
    ) -> QueryResult<Vec<Self>> {
        chat_groups_uids::table
            .filter(chat_groups_uids::gid.eq(gid))
            .order_by(chat_groups_uids::created_time.asc())
            .load(conn)
    }

    pub fn get_groups_chat_list_member_count(conn: &PgConnection, gid: i64) -> QueryResult<i64> {
        use diesel::dsl::count;

        chat_groups_uids::table
            .filter(chat_groups_uids::gid.eq(gid))
            .select(count(chat_groups_uids::uuid))
            .first(conn)
    }

    pub fn get_second_groups_chat_uid(conn: &PgConnection, gid: i64, uid: i64) -> QueryResult<i64> {
        chat_groups_uids::table
            .filter(chat_groups_uids::gid.eq(gid))
            .filter(chat_groups_uids::uuid.ne(uid))
            .order_by(chat_groups_uids::created_time.desc())
            .select(chat_groups_uids::uuid)
            .first(conn)
    }

    pub fn create_multi_group_uids(
        conn: &PgConnection,
        uuids: Vec<i64>,
        gid: i64,
    ) -> QueryResult<()> {
        let mut datas = Vec::with_capacity(uuids.len());

        for uuid in uuids.into_iter() {
            let data = NewChatGroupUid {
                guid: get_guid_value() as i64,
                gid,
                uuid,
                latest_timestamp: 0,
                unread_count: 0,
            };

            datas.push(data);
        }

        diesel::insert_into(chat_groups_uids::table)
            .values(datas)
            .execute(conn)?;
        Ok(())
    }

    pub fn find_user_exists_group(conn: &PgConnection, gid: i64, uid: i64) -> QueryResult<bool> {
        use diesel::dsl::exists;
        let f = chat_groups_uids::table
            .filter(chat_groups_uids::gid.eq(gid))
            .filter(chat_groups_uids::uuid.eq(uid));
        diesel::select(exists(f)).get_result(conn)
    }

    pub fn del_multi_group_uid(conn: &PgConnection, uuids: Vec<i64>, gid: i64) -> QueryResult<()> {
        for uuid in uuids.into_iter() {
            diesel::delete(chat_groups_uids::table)
                .filter(chat_groups_uids::gid.eq(gid))
                .filter(chat_groups_uids::uuid.eq(uuid))
                .execute(conn)?;
        }

        Ok(())
    }

    pub fn del_group_uids_by_gid(conn: &PgConnection, gid: i64) -> QueryResult<()> {
        diesel::delete(chat_groups_uids::table)
            .filter(chat_groups_uids::gid.eq(gid))
            .execute(conn)?;
        Ok(())
    }
}
