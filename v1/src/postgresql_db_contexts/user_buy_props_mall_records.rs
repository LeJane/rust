use crate::models::user_buy_props_mall_records::UserBuyPropsMallRecord;
use crate::schema::user_buy_props_mall_records;
use chrono::Utc;
use diesel::prelude::*;
use std::collections::HashMap;

impl UserBuyPropsMallRecord {
    pub fn exist_user_buy_props_mall_item(
        conn: &PgConnection,
        uuid: i64,
        item_id: i64,
        mall_type: i32,
        is_check_expire_time: bool,
    ) -> QueryResult<bool> {
        use diesel::dsl::exists;
        let t = Utc::now();
        let mut f = user_buy_props_mall_records::table
            .filter(user_buy_props_mall_records::uuid.eq(uuid))
            .filter(user_buy_props_mall_records::item_id.eq(item_id))
            .filter(user_buy_props_mall_records::mall_type.eq(mall_type))
            .into_boxed();

        if is_check_expire_time {
            f = f.filter(user_buy_props_mall_records::expire_time.gt(t.timestamp()));
        }

        diesel::select(exists(f)).get_result(conn)
    }

    pub fn get_super_value_bundle_category_levels(
        conn: &PgConnection,
        uuid: i64,
    ) -> QueryResult<HashMap<i32, i64>> {
        use diesel::dsl::sql;
        let t = Utc::now();

        let item_category_levels = user_buy_props_mall_records::table
            .filter(user_buy_props_mall_records::uuid.eq(uuid))
            .filter(user_buy_props_mall_records::expire_time.gt(t.timestamp()))
            .filter(user_buy_props_mall_records::mall_type.eq(1081001))
            .select((
                user_buy_props_mall_records::item_category,
                sql::<diesel::sql_types::BigInt>("max(level) as level"),
            ))
            .group_by(user_buy_props_mall_records::item_category)
            .load::<(i32, i64)>(conn)?
            .into_iter()
            .collect::<HashMap<i32, i64>>();

        Ok(item_category_levels)
    }


    pub fn get_super_value_bundle_purchase_limit(
        conn: &PgConnection,
        uuid: i64,
        item_id: i64,
        level: i64,
    ) -> QueryResult<i16> {
        let t = Utc::now();

        user_buy_props_mall_records::table
            .filter(user_buy_props_mall_records::uuid.eq(uuid))
            .filter(user_buy_props_mall_records::item_id.gt(item_id))
            .filter(user_buy_props_mall_records::level.gt(level))
            .filter(user_buy_props_mall_records::expire_time.gt(t.timestamp()))
            .filter(user_buy_props_mall_records::mall_type.eq(1081001))
            .select(
                user_buy_props_mall_records::purchase_limit,
            )
            .first(conn)
    }
}
