use crate::{utils::binary_read_helper::*, BinaryDecode, BinaryEncode};
use anyhow::Result;
use std::io::Cursor;

#[derive(Debug, Clone, Queryable)]
pub struct FrontDisplayUserPlayerTrack {
    pub tid: i64,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
}

impl BinaryEncode for FrontDisplayUserPlayerTrack {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, self.tid)?;
        binary_write_f32(&mut encoded, self.rotation_x)?;
        binary_write_f32(&mut encoded, self.rotation_y)?;
        binary_write_f32(&mut encoded, self.rotation_z)?;
        binary_write_f32(&mut encoded, self.location_x)?;
        binary_write_f32(&mut encoded, self.location_y)?;
        binary_write_f32(&mut encoded, self.location_z)?;

        //set item length
        encoded.encode()
    }
}

impl<'a> BinaryDecode<'a> for FrontDisplayUserPlayerTrack {
    fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        _bytes: &'a [u8],
    ) -> Result<FrontDisplayUserPlayerTrack> {
        let tid = binary_read_i64(cursor)?;
        let rotation_x = binary_read_f32(cursor)?;
        let rotation_y = binary_read_f32(cursor)?;
        let rotation_z = binary_read_f32(cursor)?;
        let location_x = binary_read_f32(cursor)?;
        let location_y = binary_read_f32(cursor)?;
        let location_z = binary_read_f32(cursor)?;

        let data = FrontDisplayUserPlayerTrack {
            tid,
            rotation_x,
            rotation_y,
            rotation_z,
            location_x,
            location_y,
            location_z,
        };
        Ok(data)
    }
}
