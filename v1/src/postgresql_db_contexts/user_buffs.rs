use crate::get_guid_value;
use crate::models::user_buffs::{NewUserBuff, UserBuff};
use crate::schema::user_buffs;
use anyhow::Result;
use diesel::prelude::*;

impl UserBuff {
    #[allow(clippy::too_many_arguments)]
    pub fn add_user_buff(
        conn: &PgConnection,
        obj_id: i64,
        obj_type: i16,
        buff_id: i64,
        buff_amounts: i32,
        buff_category: i32,
        buff_type: i32,
        sub_buff_type: i32,
        buff_source: i32,
        is_show: i16,
    ) -> QueryResult<()> {
        let data = NewUserBuff {
            bid: get_guid_value(),
            obj_id,
            obj_type,
            buff_id,
            buff_amounts,
            buff_category,
            buff_type,
            sub_buff_type,
            buff_source,
            is_show,
        };

        diesel::insert_into(user_buffs::table)
            .values(data)
            .execute(conn)?;
        Ok(())
    }

    pub fn get_buff_amounts_sum(
        conn: &PgConnection,
        buff_category: i32,
        buff_type: i32,
        sub_buff_type: i32,
        buff_source: i32,
    ) -> Result<i32> {
        use diesel::dsl::sum;

        let value: Option<i64> = user_buffs::table
            .filter(user_buffs::buff_category.eq(buff_category))
            .filter(user_buffs::buff_type.eq(buff_type))
            .filter(user_buffs::sub_buff_type.eq(sub_buff_type))
            .filter(user_buffs::buff_source.eq(buff_source))
            .select(sum(user_buffs::buff_amounts))
            .get_result(conn)?;

        let mut res = 0;

        if let Some(v) = value {
            res = v;
        }

        Ok(res as i32)
    }
}
