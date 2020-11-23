use crate::front_models::user_players::FrontDisplayUserPlayer;
use crate::get_guid_value;
use crate::models::{
    players::Player,
    user_players::{NewUserPlayer, UserPlayer},
};
use crate::schema::user_players;
use diesel::prelude::*;

impl UserPlayer {
    pub fn get_front_display_user_default_player_data(
        conn: &PgConnection,
        uid: i64,
    ) -> QueryResult<FrontDisplayUserPlayer> {
        user_players::table
            .filter(user_players::uid.eq(uid))
            .filter(user_players::is_default.eq(2))
            .select((
                user_players::id,
                user_players::pid,
                user_players::max_hp,
                user_players::attack_power,
                user_players::move_speed,
                user_players::max_mana,
                user_players::defense,
                user_players::level,
                user_players::star_level,
                user_players::level_experience,
            ))
            .get_result(conn)
    }

    pub fn get_front_display_user_player_data(
        conn: &PgConnection,
        uid: i64,
        pid: i64,
    ) -> QueryResult<FrontDisplayUserPlayer> {
        user_players::table
            .filter(user_players::uid.eq(uid))
            .filter(user_players::pid.eq(pid))
            .select((
                user_players::id,
                user_players::pid,
                user_players::max_hp,
                user_players::attack_power,
                user_players::move_speed,
                user_players::max_mana,
                user_players::defense,
                user_players::level,
                user_players::star_level,
                user_players::level_experience,
            ))
            .get_result(conn)
    }

    pub fn create_user_default_player(
        conn: &PgConnection,
        uid: i64,
        player: Player,
    ) -> QueryResult<()> {
        let user_player = NewUserPlayer {
            id: get_guid_value() as i64,
            pid: player.pid,
            uid,
            max_hp: player.max_hp,
            attack_power: player.attack_power,
            move_speed: player.move_speed,
            max_mana: player.max_mana,
            defense: player.defense,
            level: player.level,
            star_level: player.star_level,
            level_experience: player.level_experience,
            is_default: 2,
        };

        let _usize = diesel::insert_into(user_players::table)
            .values(user_player)
            .execute(conn)?;

        Ok(())
    }
}
