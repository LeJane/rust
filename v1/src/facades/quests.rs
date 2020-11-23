use super::versions;
use crate::{models::{
    quests_assets::QuestsAsset,
    quests_attribute_assets::QuestsAttributeAsset,
    quests_metadatas::QuestsMetadata,
    quests_relations::QuestsRelation,
}, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;




pub fn get_quest_asset_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<QuestsAsset>(conn, version_id)
}


pub fn get_quest_attribute_asset_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<QuestsAttributeAsset>(conn, version_id)
}

pub fn get_quests_metadata_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<QuestsMetadata>(conn, version_id)
}

pub fn get_quests_relation_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<QuestsRelation>(conn, version_id)
}
