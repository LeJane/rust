use crate::models::versions::Version;
use crate::schema::versions;
use diesel::prelude::*;

impl Version {
    pub fn get_version_by_last_version(
        conn: &PgConnection,
        last_version_id: i64,
    ) -> QueryResult<Self> {
        versions::table
            .filter(versions::last_version_id.eq(last_version_id))
            .get_result(conn)
    }

    pub fn get_max_version(conn: &PgConnection) -> QueryResult<Self> {
        versions::table
            .order_by(versions::version_id.desc())
            .get_result(conn)
    }
}
