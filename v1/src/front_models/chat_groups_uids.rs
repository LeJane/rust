use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

#[derive(Debug, Clone, Queryable)]
pub struct FrontDisplayChatGroupsUid {
    pub gid: i64,
    pub group_name: String,
    pub group_thumbnail: String,
}

impl BinaryEncode for FrontDisplayChatGroupsUid {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.gid)?;
        binary_write_string(&mut encoded, self.group_name.as_str())?;
        binary_write_string(&mut encoded, self.group_thumbnail.as_str())?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayChatGroupsUid {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayChatGroupsUid> {
        let gid = binary_read_i64(cursor)?;
        let group_name = binary_read_string(cursor, bytes)?;
        let group_thumbnail = binary_read_string(cursor, bytes)?;

        let data = FrontDisplayChatGroupsUid {
            gid,
            group_name,
            group_thumbnail,
        };
        Ok(data)
    }
}
