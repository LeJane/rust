CREATE table users
(
    uuid BIGINT PRIMARY KEY,
    uid INTEGER NOT NULL,
    name VARCHAR(500) NOT NULL,
    avatar VARCHAR(500) NOT NULL,
    login_days INTEGER NOT NULL,
    server_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now(),
    action_points INTEGER NOT NULL DEFAULT 1000,
    max_action_points INTEGER NOT NULL DEFAULT 1000,
    action_points_latest_timestamp INTEGER NOT NULL DEFAULT 0,
    login_time TIMESTAMP NOT NULL DEFAULT now()
);


-- alter table users alter column login_days type INTEGER;

-- ALTER TABLE users ADD COLUMN max_action_force integer DEFAULT 1000 NOT NULL;
-- ALTER TABLE users ADD COLUMN action_force_latest_timestamp integer DEFAULT 0 NOT NULL;

CREATE table user_assets
(
    asid BIGINT PRIMARY KEY,
    uid INTEGER NOT NULL,
    gold_amounts INTEGER NOT NULL DEFAULT 0,
    gem_amounts INTEGER NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now(),
    food_amounts INTEGER NOT NULL,
    wood_amounts INTEGER NOT NULL,
    stone_amounts INTEGER NOT NULL,
);

-- alter table user_assets rename gems to gem_amounts;
-- alter table user_assets rename golds to gold_amounts;
-- alter table user_assets rename foods to food_amounts;
-- alter table user_assets rename woods to wood_amounts;
-- alter table user_assets rename stones to stone_amounts;

-- alter table link_accounts rename to user_link_accounts

CREATE table user_link_accounts
(
    lid BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL,
    account_type SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table servers
(
    sid BIGINT PRIMARY KEY,
    server_number INTEGER NOT NULL,
    name VARCHAR(500) NOT NULL,
--     ip VARCHAR(500) NOT NULL,
--     ports SMALLINT NOT NULL,
    person_count INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table friends
(
    fid BIGINT PRIMARY KEY,
    uuid_a BIGINT NOT NULL,
    uuid_b BIGINT NOT NULL,
    state SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table blacklists
(
    bid BIGINT PRIMARY KEY,
    uuid_a BIGINT NOT NULL,
    uuid_b BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table chat_groups
(
    gid BIGINT PRIMARY KEY,
    group_name VARCHAR(500) NOT NULL,
    group_thumbnail VARCHAR(500) NOT NULL,
    uuid BIGINT NOT NULL,
    person_count SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table chat_groups_uids
(
    guid BIGINT PRIMARY KEY,
    gid BIGINT NOT NULL,
    uuid BIGINT NOT NULL,
    latest_timestamp BIGINT NOT NULL,
    unread_count SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table chat_user_unread_counts
(
    ucid BIGINT PRIMARY KEY,
    uuid_s BIGINT NOT NULL,
    uuid_d BIGINT NOT NULL,
    latest_timestamp BIGINT NOT NULL,
    unread_count SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table chat_messages
(
    mid BIGINT PRIMARY KEY,
    send_id BIGINT NOT NULL,
    to_id BIGINT NOT NULL,
    content VARCHAR(500) NOT NULL,
    created_timestamp BIGINT NOT NULL,
    kind SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now(),
    msg_type SMALLINT NOT NULL DEFAULT 1,
);

CREATE table server_lists
(
    slid BIGINT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    area VARCHAR(50) NOT NULL,
    ip VARCHAR(200) NOT NULL,
    port SMALLINT NOT NULL,
    server_type SMALLINT NOT NULL,
    state SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--create user counter
create sequence seq_user_counter increment by 1 minvalue 1000000 maxvalue 2000000000;
CREATE table user_counters
(
    id INTEGER PRIMARY KEY default nextval('seq_user_counter')
);

INSERT INTO server_lists
    (slid,name,country_code,area,ip,port,server_type,state)
VALUES(7817487988393814824, 'localhost-mache', '86', 'ch', '192.168.1.129', 4434, 2, 0);


---Match3RPG
--
-- drop table enemys;
-- drop table players;
-- drop table skills;
-- drop table skill_fight_relateds;
-- drop table equipment_kinds;
-- drop table equipments;
-- drop table gems;
-- drop table gem_relateds;
-- drop table player_mount_equipments;
-- drop table user_players;
-- drop table user_equipments;
-- drop table user_player_tracks;

--user related
-- delete from users;
-- delete from user_assets;
-- delete from user_players;
-- delete from user_equipments;
-- delete from user_player_tracks;
-- delete from user_link_accounts;
-- delete from friends;
-- delete from blacklists;
-- delete from chat_groups;
-- delete from chat_groups_uids;
-- delete from chat_user_unread_counts;
-- delete from chat_messages;
-- --
-- delete from enemys;
-- delete from players;
-- delete from skills;
-- delete from skill_fight_relateds;
-- delete from equipment_kinds;
-- delete from equipments;
-- delete from gems;
-- delete from gem_relateds;
-- delete from player_mount_equipments;



-- enemys
CREATE table enemys
(
    eid BIGINT PRIMARY KEY,
    enemy_name VARCHAR(200) NOT NULL,
    model_path VARCHAR(100) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    animation_hit_delay REAL NOT NULL,
    spawn_style_class VARCHAR(200) NOT NULL,
    bp_enemy VARCHAR(200) NOT NULL,
    ap_enemy VARCHAR(200) NOT NULL,
    skm_enemy VARCHAR(200) NOT NULL,
    aenemy_die VARCHAR(200) NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

-- players
CREATE table players
(
    pid BIGINT PRIMARY KEY,
    player_name VARCHAR(200) NOT NULL,
    model_path VARCHAR(100) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    animation_hit_delay REAL NOT NULL,
    spawn_style_class VARCHAR(200) NOT NULL,
    level SMALLINT NOT NULL,
    star_level SMALLINT NOT NULL,
    level_experience INTEGER NOT NULL,
    is_default SMALLINT NOT NULL DEFAULT 1,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--skills
CREATE table skills
(
    id BIGINT PRIMARY KEY,
    thumbnail VARCHAR(200) NOT NULL,
    skill_name VARCHAR(200) NOT NULL,
    skill_description VARCHAR(200) NOT NULL,
    model_path VARCHAR(200) NOT NULL,
    cool_down INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    mana_power INTEGER NOT NULL,
    level SMALLINT NOT NULL DEFAULT 0,
    level_experience INTEGER NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--skill_fight_relateds
CREATE table skill_fight_relateds
(
    id BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL,
    cool_down INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    mana_power INTEGER NOT NULL,
    probability SMALLINT NOT NULL DEFAULT 0,
    level SMALLINT NOT NULL DEFAULT 0,
    level_experience INTEGER NOT NULL DEFAULT 0,
    obj_type SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--equipment kinds
CREATE table equipment_kinds
(
    kid BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    kind SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--equipments
CREATE table equipments
(
    eid BIGINT PRIMARY KEY,
    kid BIGINT NOT NULL,
    name VARCHAR(200) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    price INTEGER NOT NULL,
    hp INTEGER NOT NULL,
    multiplier REAL NOT NULL,
    kind SMALLINT NOT NULL,
    is_default SMALLINT NOT NULL DEFAULT 1,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--gems
CREATE table gems
(
    gid BIGINT PRIMARY KEY,
    gem_icon VARCHAR(200) NOT NULL,
    gem_selected_material VARCHAR(200) NOT NULL,
    gem_link_material VARCHAR(200) NOT NULL,
    model_path VARCHAR(200) NOT NULL,
    kind SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



--gem_relateds
CREATE table gem_relateds
(
    grid BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    gid BIGINT NOT NULL,
    obj_type SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--player_mount_equipments
CREATE table player_mount_equipments
(
    id BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    equipment_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--user_players
CREATE table user_players
(
    id BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    level SMALLINT NOT NULL,
    star_level SMALLINT NOT NULL,
    level_experience INTEGER NOT NULL,
    is_default SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--user_equipments
CREATE table user_equipments
(
    id BIGINT PRIMARY KEY,
    eid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--user_player_tracks
CREATE table user_player_tracks
(
    tid BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    rotation_x REAL NOT NULL,
    rotation_y REAL NOT NULL,
    rotation_z REAL NOT NULL,
    location_x REAL NOT NULL,
    location_y REAL NOT NULL,
    location_z REAL NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


-- mall---




--
--
-- delete from mall_first_recharge_gift_packages;
-- delete from mall_first_recharge_gift_package_assets;
-- delete from mall_user_first_recharge_gift_package_records;
-- delete from mall_gem_stores;
-- delete from mall_user_buy_gem_store_records;
-- delete from mall_supply_stations;
-- delete from mall_supply_station_assets;
-- delete from mall_user_buy_supply_station_records;
--
-- delete from props_resources_categories;
-- delete from props_speed_up_categories;
--
-- drop table mall_first_recharge_gift_packages;
-- drop table mall_first_recharge_gift_package_assets;
-- drop table mall_user_first_recharge_gift_package_records;
-- drop table mall_gem_stores;
-- drop table mall_user_buy_gem_store_records;
-- drop table mall_supply_stations;
-- drop table mall_supply_station_assets;
-- drop table mall_user_buy_supply_station_records;
--
-- CREATE table mall_first_recharge_gift_packages
-- (
--     id BIGINT PRIMARY KEY,
--     name VARCHAR(50) NOT NULL,
--     bg_url VARCHAR(200) NOT NULL,
--     bg_desc VARCHAR(200) NOT NULL,
--     asset_desc VARCHAR(200) NOT NULL,
--     price REAL NOT NULL,
--     product_number VARCHAR(200) NOT NULL,
--     status SMALLINT NOT NULL DEFAULT 0,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
--
-- CREATE table mall_first_recharge_gift_package_assets
-- (
--     id BIGINT PRIMARY KEY,
--     obj_id BIGINT NOT NULL,
--     obj_type SMALLINT NOT NULL,
--     sort_value SMALLINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_user_first_recharge_gift_package_records
-- (
--     id BIGINT PRIMARY KEY,
--     uuid BIGINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
--
-- --gem store
--
-- CREATE table mall_gem_stores
-- (
--     id BIGINT PRIMARY KEY,
--     name VARCHAR(200) NOT NULL,
--     gem_count INTEGER NOT NULL,
--     first_buy_handsel INTEGER NOT NULL,
--     late_buy_handsel INTEGER NOT NULL,
--     price REAL NOT NULL,
--     product_number VARCHAR(200) NOT NULL,
--     status SMALLINT NOT NULL DEFAULT 0,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_user_buy_gem_store_records
-- (
--     rid BIGINT PRIMARY KEY,
--     uuid BIGINT NOT NULL,
--     gsid BIGINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- --supply station
--
-- CREATE table mall_supply_stations
-- (
--     id BIGINT PRIMARY KEY,
--     name VARCHAR(200) NOT NULL,
--     description VARCHAR(200) NOT NULL,
--     thumbnail VARCHAR(200) NOT NULL,
--     gem_count INTEGER NOT NULL,
--     valid_period_day SMALLINT NOT NULL,
--     price REAL NOT NULL,
--     product_number VARCHAR(200) NOT NULL,
--     status SMALLINT NOT NULL DEFAULT 0,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
--
-- CREATE table mall_supply_station_assets
-- (
--     id BIGINT PRIMARY KEY,
--     sid BIGINT NOT NULL,
--     obj_id BIGINT NOT NULL,
--     obj_type SMALLINT NOT NULL,
--     sort_value SMALLINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_user_buy_supply_station_records
-- (
--     id BIGINT PRIMARY KEY,
--     uuid BIGINT NOT NULL,
--     sid BIGINT NOT NULL,
--     expire_time BIGINT NOT NULL,
--     latest_receive_time BIGINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
--
-- --mall super value bundle
-- CREATE table mall_super_value_bundles
-- (
--     sid BIGINT PRIMARY KEY,
--     name VARCHAR(200) NOT NULL,
--     thumbnail VARCHAR(200) NOT NULL,
--     small_icon VARCHAR(200) NOT NULL,
--     is_buy_limit SMALLINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_super_value_bundle_levels
-- (
--     lid BIGINT PRIMARY KEY,
--     sid BIGINT NOT NULL ,
--     description VARCHAR(200) NOT NULL,
--     gem_count INTEGER NOT NULL,
--     level SMALLINT NOT NULL,
--     price REAL NOT NULL,
--     product_number VARCHAR(200) NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_user_buy_super_value_bundle_records
-- (
--     rid BIGINT PRIMARY KEY,
--     uuid BIGINT NOT NULL ,
--     sid BIGINT NOT NULL ,
--     level SMALLINT NOT NULL,
--     is_buy_limit SMALLINT NOT NULL,
--     buy_timestamp BIGINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- --mall_daily_special_offers
--
-- CREATE table mall_daily_special_offers
-- (
--     did BIGINT PRIMARY KEY,
--     name VARCHAR(200) NOT NULL,
--     thumbnail VARCHAR(200) NOT NULL,
--     description VARCHAR(200) NOT NULL,
--     gem_count INTEGER NOT NULL,
--     price REAL NOT NULL,
--     product_number VARCHAR(200) NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- CREATE table mall_user_buy_daily_special_offer_records
-- (
--     rid BIGINT PRIMARY KEY,
--     uuid BIGINT NOT NULL ,
--     did BIGINT NOT NULL ,
--     buy_timestamp BIGINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );
--
-- --mall props
--
-- CREATE table mall_props
-- (
--     aid BIGINT PRIMARY KEY,
--     mall_obj_id BIGINT NOT NULL,
--     mall_obj_type SMALLINT NOT NULL,
--     obj_id BIGINT NOT NULL,
--     obj_type SMALLINT NOT NULL,
--     count SMALLINT NOT NULL,
--     modify_time TIMESTAMP NOT NULL DEFAULT now(),
--     created_time TIMESTAMP NOT NULL DEFAULT now()
-- );


--pay order table

CREATE table purchase_orders
(
    oid BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    obj_type SMALLINT NOT NULL,
    uuid BIGINT NOT NULL,
    hash VARCHAR(200) NOT NULL,
    product_number VARCHAR(200) NOT NULL,
    pay_platform SMALLINT NOT NULL,
    order_no BIGINT NOT NULL,
    status SMALLINT NOT NULL,
    pay_time BIGINT NOT NULL,
    price REAL NOT NULL,
    request_receipt_data TEXT NOT NULL,
    response_receipt_data TEXT NOT NULL,
    deleted_time BIGINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--items

-- alter table props_item_metadatas alter column sub_item_type type INTEGER;
-- alter table props_item_metadatas alter column bag_type type INTEGER;
-- alter table props_item_metadatas alter column rarity_type type INTEGER;

-- alter table user_item_bags alter column bag_type type INTEGER;
-- alter table user_item_bags alter column sub_item_type type INTEGER;

--item metadatas

-- drop table props_item_metadatas;
-- drop table props_resources_categories;
-- drop table props_speed_up_categories;
-- drop table props_action_points_categories;
-- drop table props_tome_of_knowledge_categories;
-- drop table props_starlight_sculpture_categories;
-- drop table props_key_categories;
-- drop table props_fixed_treasure_chest_categories;
-- drop table props_fixed_treasure_chest_category_assets;
-- drop table buff_metadatas;



CREATE table props_item_metadatas
(
    item_id BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    description VARCHAR(200) NOT NULL,
    overlay_status SMALLINT NOT NULL,
    sub_item_type INTEGER NOT NULL,
    bag_type INTEGER NOT NULL,
    rarity_type INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--资源类道具
CREATE table props_resources_categories
(
    item_id BIGINT PRIMARY KEY,
    rss_value INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--加速类道具
CREATE table props_speed_up_categories
(
    item_id BIGINT PRIMARY KEY,
    speed_time INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--行动力道具
CREATE table props_action_points_categories
(
    item_id BIGINT PRIMARY KEY,
    ap_value INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_tome_of_knowledge_categories
(
    item_id BIGINT PRIMARY KEY,
    exp_value INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table props_starlight_sculpture_categories
(
    item_id BIGINT PRIMARY KEY,
    exp_value INTEGER NOT NULL,
    probability_value REAL NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_key_categories
(
    item_id BIGINT PRIMARY KEY,
    chest_item_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table props_fixed_treasure_chest_categories
(
    item_id BIGINT PRIMARY KEY,
    price REAL NOT NULL,
    is_instantly_open SMALLINT NOT NULL,
    option_values SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--道具商城资产
CREATE table props_fixed_treasure_chest_category_assets
(
    aid BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    sub_item_id BIGINT NOT NULL,
    amounts INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--道具商城
CREATE table props_malls
(
    item_id BIGINT PRIMARY KEY,
    next_item_id BIGINT NOT NULL,
    level BIGINT NOT NULL,
    item_category INTEGER NOT NULL,
    price REAL NOT NULL,
    purchase_limit SMALLINT NOT NULL,
    small_icon VARCHAR(200) NOT NULL,
    gem_amounts INTEGER NOT NULL,
    food_amounts INTEGER NOT NULL,
    wood_amounts INTEGER NOT NULL,
    first_buy_handsel INTEGER NOT NULL,
    late_buy_handsel INTEGER NOT NULL,
    valid_period_day SMALLINT NOT NULL,
    mall_type INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--道具商城资产
CREATE table props_mall_assets
(
    aid BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    sub_item_id BIGINT NOT NULL,
    amounts INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--用户购买道具商城记录
CREATE table user_buy_props_mall_records
(
    rid BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    uuid BIGINT NOT NULL,
    level BIGINT NOT NULL,
    item_category INTEGER NOT NULL,
    purchase_limit SMALLINT NOT NULL,
    expire_time BIGINT NOT NULL,
    latest_receive_time BIGINT NOT NULL,
    mall_type INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--用户购买道具商城历史记录
CREATE table user_buy_props_mall_record_logs
(
    rid BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    uuid BIGINT NOT NULL,
    level BIGINT NOT NULL,
    item_category INTEGER NOT NULL,
    purchase_limit SMALLINT NOT NULL,
    expire_time BIGINT NOT NULL,
    latest_receive_time BIGINT NOT NULL,
    mall_type INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table props_product_numbers
(
    item_id BIGINT PRIMARY KEY,
    product_number VARCHAR(300) NOT NULL,
    platform_id SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



--action point

CREATE table system_configs
(
    scid BIGINT PRIMARY KEY,
    key VARCHAR(200) NOT NULL,
    value varchar(500)  NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--增益表
CREATE table user_buffs
(
    bid BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    obj_type SMALLINT NOT NULL,
    buff_id BIGINT NOT NULL,
    buff_amounts INTEGER NOT NULL,
    buff_category INTEGER NOT NULL,
    buff_type INTEGER NOT NULL,
    sub_buff_type INTEGER NOT NULL,
    buff_source INTEGER NOT NULL,
    is_show SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table buff_metadatas
(
    buff_id BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    amounts INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    buff_category INTEGER NOT NULL,
    buff_type INTEGER NOT NULL,
    sub_buff_type INTEGER NOT NULL,
    buff_source INTEGER NOT NULL,
    is_show SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--vip system

CREATE table vip_levels
(
    level BIGINT PRIMARY KEY,
    vip_points_needed INTEGER NOT NULL,
    free_treasure_chest_item_id BIGINT NOT NULL,
    pay_treasure_chest_item_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table vip_buffs
(
    id BIGINT PRIMARY KEY,
    level BIGINT Not Null,
    buff_id BIGINT NOT NULL,
    is_new_mark SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



CREATE table vip_daily_login_treasure_chests
(
    id BIGINT PRIMARY KEY,
    continuous_login_days INTEGER NOT NULL ,
    today_vip_points INTEGER Not Null ,
    tomorrow_vip_points INTEGER Not Null ,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



CREATE table user_vips
(
    id BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL,
    level BIGINT Not Null,
    vip_points INTEGER Not Null,
    daily_treasure_chests_time BIGINT Not Null,
    free_everyday_treasure_chests_time BIGINT Not Null,
    special_privileges_treasure_chests_time BIGINT Not Null,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table user_queues
(
    qid BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL ,
    building_queue SMALLINT NOT NULL,
    research_queue SMALLINT NOT NULL,
    training_queue SMALLINT NOT NULL,
    healing_queue SMALLINT NOT NULL,
    armies_queue SMALLINT NOT NULL,
    scout_queue SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table shops
(
    sid BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    bag_type INTEGER NOT NULL,
    sub_item_type INTEGER NOT NULL,
    order_value BIGINT NOT NULL,
    gems_needed INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table shop_vip_metadatas
(
    shop_vip_id BIGINT PRIMARY KEY,
    level BIGINT NOT NULL ,
    item_id BIGINT NOT NULL,
    bag_type INTEGER NOT NULL,
    sub_item_type INTEGER NOT NULL,
    cost_value INTEGER NOT NULL,
    attribute_id INTEGER NOT NULL,
    discount REAL NOT NULL,
    limit_amounts INTEGER NOT NULL,
    order_value BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table user_buy_shop_vip_records
(
    id BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL,
    shop_vip_id BIGINT NOT NULL,
    amounts INTEGER NOT NULL ,
    week_time INTEGER NOT NULL,      -- year*100+一年中的第几个周
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table categories
(
    id BIGINT PRIMARY KEY,
    name varchar(300) NOT NULL,
    type_id INTEGER NOT NULL,
    type_name varchar(200) NOT NULL ,
    system_name varchar(200) NOT NULL ,
    system_id INTEGER NOT NULL ,
    table_name varchar(200)  NOT NULL ,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE TABLE quests_metadatas (
    quests_id BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    description VARCHAR(200) NOT NULL,
    quests_value INTEGER NOT NULL,
    quests_type INTEGER NOT NULL,
    sub_quests_type INTEGER NOT NULL,
    modify_time timestamp NOT NULL DEFAULT now(),
    created_time timestamp NOT NULL DEFAULT now()
);

CREATE TABLE quests_attribute_assets (
   id BIGINT PRIMARY KEY,
   quests_id BIGINT NOT NULL,
   attribute_id INTEGER NOT NULL,
   amounts INTEGER NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);

CREATE TABLE quests_assets (
 id BIGINT PRIMARY KEY,
 quests_id BIGINT NOT NULL,
 item_id BIGINT NOT NULL,
 amounts INTEGER NOT NULL,
 modify_time timestamp NOT NULL DEFAULT now(),
 created_time timestamp NOT NULL DEFAULT now()
);


-- alter table props_fixed_treasure_chest_categories rename to props_fixed_treasure_chest_categories;
-- alter table props_action_points_categories rename to props_action_points_categories;
-- alter table props_key_categories rename to props_key_categories;
-- alter table props_resources_categories rename to props_resources_categories;
-- alter table props_speed_up_categories rename to props_speed_up_categories;
-- alter table props_starlight_sculpture_categories rename to props_starlight_sculpture_categories;
-- alter table props_tome_of_knowledge_categories rename to props_tome_of_knowledge_categories;

CREATE TABLE quests_relations (
    id BIGINT PRIMARY KEY,
    quests_id BIGINT NOT NULL,
    quests_next_id BIGINT NOT NULL,
    endpoint INTEGER NOT NULL,
    quests_type INTEGER NOT NULL,
    sub_quests_type INTEGER NOT NULL,
    modify_time timestamp NOT NULL DEFAULT now(),
    created_time timestamp NOT NULL DEFAULT now()
);

CREATE TABLE user_quests (
   id BIGINT PRIMARY KEY,
   uuid BIGINT NOT NULL,
   quests_id BIGINT NOT NULL,
   quests_finished_value INTEGER NOT NULL,
   is_finished INTEGER NOT NULL,
   is_receive_award SMALLINT NOT NULL,
   day_time BIGINT NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);

//building
CREATE TABLE quests_building_categories (
   quests_id BIGINT PRIMARY KEY,
   building_id BIGINT  NULL,
   level INTEGER NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);


//collect
CREATE TABLE quests_collect_categories (
   quests_id BIGINT PRIMARY KEY,
   building_id BIGINT NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);

//barbarians
CREATE TABLE quests_barbarians_categories (
   quests_id BIGINT PRIMARY KEY,
   barbarian_id BIGINT NOT NULL,
   level INTEGER NOT NULL,
   number_of_times INTEGER NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);


//training
CREATE TABLE quests_training_categories (
   quests_id BIGINT PRIMARY KEY,
   building_id BIGINT NOT NULL,
   troop_id BIGINT NOT NULL,
   level INTEGER NOT NULL,
   modify_time timestamp NOT NULL DEFAULT now(),
   created_time timestamp NOT NULL DEFAULT now()
);






-- Boosts
-- Equipment

-- Builder Recruitment
--// Commander Sculpture
-- // Teleport
-- //Arrow Of Resistance
-- Random Chests


CREATE table props_boost_categories
(
    item_id BIGINT PRIMARY KEY,
    boost_time BIGINT NOT NULL,
    boost_value REAL NOT NULL,
    buff_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_builder_recruitment_categories
(
    item_id BIGINT PRIMARY KEY,
    recruit_time BIGINT NOT NULL,
    buff_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



CREATE table props_random_treasure_chest_categories
(
    item_id BIGINT PRIMARY KEY,
    price REAL NOT NULL,
    is_instantly_open SMALLINT NOT NULL,
    option_values SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_random_treasure_chest_category_attribute_assets
(
    id BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    attribute_id INTEGER NOT NULL,
    amounts INTEGER NOT NULL,
    probability_value REAL NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



CREATE table props_random_treasure_chest_category_assets
(
    id BIGINT PRIMARY KEY,
    item_id BIGINT NOT NULL,
    sub_item_id BIGINT NOT NULL,
    amounts INTEGER NOT NULL,
    probability_value REAL NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);




CREATE  table versions
(
    version_id BIGINT PRIMARY KEY,
    last_version_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table version_meta_relations
(
    operation_id BIGINT PRIMARY KEY,
    version_id BIGINT NOT NULL,
    update_type SMALLINT NOT NULL,
    table_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



CREATE table version_meta_operation_relations
(
    id BIGINT PRIMARY KEY,
    operation_id BIGINT NOT NULL,
    version_id BIGINT NOT NULL,
    action_id BIGINT NOT NULL,
    action_type INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

