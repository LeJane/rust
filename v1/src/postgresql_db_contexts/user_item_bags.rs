use crate::front_models::user_item_bags::FrontDisplayUserItemBag;
use crate::get_guid_value;
use crate::models::user_item_bags::{NewUserItemBag, UserItemBag};
use crate::schema::user_item_bags;
use anyhow::Result;
use chrono::Utc;
use diesel::prelude::*;

impl UserItemBag {
    #[allow(clippy::too_many_arguments)]
    pub fn add_user_item_bag_data(
        conn: &PgConnection,
        uuid: i64,
        item_id: i64,
        overlay_status: i16,
        bag_type: i32,
        count: i32,
        order_value: i64,
        sub_item_type: i32,
    ) -> QueryResult<()> {
        let data = NewUserItemBag {
            bid: get_guid_value(),
            uuid,
            item_id,
            overlay_status,
            bag_type,
            count,
            order_value,
            sub_item_type,
        };

        diesel::insert_into(user_item_bags::table)
            .values(data)
            .execute(conn)?;

        Ok(())
    }

    pub fn update_user_item_bag_count_by_bid(
        conn: &PgConnection,
        bid: i64,
        count: i32,
    ) -> QueryResult<()> {
        let t = Utc::now();
        diesel::update(user_item_bags::table)
            .set((
                user_item_bags::count.eq(user_item_bags::count + count),
                user_item_bags::modify_time.eq(t.naive_utc()),
            ))
            .filter(user_item_bags::bid.eq(bid))
            .execute(conn)?;

        Ok(())
    }

    pub fn exist_user_item_bag_by_item_id(conn: &PgConnection, item_id: i64) -> QueryResult<bool> {
        use diesel::dsl::exists;

        let f = user_item_bags::table.filter(user_item_bags::item_id.eq(item_id));

        diesel::select(exists(f)).get_result(conn)
    }

    pub fn get_user_item_bag_by_id(conn: &PgConnection, bid: i64) -> QueryResult<Self> {
        user_item_bags::table
            .filter(user_item_bags::bid.eq(bid))
            .get_result(conn)
    }

    pub fn delete_user_item_bag_by_id(conn: &PgConnection, bid: i64) -> QueryResult<Self> {
        diesel::delete(user_item_bags::table)
            .filter(user_item_bags::bid.eq(bid))
            .get_result(conn)
    }

    pub fn get_user_item_bag_by_item_id(conn: &PgConnection, item_id: i64) -> QueryResult<Self> {
        user_item_bags::table
            .filter(user_item_bags::item_id.eq(item_id))
            .get_result(conn)
    }

    pub fn get_user_vip_points_item_bag_list(conn: &PgConnection, uuid: i64) -> Result<Vec<Self>> {
        let data = user_item_bags::table
            .filter(user_item_bags::uuid.eq(uuid))
            .filter(user_item_bags::sub_item_type.eq(303))
            .order(user_item_bags::order_value.asc())
            .get_results(conn)?;

        Ok(data)
    }

    pub fn get_user_item_bag_list(conn: &PgConnection, uuid: i64) -> QueryResult<Vec<Self>> {
        user_item_bags::table
            .filter(user_item_bags::uuid.eq(uuid))
            .order(user_item_bags::order_value.asc())
            .get_results(conn)
    }

    pub fn get_front_display_user_item_bag_list(
        conn: &PgConnection,
        uuid: i64,
    ) -> Result<Vec<FrontDisplayUserItemBag>> {
        let data = user_item_bags::table
            .filter(user_item_bags::uuid.eq(uuid))
            .select((
                user_item_bags::bid,
                user_item_bags::item_id,
                user_item_bags::count,
                user_item_bags::order_value,
            ))
            .order(user_item_bags::order_value.asc())
            .get_results(conn)?;

        Ok(data)
    }
}
