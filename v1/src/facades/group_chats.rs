use crate::front_models::{
    chat_groups::FrontDisplayChatGroup, chat_groups::FrontDisplayChatGroupDetail,
    chat_groups_uids::FrontDisplayChatGroupsUid,
};
use crate::models::{chat_groups::ChatGroup, chat_groups_uids::ChatGroupsUid, users::User};
use anyhow::{Error, Result};
use diesel::prelude::*;

pub fn get_my_groups_chat_list(
    conn: &PgConnection,
    uid: i64,
) -> QueryResult<Vec<FrontDisplayChatGroupsUid>> {
    let group_chat_uid_list = ChatGroupsUid::get_groups_chat_list_by_uid(conn, uid)?;

    let mut group_uid_list = Vec::with_capacity(group_chat_uid_list.len());

    for group_chat_uid in group_chat_uid_list.into_iter() {
        let chat_group_info = ChatGroup::get_chat_group_by_gid(conn, group_chat_uid.gid)?;

        let data = FrontDisplayChatGroupsUid {
            gid: chat_group_info.gid,
            group_name: chat_group_info.group_name,
            group_thumbnail: chat_group_info.group_thumbnail,
        };
        group_uid_list.push(data);
    }

    Ok(group_uid_list)
}

pub fn get_chat_group_by_gid(conn: &PgConnection, gid: i64) -> QueryResult<ChatGroup> {
    ChatGroup::get_chat_group_by_gid(conn, gid)
}
pub fn find_user_exists_group(conn: &PgConnection, gid: i64, uid: i64) -> QueryResult<bool> {
    ChatGroupsUid::find_user_exists_group(conn, gid, uid)
}

pub fn create_group_chat(
    conn: &PgConnection,
    gid: i64,
    group_name: Vec<String>,
    owner_uid: i64,
    uuids: Vec<i64>,
) -> Result<FrontDisplayChatGroup> {
    let resp_data = conn.transaction::<FrontDisplayChatGroup, Error, _>(|| {
        let group_chat_info = ChatGroup::create_group_chat(
            &conn,
            gid,
            group_name.join(","),
            "".into(),
            owner_uid,
            uuids.len() as i16,
        )?;
        ChatGroupsUid::create_multi_group_uids(conn, uuids, gid)?;

        Ok(FrontDisplayChatGroup {
            gid,
            group_name: group_chat_info.group_name,
            group_thumbnail: group_chat_info.group_thumbnail,
            uuid: group_chat_info.uuid,
            person_count: group_chat_info.person_count,
        })
    })?;

    Ok(resp_data)
}

pub fn add_member_to_group_chat(conn: &PgConnection, gid: i64, add_uuids: Vec<i64>) -> Result<()> {
    conn.transaction::<(), Error, _>(|| {
        ChatGroup::update_chat_group_person_count(conn, add_uuids.len() as i16, gid)?;

        ChatGroupsUid::create_multi_group_uids(conn, add_uuids, gid)?;

        Ok(())
    })?;

    Ok(())
}

pub fn del_member_from_group_chat(
    conn: &PgConnection,
    gid: i64,
    del_uuids: Vec<i64>,
) -> Result<()> {
    conn.transaction::<(), Error, _>(|| {
        ChatGroup::update_chat_group_person_count(conn, -(del_uuids.len() as i16), gid)?;
        ChatGroupsUid::del_multi_group_uid(conn, del_uuids, gid)?;

        Ok(())
    })?;

    Ok(())
}

pub fn update_chat_group_name(conn: &PgConnection, gid: i64, name: &str) -> QueryResult<()> {
    ChatGroup::update_chat_group_name(conn, gid, name)?;

    Ok(())
}

pub fn del_group_chat(conn: &PgConnection, gid: i64) -> Result<()> {
    conn.transaction::<(), Error, _>(|| {
        ChatGroup::del_group_chat(conn, gid)?;

        ChatGroupsUid::del_group_uids_by_gid(conn, gid)?;

        Ok(())
    })?;

    Ok(())
}

pub fn exit_group_chat(conn: &PgConnection, gid: i64, uid: i64) -> Result<()> {
    let group_chat = ChatGroup::get_chat_group_by_gid(conn, gid)?;

    if uid == group_chat.uuid {
        if ChatGroupsUid::get_groups_chat_list_member_count(conn, gid)? == 1 {
            ChatGroup::del_group_chat(conn, gid)?;
        } else {
            //find
            let new_owner_uid = ChatGroupsUid::get_second_groups_chat_uid(conn, gid, uid)?;

            conn.transaction::<(), Error, _>(|| {
                ChatGroup::update_chat_group_owner_uid(conn, gid, new_owner_uid)?;

                ChatGroupsUid::del_multi_group_uid(conn, vec![uid], gid)?;

                Ok(())
            })?;
        }
    } else {
        ChatGroupsUid::del_multi_group_uid(conn, vec![uid], gid)?;
    }

    Ok(())
}
pub fn get_group_chat_detail(
    conn: &PgConnection,
    gid: i64,
) -> QueryResult<FrontDisplayChatGroupDetail> {
    let group_chat = ChatGroup::get_chat_group_by_gid(conn, gid)?;

    let group_chat_uid_list = ChatGroupsUid::get_groups_chat_uid_list_by_gid(conn, gid)?;

    let mut members = Vec::with_capacity(group_chat_uid_list.len());

    for group_chat_uid in group_chat_uid_list.into_iter() {
        let member_user = User::get_front_display_chat_user_info(conn, group_chat_uid.uuid)?;

        members.push(member_user);
    }

    if !members.is_empty() && members[0].uuid != group_chat.uuid {
        let mut index = 0;
        for (i, member) in members.iter().enumerate() {
            if member.uuid == group_chat.uuid {
                index = i;
                break;
            }
        }

        members.swap(0, index);
    }

    Ok(FrontDisplayChatGroupDetail {
        gid,
        group_name: group_chat.group_name,
        group_thumbnail: group_chat.group_thumbnail,
        person_count: group_chat.person_count,
        members,
    })
}
