use crate::front_models::user_equipments::FrontDisplayUserEquipment;
use crate::get_guid_value;
use crate::models::user_equipments::{NewUserEquipment, UserEquipment};
use crate::schema::user_equipments;
use anyhow::{bail, Result};
use diesel::prelude::*;

impl UserEquipment {
    pub fn get_front_display_user_equipment_list(
        conn: &PgConnection,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<FrontDisplayUserEquipment>> {
        user_equipments::table
            .filter(user_equipments::uid.eq(uid))
            .select((user_equipments::id, user_equipments::eid))
            .limit(limit)
            .offset(offset)
            .load(conn)
    }

    pub fn get_user_equipment_list(
        conn: &PgConnection,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> QueryResult<Vec<Self>> {
        user_equipments::table
            .filter(user_equipments::uid.eq(uid))
            .limit(limit)
            .offset(offset)
            .load(conn)
    }

    pub fn create_user_default_equipments(
        conn: &PgConnection,
        uid: i64,
        eid_list: Vec<i64>,
    ) -> QueryResult<()> {
        let mut user_equipments = Vec::new();
        eid_list.into_iter().for_each(|eid| {
            let user_equipment = NewUserEquipment {
                id: get_guid_value() as i64,
                eid,
                uid,
            };
            user_equipments.push(user_equipment);
        });

        let _usize = diesel::insert_into(user_equipments::table)
            .values(user_equipments)
            .execute(conn)?;

        Ok(())
    }

    pub fn user_buy_equipment(conn: &PgConnection, eid: i64, uid: i64) -> Result<()> {
        if let Ok(exists) = UserEquipment::find_user_equipment_exists(conn, uid, eid) {
            if exists {
                bail!("user {} already have this equipment {}.", uid, eid);
            }
        }

        let user_equipment = NewUserEquipment {
            id: get_guid_value() as i64,
            eid,
            uid,
        };

        diesel::insert_into(user_equipments::table)
            .values(user_equipment)
            .execute(conn)?;

        Ok(())
    }

    pub fn find_user_equipment_exists(
        conn: &PgConnection,
        uid: i64,
        eid: i64,
    ) -> QueryResult<bool> {
        use diesel::dsl::exists;

        let f = user_equipments::table
            .filter(user_equipments::eid.eq(eid))
            .filter(user_equipments::uid.eq(uid));

        diesel::select(exists(f)).get_result(conn)
    }
}
