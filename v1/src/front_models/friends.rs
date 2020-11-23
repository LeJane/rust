use crate::front_models::users::FrontDisplayChatUser;
use crate::{deserialize_binary, utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayFriend {
    pub fid: i64, //fid/black list id
    pub user: FrontDisplayChatUser,
    pub state: i16, //0：not friend 1:myself send request 2:peer send request,3:pass
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayGetSpecialUserFriendInfo {
    pub fid: i64,
    pub bid: i64,
    pub user: FrontDisplayChatUser,
    pub friend_state: i16, //0：not friend 1:myself send request 2:peer send request,3:pass
    pub blacklist_state: i16, //0:not 1:me to he black list
}

impl BinaryEncode for FrontDisplayFriend {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.fid)?;

        let user = self.user.encode()?;
        encoded.extend(user);

        binary_write_i16(&mut encoded, self.state)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayFriend {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayFriend> {
        let fid = binary_read_i64(cursor)?;

        let _user_item_length = binary_read_i32(cursor)?;
        let user: FrontDisplayChatUser = deserialize_binary(cursor, bytes)?;
        let state = binary_read_i16(cursor)?;

        let data = FrontDisplayFriend { fid, user, state };

        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayGetSpecialUserFriendInfo {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.fid)?;
        binary_write_i64(&mut encoded, self.bid)?;

        let user = self.user.encode()?;
        encoded.extend(user);

        binary_write_i16(&mut encoded, self.friend_state)?;
        binary_write_i16(&mut encoded, self.blacklist_state)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayGetSpecialUserFriendInfo {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
    ) -> Result<FrontDisplayGetSpecialUserFriendInfo> {
        let fid = binary_read_i64(cursor)?;
        let bid = binary_read_i64(cursor)?;

        let _user_item_length = binary_read_i32(cursor)?;
        let user: FrontDisplayChatUser = deserialize_binary(cursor, bytes)?;
        let friend_state = binary_read_i16(cursor)?;
        let blacklist_state = binary_read_i16(cursor)?;

        let data = FrontDisplayGetSpecialUserFriendInfo {
            fid,
            bid,
            user,
            friend_state,
            blacklist_state,
        };

        Ok(data)
    }
}
