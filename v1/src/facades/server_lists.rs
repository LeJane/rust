use crate::front_models::server_lists::FrontDisplayServerTime;
use crate::{facades::versions, models::server_lists::ServerList, FrontDisplayMetaVersion};
use anyhow::Result;
use chrono::Utc;
use diesel::PgConnection;


pub fn get_server_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<ServerList>(conn, version)
}

pub fn get_server_utc_time() -> Result<FrontDisplayServerTime> {
    let t = Utc::now();

    let d = FrontDisplayServerTime {
        time: t.timestamp(),
    };

    Ok(d)
}
