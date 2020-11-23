use crate::front_models::versions::{
    FrontDisplayMetaVersion, FrontDisplayMetaVersionRelation, FrontDisplayVersion,
};
use crate::models::{
    version_meta_operation_relations::VersionMetaOperationRelation,
    version_meta_relations::VersionMetaRelation, versions::Version,
};
use crate::{MetadataInstance, MetadataTypeEnum};
use anyhow::{anyhow, Result};
use diesel::prelude::*;

pub fn get_meta_version_relation_by_version(
    conn: &PgConnection,
    version: i64,
) -> Result<FrontDisplayVersion> {
    let max_version = Version::get_max_version(conn)?;

    if version == 0 {
        return Ok(FrontDisplayVersion {
            update_type: 2,
            latest_version: max_version.version_id,
            table_id_list: vec![],
        });
    }

    let last_version_id = match Version::get_version_by_last_version(conn, version) {
        Ok(v) => v.version_id,
        Err(_) => 0,
    };


    if last_version_id != max_version.version_id {
        //full update
        Ok(FrontDisplayVersion {
            update_type: 2,
            latest_version: max_version.version_id,
            table_id_list: vec![],
        })
    } else {
        //incr update

        let table_id_list =
            VersionMetaRelation::get_version_meta_relation_table_id_by_version(conn, version)?;

        Ok(FrontDisplayVersion {
            update_type: 1,
            latest_version: max_version.version_id,
            table_id_list,
        })
    }
}

pub fn get_meta_version_relation_type_data<T: MetadataInstance>(
    conn: &PgConnection,
    version_id: i64,
) -> Result<FrontDisplayMetaVersion> {
    let version_meta_relation = VersionMetaRelation::get_version_meta_relation_by_version_id(
        conn,
        version_id,
        T::get_table_id()?,
    ).map_err(|e| anyhow!("failed get version {} meta relation:{}",version_id,e))?;

    if version_meta_relation.update_type == 2 {
        T::get_instance_list(conn).map_err(|e| anyhow!("failed get type metadata instance list:{}",e))
    } else {
        let version_relation_operation_list =
            VersionMetaOperationRelation::get_version_meta_relation_operation_list(
                conn,
                version_meta_relation.operation_id,
            ).map_err(|e| anyhow!("failed get version meta relation operation list:{}",e))?;

        let mut data_list = Vec::with_capacity(version_relation_operation_list.len());

        for version_relation_operation in version_relation_operation_list.into_iter() {
            match version_relation_operation.action_type {
                1 | 2 => {
                    let enum_value =
                        T::get_single_instance(conn, version_relation_operation.action_id).map_err(|e| anyhow!("failed get single type metadata instance:{}",e))?;

                    let d = FrontDisplayMetaVersionRelation {
                        action_type: version_relation_operation.action_type,
                        table_id: T::get_table_id()?,
                        data: enum_value,
                    };

                    data_list.push(d);
                }
                3 => {
                    let d = FrontDisplayMetaVersionRelation {
                        action_type: version_relation_operation.action_type,
                        table_id: T::get_table_id()?,
                        data: MetadataTypeEnum::I64(version_relation_operation.action_id),
                    };
                    data_list.push(d);
                }
                _ => continue,
            }
        }

        Ok(FrontDisplayMetaVersion {
            update_type: 1,
            data_list,
        })
    }
}
