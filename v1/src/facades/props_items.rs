use super::versions;
use crate::{models::{
    props_action_points_categories::PropsActionPointsCategory,
    props_boost_categories::PropsBoostCategory,
    props_builder_recruitment_categories::PropsBuilderRecruitmentCategory,
    props_fixed_treasure_chest_categories::PropsFixedTreasureChestCategory,
    props_fixed_treasure_chest_category_assets::PropsFixedTreasureChestCategoryAsset,
    props_item_metadatas::PropsItemMetadata,
    props_key_categories::PropsKeyCategory,
    props_product_numbers::PropsProductNumber,
    props_random_treasure_chest_categories::PropsRandomTreasureChestCategory,
    props_random_treasure_chest_category_assets::PropsRandomTreasureChestCategoryAsset,
    props_random_treasure_chest_category_attribute_assets::PropsRandomTreasureChestCategoryAttributeAsset,
    props_resources_categories::PropsResourcesCategory,
    props_speed_up_categories::PropsSpeedUpCategory,
    props_starlight_sculpture_categories::PropsStarlightSculptureCategory,
    props_tome_of_knowledge_categories::PropsTomeOfKnowledgeCategory,
}, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::*;


pub fn get_item_metadata_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsItemMetadata>(conn, version)
}

pub fn get_action_points_props_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsActionPointsCategory>(conn, version)
}

pub fn get_boost_props_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsBoostCategory>(conn, version)
}

pub fn get_builder_recruitment_props_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsBuilderRecruitmentCategory>(conn, version)
}

pub fn get_fixed_treasure_chest_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsFixedTreasureChestCategory>(conn, version)
}

pub fn get_fixed_treasure_chest_asset_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsFixedTreasureChestCategoryAsset>(conn, version)
}

pub fn get_key_props_list(conn: &PgConnection, version: i64) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsKeyCategory>(conn, version)
}

pub fn get_props_product_number_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsProductNumber>(conn, version_id)
}

pub fn get_random_treasure_chest_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsRandomTreasureChestCategory>(conn, version_id)
}

pub fn get_random_treasure_chest_asset_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsRandomTreasureChestCategoryAsset>(conn, version_id)
}

pub fn get_random_treasure_chest_attribute_asset_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsRandomTreasureChestCategoryAttributeAsset>(conn, version_id)
}

pub fn get_resources_props_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsResourcesCategory>(conn, version_id)
}

pub fn get_speed_up_props_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsSpeedUpCategory>(conn, version_id)
}

pub fn get_starlight_sculpture_props_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsStarlightSculptureCategory>(conn, version_id)
}

pub fn get_tome_of_knowledge_props_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsTomeOfKnowledgeCategory>(conn, version_id)
}

