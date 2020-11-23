use crate::get_guid_value;
use crate::models::user_buy_shop_vip_records::{NewUserBuyShopVipRecords, UserBuyShopVipRecord};
use crate::schema::user_buy_shop_vip_records;
use diesel::prelude::*;

impl UserBuyShopVipRecord {
    pub fn get_user_buy_shop_vip_by_week(
        conn: &PgConnection,
        uuid: i64,
        shop_vip_id: i64,
        week_time: i32,
    ) -> QueryResult<Self> {
        user_buy_shop_vip_records::table
            .filter(user_buy_shop_vip_records::uuid.eq(uuid))
            .filter(user_buy_shop_vip_records::shop_vip_id.eq(shop_vip_id))
            .filter(user_buy_shop_vip_records::week_time.eq(week_time))
            .get_result(conn)
    }

    pub fn update_user_buy_shop_vip_amounts(
        conn: &PgConnection,
        id: i64,
        amounts: i32,
    ) -> QueryResult<()> {
        diesel::update(user_buy_shop_vip_records::table)
            .set(
                user_buy_shop_vip_records::amounts.eq(user_buy_shop_vip_records::amounts + amounts),
            )
            .filter(user_buy_shop_vip_records::id.eq(id))
            .execute(conn)?;

        Ok(())
    }

    pub fn add_user_buy_shop_vip_record(
        conn: &PgConnection,
        uuid: i64,
        shop_vip_id: i64,
        week_time: i32,
    ) -> QueryResult<()> {
        let data = NewUserBuyShopVipRecords {
            id: get_guid_value(),
            uuid,
            shop_vip_id,
            amounts: 1,
            week_time,
        };

        diesel::insert_into(user_buy_shop_vip_records::table)
            .values(data)
            .execute(conn)?;

        Ok(())
    }
}
