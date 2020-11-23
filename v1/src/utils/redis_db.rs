use redis::{Connection, RedisResult};
use std::env;

pub fn get_redis_connection() -> RedisResult<Connection> {
    let redis_url = env::var("REDIS_URL").expect("DATABASE_URL must be set");
    let client = redis::Client::open(redis_url).unwrap();

    let conn = client.get_connection();

    conn
}

pub const GLOBAL_USERS_COUNTER_REDIS_KEY: &str = "global_user_counter";
