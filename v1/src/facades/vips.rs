use super::versions;
use crate::{models::{
    vip_buffs::VipBuff,
    vip_levels::VipLevel,
    vip_daily_login_treasure_chests::VipDailyLoginTreasureChest,
}, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_vip_buff_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<VipBuff>(conn, version_id)
}

pub fn get_daily_login_treasure_chest_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<VipDailyLoginTreasureChest>(conn, version_id)
}

pub fn get_vip_level_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<VipLevel>(conn, version_id)
}
