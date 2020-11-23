use crate::front_models::users::FrontDisplayChatUser;
use crate::{deserialize_binary, utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

#[derive(Debug, Clone, Queryable)]
pub struct FrontDisplayChatGroup {
    pub gid: i64,
    pub group_name: String,
    pub group_thumbnail: String,
    pub uuid: i64,
    pub person_count: i16,
}

#[derive(Debug, Clone, Queryable)]
pub struct FrontDisplayChatGroupDetail {
    pub gid: i64,
    pub group_name: String,
    pub group_thumbnail: String,
    pub person_count: i16,
    pub members: Vec<FrontDisplayChatUser>,
}

impl BinaryEncode for FrontDisplayChatGroup {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        binary_write_i64(&mut encoded, self.gid)?;
        binary_write_string(&mut encoded, self.group_name.as_str())?;
        binary_write_string(&mut encoded, self.group_thumbnail.as_str())?;
        binary_write_i64(&mut encoded, self.uuid)?;
        binary_write_i16(&mut encoded, self.person_count)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayChatGroup {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayChatGroup> {
        let gid = binary_read_i64(cursor)?;
        let group_name = binary_read_string(cursor, bytes)?;
        let group_thumbnail = binary_read_string(cursor, bytes)?;
        let uuid = binary_read_i64(cursor)?;
        let person_count = binary_read_i16(cursor)?;

        let data = FrontDisplayChatGroup {
            gid,
            group_name,
            group_thumbnail,
            uuid,
            person_count,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayChatGroupDetail {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.gid)?;
        binary_write_string(&mut encoded, self.group_name.as_str())?;
        binary_write_string(&mut encoded, self.group_thumbnail.as_str())?;
        binary_write_i16(&mut encoded, self.person_count)?;

        let members = self.members.encode()?;
        encoded.extend(members);
        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayChatGroupDetail {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
    ) -> Result<FrontDisplayChatGroupDetail> {
        let gid = binary_read_i64(cursor)?;
        let group_name = binary_read_string(cursor, bytes)?;
        let group_thumbnail = binary_read_string(cursor, bytes)?;
        let person_count = binary_read_i16(cursor)?;

        let members: Vec<FrontDisplayChatUser> = deserialize_binary(cursor, bytes)?;

        let data = FrontDisplayChatGroupDetail {
            gid,
            group_name,
            group_thumbnail,
            person_count,
            members,
        };
        Ok(data)
    }
}
