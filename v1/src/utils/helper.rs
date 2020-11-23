use anyhow::Result;
use chrono::NaiveDateTime;
use chrono::{Datelike, Utc};
use guid_create::GUID;
use murmur3::murmur3_x86_128;
use std::io::Cursor;

pub fn get_guid_value() -> i64 {
    let _u64_max_value: u64 = 18446744073709551615;
    let guid = GUID::rand();
    let guid = guid.to_string();
    let guid_str = guid.replace("-", "");
    let hash_result = murmur3_x86_128(&mut Cursor::new(guid_str), 0).unwrap();
    let uuid = (hash_result >> 65) as u64;

    uuid as i64
}

//get iso week
pub fn get_iso_week() -> i32 {
    let t = Utc::now();

    let week = t.iso_week().week() as i32;

    t.year() * 100 + week
}

pub fn get_naive_date_time_from_str(time_str: &str) -> Result<NaiveDateTime> {
    let t = NaiveDateTime::parse_from_str(time_str, "%Y.%m.%d-%H.%M.%S%.f")?;
    Ok(t)
}
