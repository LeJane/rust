use crate::get_guid_value;
use crate::models::system_configs::{NewSystemConfig, SystemConfig};
use crate::schema::system_configs;
use diesel::expression::exists::exists;
use diesel::prelude::*;

impl SystemConfig {
    pub fn add_system_config(conn: &PgConnection, key: &str, value: &str) -> QueryResult<Self> {
        let d = NewSystemConfig {
            scid: get_guid_value(),
            key,
            value,
        };

        diesel::insert_into(system_configs::table)
            .values(d)
            .get_result(conn)
    }
    pub fn get_system_config(conn: &PgConnection, key: &str) -> QueryResult<Self> {
        system_configs::table
            .filter(system_configs::key.eq(key))
            .get_result(conn)
    }

    pub fn exists_system_config(conn: &PgConnection, key: &str) -> QueryResult<bool> {
        let f = system_configs::table.filter(system_configs::key.eq(key));

        diesel::select(exists(f)).get_result(conn)
    }
}
