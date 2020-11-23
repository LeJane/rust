use crate::{
    deserialize_binary, utils::binary_read_helper::*, BinaryDecode, BinaryEncode, MetadataInstance,
    MetadataTypeEnum,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrontDisplayVersion {
    pub update_type: i32, //1:Incremental update,2:Full update
    pub latest_version: i64,
    pub table_id_list: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrontDisplayMetaVersion {
    pub update_type: i32, //1:Incremental update,2:Full update
    pub data_list: Vec<FrontDisplayMetaVersionRelation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrontDisplayMetaVersionRelation {
    pub action_type: i32, //0:default,1:insert,2:update,3:del
    pub table_id: i32,
    pub data: MetadataTypeEnum,
}

impl FrontDisplayMetaVersionRelation {
    pub fn get_front_display_meta_version_from_value<T, E>(
        values: Vec<T>,
        value_enum_fn: E,
    ) -> Result<FrontDisplayMetaVersion>
    where
        T: MetadataInstance,
        E: Fn(T) -> MetadataTypeEnum,
    {
        let table_id = T::get_table_id()?;

        let data_list = values
            .into_iter()
            .map(|data| FrontDisplayMetaVersionRelation {
                action_type: 0,
                table_id,
                data: value_enum_fn(data),
            })
            .collect();

        Ok(FrontDisplayMetaVersion {
            update_type: 2,
            data_list,
        })
    }
}

impl BinaryEncode for FrontDisplayMetaVersion {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, self.update_type)?;
        let data_list = self.data_list.encode()?;
        encoded.extend(data_list);

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayMetaVersion {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayMetaVersion> {
        let update_type = binary_read_i32(cursor)?;
        let data_list: Vec<FrontDisplayMetaVersionRelation> = deserialize_binary(cursor, bytes)?;

        let data = FrontDisplayMetaVersion {
            update_type,
            data_list,
        };
        Ok(data)
    }
}

impl BinaryEncode for FrontDisplayMetaVersionRelation {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, self.action_type)?;
        binary_write_i32(&mut encoded, self.table_id)?;

        let data = self.data.encode()?;
        encoded.extend(data);

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayMetaVersionRelation {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
    ) -> Result<FrontDisplayMetaVersionRelation> {
        let action_type = binary_read_i32(cursor)?;
        let table_id = binary_read_i32(cursor)?;

        let data: MetadataTypeEnum =
            MetadataTypeEnum::decode(cursor, bytes, action_type, table_id)?;

        let res = FrontDisplayMetaVersionRelation {
            action_type,
            table_id,
            data,
        };
        Ok(res)
    }
}

impl BinaryEncode for FrontDisplayVersion {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, self.update_type)?;
        //Vec(&mut encoded, self.data_list)?;
        binary_write_i64(&mut encoded, self.latest_version)?;

        let table_id_list = self.table_id_list.encode()?;

        encoded.extend(table_id_list);

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayVersion {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayVersion> {
        let update_type = binary_read_i32(cursor)?;

        let latest_version = binary_read_i64(cursor)?;

        let table_id_list: Vec<i32> = deserialize_binary(cursor, bytes)?;

        let data = FrontDisplayVersion {
            update_type,
            latest_version,
            table_id_list,
        };
        Ok(data)
    }
}
