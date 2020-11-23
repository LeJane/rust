
--item metadatas
CREATE table item_metadatas
(
    item_id BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    description VARCHAR(200) NOT NULL,
    overlay_status SMALLINT NOT NULL,
    item_type SMALLINT NOT NULL,
    sub_item_type SMALLINT NOT NULL,
    bag_type SMALLINT NOT NULL,
    rarity_type SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--资源类道具
CREATE table props_resources_categorys
(
    item_id BIGINT PRIMARY KEY,
    rss_value INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--加速类道具
CREATE table props_speed_up_categorys
(
    item_id BIGINT PRIMARY KEY,
    speed_time INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--行动力道具
CREATE table props_action_points_categorys
(
    item_id BIGINT PRIMARY KEY,
    ap_value INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_tome_of_knowledge_categorys
(
    item_id BIGINT PRIMARY KEY,
    exp_value SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table props_starlight_sculpture_categorys
(
    item_id BIGINT PRIMARY KEY,
    exp_value SMALLINT NOT NULL,
    probability_value SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table props_key_categorys
(
    item_id BIGINT PRIMARY KEY,
    chest_item_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table props_fixed_treasure_chest_categorys
(
    item_id BIGINT PRIMARY KEY,
    price REAL NOT NULL,
    product_number VARCHAR(200) NOT NULL,
    is_instantly_open SMALLINT NOT NULL,
    option_values SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



