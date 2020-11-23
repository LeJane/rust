use diesel::prelude::*;
use v1::schema::enemys;
use v1::schema::{
    equipment_kinds, equipments, gem_relateds, gems, players, skill_fight_relateds, skills,
};

use v1::utils::diesel_db::get_diesel_pool;

mod data;

use data::default_data::get_default_data;

// cargo test -- --nocapture -Z unstable-options --format=json --test insert_default_data
#[test]
fn insert_default_data() {
    println!("Make this test fail");
    // return;
    let conn = get_diesel_pool().get().unwrap();

    let (skills, enemys, gems, equipment_kinds, equipments, players, skills_relateds, gem_relateds) =
        get_default_data();

    diesel::insert_into(skills::table)
        .values(skills)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(enemys::table)
        .values(enemys)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(gems::table)
        .values(gems)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(equipment_kinds::table)
        .values(equipment_kinds)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(equipments::table)
        .values(equipments)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(players::table)
        .values(players)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(skill_fight_relateds::table)
        .values(skills_relateds)
        .execute(&conn)
        .unwrap();

    diesel::insert_into(gem_relateds::table)
        .values(gem_relateds)
        .execute(&conn)
        .unwrap();
}
