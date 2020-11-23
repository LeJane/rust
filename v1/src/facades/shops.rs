use crate::facades::{user_item_bags, versions};
use crate::front_models::versions::FrontDisplayMetaVersion;
use crate::models::{
    props_item_metadatas::PropsItemMetadata, shops::Shop, user_assets::UserAsset,
    user_item_bags::UserItemBag,
};
use anyhow::{anyhow, Error, Result};
use diesel::prelude::*;

pub fn get_shop_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<Shop>(conn, version)
}

pub fn buy_shop_item_by_gems(conn: &PgConnection, uid: i64, sid: i64) -> Result<()> {
    //gems
    // check user gems enough
    let user_asset = UserAsset::get_user_assets(conn, uid)?;

    let shop_data = Shop::get_shop_by_id(conn, sid)?;

    if user_asset.gem_amounts < shop_data.gems_needed {
        return Err(anyhow!("user gems asset not enough"));
    }

    let item_metadata = PropsItemMetadata::get_item_metadata_by_id(conn, shop_data.item_id)
        .map_err(|e| anyhow!("failed item meta data:{}", e))?;

    conn.transaction::<(), Error, _>(|| {
        //can't overlay
        if item_metadata.overlay_status == 2 {
            //add
            UserItemBag::add_user_item_bag_data(
                conn,
                uid,
                item_metadata.item_id,
                item_metadata.overlay_status,
                item_metadata.bag_type,
                1,
                (item_metadata.bag_type
                    * (item_metadata.item_id as i32)
                    * item_metadata.rarity_type) as i64,
                item_metadata.sub_item_type,
            )?;
        } else {
            user_item_bags::check_item_bag_exists_item_and_update(conn, &item_metadata, uid)?;
        }

        UserAsset::update_user_asset_count(conn, uid, 0, -shop_data.gems_needed, 0, 0, 0)
            .map_err(|e| anyhow!("failed update user gem amounts:{}", e))?;
        Ok(())
    })
    .map_err(|e| anyhow!("{}", e))
}
