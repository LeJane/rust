use crate::front_models::users::FrontDisplayChatUser;
use crate::{deserialize_binary, utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct FrontDisplayBlacklist {
    pub bid: i64,
    pub user: FrontDisplayChatUser,
}

impl BinaryEncode for FrontDisplayBlacklist {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.bid)?;
        let user = self.user.encode()?;

        encoded.extend(user);

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayBlacklist {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayBlacklist> {
        let bid = binary_read_i64(cursor)?;
        let user: FrontDisplayChatUser = deserialize_binary(cursor, bytes)?;

        let data = FrontDisplayBlacklist { bid, user };
        Ok(data)
    }
}
