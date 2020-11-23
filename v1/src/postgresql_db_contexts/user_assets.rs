use crate::front_models::user_assets::FrontDisplayUserAsset;
use crate::get_guid_value;
use crate::models::user_assets::{NewUserAsset, UserAsset};
use crate::schema::user_assets;
use diesel::prelude::*;

impl UserAsset {
    pub fn create_user_assets(conn: &PgConnection, uid: i64) -> QueryResult<()> {
        let asset = NewUserAsset {
            asid: get_guid_value(),
            uid,
            gold_amounts: 0,
            gem_amounts: 0,
            food_amounts: 0,
            wood_amounts: 0,
            stone_amounts: 0,
        };

        let _usize = diesel::insert_into(user_assets::table)
            .values(asset)
            .execute(conn)?;

        Ok(())
    }

    pub fn get_front_display_user_assets(
        conn: &PgConnection,
        uid: i64,
    ) -> QueryResult<FrontDisplayUserAsset> {
        user_assets::table
            .filter(user_assets::uid.eq(uid))
            .select((
                user_assets::uid,
                user_assets::gold_amounts,
                user_assets::gem_amounts,
                user_assets::food_amounts,
                user_assets::wood_amounts,
                user_assets::stone_amounts,
            ))
            .first(conn)
    }

    pub fn get_user_assets(conn: &PgConnection, uid: i64) -> QueryResult<Self> {
        user_assets::table
            .filter(user_assets::uid.eq(uid))
            .first(conn)
    }

    pub fn update_user_asset_count(
        conn: &PgConnection,
        uid: i64,
        gold_amounts: i32,
        gem_amounts: i32,
        food_amounts: i32,
        wood_amounts: i32,
        stone_amounts: i32,
    ) -> QueryResult<()> {
        diesel::update(user_assets::table)
            .set((
                user_assets::gold_amounts.eq(user_assets::gold_amounts + gold_amounts),
                user_assets::gem_amounts.eq(user_assets::gem_amounts + gem_amounts),
                user_assets::food_amounts.eq(user_assets::food_amounts + food_amounts),
                user_assets::wood_amounts.eq(user_assets::wood_amounts + wood_amounts),
                user_assets::stone_amounts.eq(user_assets::stone_amounts + stone_amounts),
            ))
            .filter(user_assets::uid.eq(uid))
            .execute(conn)?;

        Ok(())
    }
}
