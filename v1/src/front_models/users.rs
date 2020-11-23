use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayChatUser {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub server_id: i32,
    pub action_points: i32,
}

#[derive(Clone, Debug, Queryable)]
pub struct FrontDisplayChatUserAndFriendState {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub server_id: i32,
    pub action_points: i32,
    pub friend_state: i16, //0：not friend 1:myself send request 2:peer send request,3:pass
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct FrontDisplayUser {
    pub uuid: i64,
    pub uid: i32,
    pub name: String,
    pub avatar: String,
    pub server_id: i32,
    pub action_points: i32,
    pub gold_amounts: i32,
    pub gem_amounts: i32,
    pub food_amounts: i32,
    pub wood_amounts: i32,
    pub stone_amounts: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct FrontDisplayUserActionPoint {
    pub uuid: i64,
    pub action_points: i32,
    pub max_action_points: i32,
    pub one_point_every_sec: i8,
    pub recover_speed: i32,
    pub recover_time: i64,
}

impl BinaryEncode for FrontDisplayUser {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i32(&mut encoded, self.uid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.avatar.as_str())?;
        binary_write_i32(&mut encoded, self.server_id)?;
        binary_write_i32(&mut encoded, self.action_points)?;
        binary_write_i32(&mut encoded, self.gold_amounts)?;
        binary_write_i32(&mut encoded, self.gem_amounts)?;
        binary_write_i32(&mut encoded, self.food_amounts)?;
        binary_write_i32(&mut encoded, self.wood_amounts)?;
        binary_write_i32(&mut encoded, self.stone_amounts)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUser {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayUser> {
        let uuid = binary_read_i64(cursor)?;
        let uid = binary_read_i32(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let avatar = binary_read_string(cursor, bytes)?;
        let server_id = binary_read_i32(cursor)?;
        let action_points = binary_read_i32(cursor)?;
        let gold_amounts = binary_read_i32(cursor)?;
        let gem_amounts = binary_read_i32(cursor)?;
        let food_amounts = binary_read_i32(cursor)?;
        let wood_amounts = binary_read_i32(cursor)?;
        let stone_amounts = binary_read_i32(cursor)?;

        let data = FrontDisplayUser {
            uuid,
            uid,
            name,
            avatar,
            server_id,
            action_points,
            gold_amounts,
            gem_amounts,
            food_amounts,
            wood_amounts,
            stone_amounts,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayChatUser {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i32(&mut encoded, self.uid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.avatar.as_str())?;
        binary_write_i32(&mut encoded, self.server_id)?;
        binary_write_i32(&mut encoded, self.action_points)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayChatUser {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayChatUser> {
        let uuid = binary_read_i64(cursor)?;
        let uid = binary_read_i32(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let avatar = binary_read_string(cursor, bytes)?;
        let server_id = binary_read_i32(cursor)?;
        let action_points = binary_read_i32(cursor)?;

        let data = FrontDisplayChatUser {
            uuid,
            uid,
            name,
            avatar,
            server_id,
            action_points,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayChatUserAndFriendState {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i32(&mut encoded, self.uid)?;
        binary_write_string(&mut encoded, self.name.as_str())?;
        binary_write_string(&mut encoded, self.avatar.as_str())?;
        binary_write_i32(&mut encoded, self.server_id)?;
        binary_write_i32(&mut encoded, self.action_points)?;
        binary_write_i16(&mut encoded, self.friend_state)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayChatUserAndFriendState {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
    ) -> Result<FrontDisplayChatUserAndFriendState> {
        let uuid = binary_read_i64(cursor)?;
        let uid = binary_read_i32(cursor)?;
        let name = binary_read_string(cursor, bytes)?;
        let avatar = binary_read_string(cursor, bytes)?;
        let server_id = binary_read_i32(cursor)?;
        let action_points = binary_read_i32(cursor)?;
        let friend_state = binary_read_i16(cursor)?;

        let data = FrontDisplayChatUserAndFriendState {
            uuid,
            uid,
            name,
            avatar,
            server_id,
            action_points,
            friend_state,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayUserActionPoint {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i32(&mut encoded, self.action_points)?;
        binary_write_i32(&mut encoded, self.max_action_points)?;
        binary_write_i8(&mut encoded, self.one_point_every_sec)?;
        binary_write_i32(&mut encoded, self.recover_speed)?;
        binary_write_i64(&mut encoded, self.recover_time)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserActionPoint {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayUserActionPoint> {
        let uuid = binary_read_i64(cursor)?;
        let action_points = binary_read_i32(cursor)?;
        let max_action_points = binary_read_i32(cursor)?;
        let one_point_every_sec = binary_read_i8(cursor)?;
        let recover_speed = binary_read_i32(cursor)?;
        let recover_time = binary_read_i64(cursor)?;

        let data = FrontDisplayUserActionPoint {
            uuid,
            action_points,
            max_action_points,
            one_point_every_sec,
            recover_speed,
            recover_time,
        };
        Ok(data)
    }
}
