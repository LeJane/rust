use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct FrontDisplayGem {
    pub gem_icon: String,
    pub gem_selected_material: String,
    pub gem_link_material: String,
    pub model_path: String,
    pub kind: i16,
}

impl BinaryEncode for FrontDisplayGem {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_string(&mut encoded, self.gem_icon.as_str())?;
        binary_write_string(&mut encoded, self.gem_selected_material.as_str())?;
        binary_write_string(&mut encoded, self.gem_link_material.as_str())?;
        binary_write_string(&mut encoded, self.model_path.as_str())?;
        binary_write_i16(&mut encoded, self.kind)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayGem {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<FrontDisplayGem> {
        let gem_icon = binary_read_string(cursor, bytes)?;
        let gem_selected_material = binary_read_string(cursor, bytes)?;
        let gem_link_material = binary_read_string(cursor, bytes)?;

        let model_path = binary_read_string(cursor, bytes)?;
        let kind = binary_read_i16(cursor)?;

        let gem = FrontDisplayGem {
            gem_icon,
            gem_selected_material,
            gem_link_material,
            model_path,
            kind,
        };
        Ok(gem)
    }
}
