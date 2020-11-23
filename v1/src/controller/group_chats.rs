use crate::default_log_pre;
use crate::facades::{group_chats, users};
use crate::get_guid_value;
use crate::utils::binary_read_helper::{binary_read_i32, binary_read_i64, binary_read_string};
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_group_chat_list(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let resp_data = group_chats::get_my_groups_chat_list(&conn, uid)?;

    req.get_bincode(200, "Success", resp_data)
}

#[named]
pub async fn create_group_chat(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());

    let owner_uid = binary_read_i64(&mut cursor)?;
    if owner_uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, owner_uid)? {
        return Err(anyhow!("owner user not exists."));
    }

    let total_length = binary_read_i32(&mut cursor)?;

    let len = total_length / 8;

    if len < 2 {
        return Err(anyhow!("invalid group chat person count."));
    }

    let mut group_name = Vec::with_capacity((len + 1) as usize);
    let owner_user = users::get_user_info(&slave_conn, owner_uid)?;
    group_name.push(owner_user.name);

    let mut uuids = Vec::with_capacity((len + 1) as usize);
    uuids.push(owner_uid);
    let gid = get_guid_value() as i64;

    for _x in 0..len {
        let uid = binary_read_i64(&mut cursor)?;

        if _x < 2 {
            let user = users::get_user_info(&slave_conn, uid)?;
            group_name.push(user.name);
        }

        uuids.push(uid);
    }

    info!(
        "{}\tsubmit content\towner_uid:{}\ttotal_length:{}\tuuids:{:?}",
        default_log_pre!(req.code, owner_uid),
        owner_uid,
        total_length,
        uuids
    );

    let resp_data =
        group_chats::create_group_chat(&master_conn, gid, group_name, owner_uid, uuids)?;

    req.get_bincode(200, "Success", resp_data)
}

#[named]
pub async fn add_member_to_group_chat(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let gid = binary_read_i64(&mut cursor)?;
    if gid <= 0 {
        return Err(anyhow!("invalid group id."));
    }

    let group_info = group_chats::get_chat_group_by_gid(&slave_conn, gid)?;

    if group_info.person_count + 1 > 500 {
        return Err(anyhow!("group chat person too many."));
    }

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    if !group_chats::find_user_exists_group(&slave_conn, gid, uid)? {
        return Err(anyhow!("this user not in group chat."));
    }

    let total_length = binary_read_i32(&mut cursor)?;

    let len = total_length / 8;
    let mut add_uuids = Vec::with_capacity(len as usize);

    for _x in 0..len {
        let add_uid = binary_read_i64(&mut cursor)?;
        if add_uid <= 0 {
            return Err(anyhow!("invalid uuid."));
        }

        if !users::find_user_exists(&slave_conn, add_uid)? {
            return Err(anyhow!("user not exists."));
        }

        let exists = group_chats::find_user_exists_group(&slave_conn, gid, add_uid)?;

        if exists {
            continue;
        }

        add_uuids.push(add_uid);
    }

    info!(
        "{}\tsubmit content\tgid:{}\tuuid:{}\ttotal_length:{}\tadded uuids:{:?}",
        default_log_pre!(req.code, uid),
        gid,
        uid,
        total_length,
        add_uuids,
    );

    group_chats::add_member_to_group_chat(&master_conn, gid, add_uuids)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn del_member_from_group_chat(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let gid = binary_read_i64(&mut cursor)?;
    if gid <= 0 {
        return Err(anyhow!("invalid group id."));
    }

    let group_info = group_chats::get_chat_group_by_gid(&slave_conn, gid)?;

    if group_info.person_count + 1 > 500 {
        return Err(anyhow!("group chat person too many."));
    }

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    if group_info.uuid != uid {
        return Err(anyhow!("user not owner."));
    }

    let total_length = binary_read_i32(&mut cursor)?;

    let len = total_length / 8;
    let mut del_uuids = Vec::with_capacity(len as usize);

    for _x in 0..len {
        let del_uid = binary_read_i64(&mut cursor)?;
        if del_uid <= 0 {
            return Err(anyhow!("invalid uuid."));
        }

        if uid == del_uid {
            return Err(anyhow!("cannot delete owner."));
        }

        if !users::find_user_exists(&slave_conn, del_uid)? {
            return Err(anyhow!("user not exists."));
        }

        if !group_chats::find_user_exists_group(&slave_conn, gid, del_uid)? {
            return Err(anyhow!("this user not in group chat."));
        }

        del_uuids.push(del_uid);
    }

    info!(
        "{}\tsubmit content\tgid:{}\tuuid:{}\ttotal_length:{}\tdeleted uuids:{:?}",
        default_log_pre!(req.code, uid),
        gid,
        uid,
        total_length,
        del_uuids,
    );

    group_chats::del_member_from_group_chat(&master_conn, gid, del_uuids)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn update_group_chat_name(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let gid = binary_read_i64(&mut cursor)?;
    if gid <= 0 {
        return Err(anyhow!("invalid group id."));
    }

    let name = binary_read_string(&mut cursor, req.body.as_slice())?;

    info!(
        "{}\tsubmit content\tgid:{}\tname:{}",
        default_log_pre!(req.code, ""),
        gid,
        name,
    );

    group_chats::update_chat_group_name(&master_conn, gid, &name)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn exit_group_chat(req: ReqContext) -> ResponseResult {
    let master_conn = req.db_conn(true)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let gid = binary_read_i64(&mut cursor)?;
    if gid <= 0 {
        return Err(anyhow!("invalid group id."));
    }

    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    info!(
        "{}\tsubmit content\tgid:{}\tuuid:{}",
        default_log_pre!(req.code, uid),
        gid,
        uid
    );

    group_chats::exit_group_chat(&master_conn, gid, uid)?;

    req.get_bincode(200, "Success", "")
}

#[named]
pub async fn get_group_chat_detail(req: ReqContext) -> ResponseResult {
    let conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let gid = binary_read_i64(&mut cursor)?;
    if gid <= 0 {
        return Err(anyhow!("invalid group id."));
    }

    info!(
        "{}\tsubmit content\tgid:{}",
        default_log_pre!(req.code, ""),
        gid
    );

    let resp = group_chats::get_group_chat_detail(&conn, gid)?;

    req.get_bincode(200, "Success", resp)
}
