use crate::front_models::user_item_bags::FrontDisplayUserItemBag;
use crate::models::{props_item_metadatas::PropsItemMetadata, user_item_bags::UserItemBag};
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub fn get_front_display_user_item_bag_list(
    conn: &PgConnection,
    uid: i64,
) -> Result<Vec<FrontDisplayUserItemBag>> {
    UserItemBag::get_front_display_user_item_bag_list(conn, uid)
}

pub fn check_item_bag_exists_item_and_update(
    conn: &PgConnection,
    item_metadata: &PropsItemMetadata,
    uuid: i64,
) -> Result<()> {
    //can't overlay
    if item_metadata.overlay_status == 2 {
        //add
        UserItemBag::add_user_item_bag_data(
            conn,
            uuid,
            item_metadata.item_id,
            item_metadata.overlay_status,
            item_metadata.bag_type,
            1,
            (item_metadata.bag_type * (item_metadata.item_id as i32) * item_metadata.rarity_type)
                as i64,
            item_metadata.sub_item_type,
        )?;
    } else {
        let mut exists = false;

        if let Ok(v) = UserItemBag::exist_user_item_bag_by_item_id(conn, item_metadata.item_id) {
            exists = v;
        }

        if exists {
            let item_bag_data =
                UserItemBag::get_user_item_bag_by_item_id(conn, item_metadata.item_id)
                    .map_err(|e| anyhow!("failed get item bag data:{}", e))?;

            UserItemBag::update_user_item_bag_count_by_bid(conn, item_bag_data.bid, 1)?;
        } else {
            UserItemBag::add_user_item_bag_data(
                conn,
                uuid,
                item_metadata.item_id,
                item_metadata.overlay_status,
                item_metadata.bag_type,
                1,
                (item_metadata.bag_type
                    * (item_metadata.item_id as i32)
                    * item_metadata.rarity_type) as i64,
                item_metadata.sub_item_type,
            )?;
        }
    }

    Ok(())
}
