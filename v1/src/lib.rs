#[macro_use]
extern crate diesel;

pub mod controller;
pub mod facades;
pub mod front_models;
pub mod models;
pub mod postgresql_db_contexts;
pub mod router;
pub mod schema;
pub mod utils;

pub use router::build_routers;

pub use utils::router::{ReqContext, ResponseContext, ResponseResult, RouterRegister};
pub use utils::{
    common::deserialize_binary,
    common::BinaryDecode,
    common::BinaryEncode,
    common::MetadataInstance,
    common::MetadataTypeEnum,
    common::PageAndId,
    common::TableIdEnum,
    diesel_db::{get_master_diesel_pool, get_slave_diesel_pool, DbConnPool, DieselPool},
    helper::{get_guid_value, get_iso_week, get_naive_date_time_from_str},
    redis_db::get_redis_connection,
    redis_db::GLOBAL_USERS_COUNTER_REDIS_KEY,
    thread_pool::ThreadPool,
};

pub use front_models::versions::{FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation};

#[macro_export]
macro_rules! default_log_pre {
    ($code:expr,$user:expr) => {{
        format!(
            "code:{}\tuser:{}\tmethod:{}\tline:{:?}",
            $code,
            $user,
            function_name!(),
            line!()
        )
    }};
}
