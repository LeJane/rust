use crate::models::chat_groups::{ChatGroup, NewChatGroup};
use crate::schema::chat_groups;
use anyhow::Result;
use diesel::prelude::*;

impl ChatGroup {
    pub fn get_chat_group_by_gid(conn: &PgConnection, gid: i64) -> QueryResult<ChatGroup> {
        chat_groups::table
            .filter(chat_groups::gid.eq(gid))
            .first(conn)
    }

    pub fn create_group_chat(
        conn: &PgConnection,
        gid: i64,
        group_name: String,
        group_thumbnail: String,
        uuid: i64,
        person_count: i16,
    ) -> QueryResult<Self> {
        let data = NewChatGroup {
            gid,
            group_name: group_name.as_str(),
            group_thumbnail: group_thumbnail.as_str(),
            uuid,
            person_count,
        };

        diesel::insert_into(chat_groups::table)
            .values(data)
            .get_result(conn)
    }

    pub fn update_chat_group_person_count(
        conn: &PgConnection,
        person_count: i16,
        gid: i64,
    ) -> QueryResult<()> {
        diesel::update(chat_groups::table)
            .set(chat_groups::person_count.eq(chat_groups::person_count + person_count))
            .filter(chat_groups::gid.eq(gid))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_chat_group_owner_uid(conn: &PgConnection, gid: i64, uid: i64) -> QueryResult<()> {
        diesel::update(chat_groups::table)
            .set(chat_groups::uuid.eq(uid))
            .filter(chat_groups::gid.eq(gid))
            .execute(conn)?;
        Ok(())
    }

    pub fn update_chat_group_name(conn: &PgConnection, gid: i64, name: &str) -> QueryResult<()> {
        diesel::update(chat_groups::table)
            .set(chat_groups::group_name.eq(name))
            .filter(chat_groups::gid.eq(gid))
            .execute(conn)?;
        Ok(())
    }

    pub fn del_group_chat(conn: &PgConnection, gid: i64) -> Result<()> {
        diesel::delete(chat_groups::table)
            .filter(chat_groups::gid.eq(gid))
            .execute(conn)?;
        Ok(())
    }
}
