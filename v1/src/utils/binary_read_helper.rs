use crate::get_naive_date_time_from_str;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use chrono::NaiveDateTime;
use std::io::Cursor;
use std::str::from_utf8;

pub fn binary_read_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64> {
    let v = cursor.read_i64::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_i32(cursor: &mut Cursor<&[u8]>) -> Result<i32> {
    let v = cursor.read_i32::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_i16(cursor: &mut Cursor<&[u8]>) -> Result<i16> {
    let v = cursor.read_i16::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_i8(cursor: &mut Cursor<&[u8]>) -> Result<i8> {
    let v = cursor.read_i8()?;
    Ok(v)
}

pub fn binary_read_f32(cursor: &mut Cursor<&[u8]>) -> Result<f32> {
    let v = cursor.read_f32::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_u64(cursor: &mut Cursor<&[u8]>) -> Result<u64> {
    let v = cursor.read_u64::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32> {
    let v = cursor.read_u32::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16> {
    let v = cursor.read_u16::<LittleEndian>()?;
    Ok(v)
}

pub fn binary_read_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8> {
    let v = cursor.read_u8()?;
    Ok(v)
}

pub fn binary_read_string(cursor: &mut Cursor<&[u8]>, bytes: &[u8]) -> Result<String> {
    let length = cursor.read_i32::<LittleEndian>()?;
    let length = cursor.position() + length as u64;
    let data = &bytes[cursor.position() as usize..length as usize];
    cursor.set_position(length);

    let v = from_utf8(data)?.to_string();
    Ok(v)
}

pub fn binary_read_time(cursor: &mut Cursor<&[u8]>, bytes: &[u8]) -> Result<NaiveDateTime> {
    let length = cursor.read_i32::<LittleEndian>()?;
    let length = cursor.position() + length as u64;
    let data = &bytes[cursor.position() as usize..length as usize];
    cursor.set_position(length);

    let v = from_utf8(data)?.to_string();

    get_naive_date_time_from_str(&v)
}

pub fn binary_read_msg(cursor: &mut Cursor<&[u8]>, bytes: &[u8]) {
    let msg_length = binary_read_i32(cursor).expect("msg length error");
    let msg_length = cursor.position() + msg_length as u64;
    let _msg = &bytes[cursor.position() as usize..msg_length as usize];
    cursor.set_position(msg_length);
}

pub fn binary_write_i64(encoded: &mut Vec<u8>, v: i64) -> Result<()> {
    let v = encoded.write_i64::<LittleEndian>(v)?;
    Ok(v)
}
pub fn binary_write_u64(encoded: &mut Vec<u8>, v: u64) -> Result<()> {
    let v = encoded.write_u64::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_i32(encoded: &mut Vec<u8>, v: i32) -> Result<()> {
    let v = encoded.write_i32::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_u32(encoded: &mut Vec<u8>, v: u32) -> Result<()> {
    let v = encoded.write_u32::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_f32(encoded: &mut Vec<u8>, v: f32) -> Result<()> {
    let v = encoded.write_f32::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_i16(encoded: &mut Vec<u8>, v: i16) -> Result<()> {
    let v = encoded.write_i16::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_i8(encoded: &mut Vec<u8>, v: i8) -> Result<()> {
    let v = encoded.write_i8(v)?;
    Ok(v)
}

pub fn binary_write_u16(encoded: &mut Vec<u8>, v: u16) -> Result<()> {
    let v = encoded.write_u16::<LittleEndian>(v)?;
    Ok(v)
}

pub fn binary_write_u8(encoded: &mut Vec<u8>, v: u8) -> Result<()> {
    let v = encoded.write_u8(v)?;
    Ok(v)
}

pub fn binary_write_string(encoded: &mut Vec<u8>, v: &str) -> Result<()> {
    let bytes = v.as_bytes();
    encoded.write_i32::<LittleEndian>(bytes.len() as i32)?;
    encoded.extend_from_slice(bytes);
    Ok(())
}

pub fn binary_write_time(encoded: &mut Vec<u8>, v: NaiveDateTime) -> Result<()> {
    binary_write_string(encoded, v.format("%Y.%m.%d-%H.%M.%S").to_string().as_str())
}
