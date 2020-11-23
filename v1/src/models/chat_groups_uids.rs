use crate::schema::chat_groups_uids;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[primary_key(guid)]
pub struct ChatGroupsUid {
    pub guid: i64,
    pub gid: i64,
    pub uuid: i64,
    pub latest_timestamp: i64,
    pub unread_count: i16,
    pub modify_time: NaiveDateTime,
    pub created_time: NaiveDateTime,
}

#[derive(Debug, Default, Insertable)]
#[table_name = "chat_groups_uids"]
pub struct NewChatGroupUid {
    pub guid: i64,
    pub gid: i64,
    pub uuid: i64,
    pub latest_timestamp: i64,
    pub unread_count: i16,
}
