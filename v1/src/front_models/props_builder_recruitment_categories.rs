use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct FrontDisplayPropsBuilderRecruitmentCategory {
    pub item_id: i64,
    pub recruit_time: i64,
    pub buff_id: i64,
}

impl BinaryEncode for FrontDisplayPropsBuilderRecruitmentCategory {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.item_id)?;
        binary_write_i64(&mut encoded, self.recruit_time)?;
        binary_write_i64(&mut encoded, self.buff_id)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayPropsBuilderRecruitmentCategory {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayPropsBuilderRecruitmentCategory> {
        let item_id = binary_read_i64(cursor)?;
        let recruit_time = binary_read_i64(cursor)?;
        let buff_id = binary_read_i64(cursor)?;

        let data = FrontDisplayPropsBuilderRecruitmentCategory {
            item_id,
            recruit_time,
            buff_id,
        };
        Ok(data)
    }
}
