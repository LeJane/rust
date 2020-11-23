
CREATE table exlusive_buffs
(
    exid BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    buff_type_id BIGINT NOT NULL,
    buff_value INTEGER NOT NULL,
    is_show SMALLINT NOT NULL,
    kind SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table vip_levels
(
    vlid BIGINT PRIMARY KEY,
    vip_points_needed INTEGER NOT NULL,
    level SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table vip_buffs
(
    vbid BIGINT PRIMARY KEY,
    vlid BIGINT Not Null,
    buff_id BIGINT PRIMARY KEY,
    buff_value INTEGER NOT NULL,
    is_new_mark SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table vip_exclusive_treasure_chests
(
    etcid BIGINT PRIMARY KEY,
    vlid BIGINT NOT NULL ,
    item_id BIGINT NOT NULL,
    count INTEGER NOT NULL,
    price REAL NOT NULL,
    product_number VARCHAR(200) NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table vip_daily_login_treasure_chests
(
    vdltc_id BIGINT PRIMARY KEY,
    continuous_login_days INTEGER NOT NULL ,
    today_vip_points SMALLINT Not Null ,
    tomorrow_vip_points SMALLINT Not Null ,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table user_vips
(
    uvid BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL,
    level SMALLINT Not Null,
    vip_points INTEGER Not Null,
    daily_treasure_chests_status SMALLINT Not Null,
    free_everyday_treasure_chests_status SMALLINT Not Null,
    special_privileges_treasure_chests_status SMALLINT Not Null,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);