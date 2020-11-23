use super::versions;
use crate::{models::props_product_numbers::PropsProductNumber, FrontDisplayMetaVersion};
use anyhow::Result;
use diesel::prelude::PgConnection;

pub fn get_props_product_number_list(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    versions::get_meta_version_relation_type_data::<PropsProductNumber>(conn, version_id)
}
