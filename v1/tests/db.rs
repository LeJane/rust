//postgresql benchmark;
use diesel::prelude::*;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use v1::get_guid_value;
use v1::models::user_link_accounts::{NewUserLinkAccount, UserLinkAccount};
use v1::schema::user_link_accounts;
use v1::utils::diesel_db::{get_diesel_pool, DieselPool};

#[test]
fn exec_insert_thread() {
    println!("exec_insert_thread start");
    let diesel_pool = get_diesel_pool();
    //insert

    let mut childer = Vec::new();
    let insert_time_list = Arc::new(Mutex::new(Vec::with_capacity(100000)));
    let now = Instant::now();
    for _t in 0..3 {
        let conn_clone = diesel_pool.clone();
        let insert_time_list_clone = insert_time_list.clone();
        let join_thread = thread::spawn(move || {
            let time_list = insert_latest_data(conn_clone);

            let mut insert_time_list_update = insert_time_list_clone.lock().unwrap();
            insert_time_list_update.extend(time_list);
        });
        childer.push(join_thread);
    }

    for c in childer {
        c.join().unwrap();
    }
    let mut insert_time_list = insert_time_list.lock().unwrap();
    insert_time_list.sort();
    println!(
        "exec_insert_thread->time_list sort:{:?}\n",
        insert_time_list
    );

    println!(
        "exec_insert_thread->time_list count:{:?}\n",
        insert_time_list.len()
    );
    println!("insert total end time:{}\n", now.elapsed().as_secs());
}
#[test]
fn exec_select_thread() {
    println!("exec_select_thread start");

    let diesel_pool = get_diesel_pool();
    //select
    let count = 31000;

    let mut time_list = Vec::with_capacity(count as usize);
    for _x in 0..count {
        let conn_clone = diesel_pool.clone();

        let now = Instant::now();
        read_latest_data(conn_clone);
        let secs = now.elapsed().as_millis();
        time_list.push(secs);
        println!("_x:{},select end time:{}\n", _x, secs);
    }
    time_list.sort();
    println!("exec_select_thread->time_list sort:{:?}\n", time_list);
}

fn read_latest_data(conn: Arc<DieselPool>) {
    let conn = conn.get().unwrap();
    let data: UserLinkAccount = user_link_accounts::table
        .filter(user_link_accounts::account_type.eq(1))
        .order_by(user_link_accounts::created_time.desc())
        .get_result(&conn)
        .unwrap();

    println!("data:{:?}\n", data);
}

//insert
fn insert_latest_data(conn: Arc<DieselPool>) -> Vec<u128> {
    let count = 31000;
    let mut time_list = Vec::with_capacity(count as usize);

    for _c in 0..count {
        let mut rng = rand::thread_rng();
        let rand_value: u64 = rng.gen_range(1, 3);
        let data = NewUserLinkAccount {
            lid: get_guid_value() as i64,
            uuid: get_guid_value() as i64,
            account_type: rand_value as i16,
        };
        let now = Instant::now();
        let conn = conn.get().unwrap();
        diesel::insert_into(user_link_accounts::table)
            .values(data)
            .execute(&conn)
            .unwrap();
        let secs = now.elapsed().as_millis();
        time_list.push(secs);
        println!("_c:{},insert end time:{}", _c, secs);
    }
    time_list.sort();
    println!("insert_latest_data->time_list sort:{:?}\n", time_list);
    time_list
}
