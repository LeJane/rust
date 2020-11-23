use crate::facades::user_item_bags;
use crate::models::{
    props_fixed_treasure_chest_categories::PropsFixedTreasureChestCategory,
    props_fixed_treasure_chest_category_assets::PropsFixedTreasureChestCategoryAsset,
    props_item_metadatas::PropsItemMetadata,
};
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub fn get_treasure_chest_data_and_into_item_bags(
    conn: &PgConnection,
    uuid: i64,
    pfc_id: i64,
) -> Result<()> {
    let treasure_chest_data =
        PropsFixedTreasureChestCategory::get_fixed_treasure_chest_data_by_id(conn, pfc_id)
            .map_err(|e| anyhow!("failed fixed treasure chest data:{}", e))?;

    let item_metadata =
        PropsItemMetadata::get_item_metadata_by_id(conn, treasure_chest_data.item_id)
            .map_err(|e| anyhow!("failed item meta data:{}", e))?;

    //can't instantly open
    if treasure_chest_data.is_instantly_open == 1 {
        //into bag
        //can't overlay
        user_item_bags::check_item_bag_exists_item_and_update(conn, &item_metadata, uuid)?;
    } else {
        //item into bag

        let assets = PropsFixedTreasureChestCategoryAsset::get_treasure_chest_asset_list_by_item_id(
            conn,
            item_metadata.item_id,
        )
            .map_err(|e| anyhow!("failed get vip treasure chest data assets:{}", e))?;

        for asset in assets.into_iter() {
            let item_data = PropsItemMetadata::get_item_metadata_by_id(conn, asset.item_id)
                .map_err(|e| anyhow!("failed get item metadata:{}", e))?;

            //can't overlay
            user_item_bags::check_item_bag_exists_item_and_update(conn, &item_data, uuid)?;
        }
    }

    Ok(())
}
