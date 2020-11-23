table! {
    blacklists (bid) {
        bid -> Int8,
        uuid_a -> Int8,
        uuid_b -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    buff_metadatas (buff_id) {
        buff_id -> Int8,
        name -> Varchar,
        amounts -> Int4,
        attribute_id -> Int4,
        buff_category -> Int4,
        buff_type -> Int4,
        sub_buff_type -> Int4,
        buff_source -> Int4,
        is_show -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Int8,
        name -> Varchar,
        type_id -> Int4,
        type_name -> Varchar,
        system_name -> Varchar,
        system_id -> Int4,
        table_name -> Varchar,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_groups (gid) {
        gid -> Int8,
        group_name -> Varchar,
        group_thumbnail -> Varchar,
        uuid -> Int8,
        person_count -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_groups_uids (guid) {
        guid -> Int8,
        gid -> Int8,
        uuid -> Int8,
        latest_timestamp -> Int8,
        unread_count -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_messages (mid) {
        mid -> Int8,
        send_id -> Int8,
        to_id -> Int8,
        content -> Varchar,
        created_timestamp -> Int8,
        kind -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
        msg_type -> Int2,
    }
}

table! {
    chat_user_unread_counts (ucid) {
        ucid -> Int8,
        uuid_s -> Int8,
        uuid_d -> Int8,
        latest_timestamp -> Int8,
        unread_count -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    enemys (eid) {
        eid -> Int8,
        enemy_name -> Varchar,
        model_path -> Varchar,
        thumbnail -> Varchar,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        animation_hit_delay -> Float4,
        spawn_style_class -> Varchar,
        bp_enemy -> Varchar,
        ap_enemy -> Varchar,
        skm_enemy -> Varchar,
        enemy_die -> Varchar,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    equipment_kinds (kid) {
        kid -> Int8,
        name -> Varchar,
        kind -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    equipments (eid) {
        eid -> Int8,
        kid -> Int8,
        name -> Varchar,
        thumbnail -> Varchar,
        price -> Int4,
        hp -> Int4,
        multiplier -> Float4,
        kind -> Int2,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    friends (fid) {
        fid -> Int8,
        uuid_a -> Int8,
        uuid_b -> Int8,
        state -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    gem_relateds (grid) {
        grid -> Int8,
        obj_id -> Int8,
        gid -> Int8,
        obj_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    gems (gid) {
        gid -> Int8,
        gem_icon -> Varchar,
        gem_selected_material -> Varchar,
        gem_link_material -> Varchar,
        model_path -> Varchar,
        kind -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    player_mount_equipments (id) {
        id -> Int8,
        pid -> Int8,
        uid -> Int8,
        equipment_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    players (pid) {
        pid -> Int8,
        player_name -> Varchar,
        model_path -> Varchar,
        thumbnail -> Varchar,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        animation_hit_delay -> Float4,
        spawn_style_class -> Varchar,
        level -> Int2,
        star_level -> Int2,
        level_experience -> Int4,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_action_points_categories (item_id) {
        item_id -> Int8,
        ap_value -> Int4,
        attribute_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_boost_categories (item_id) {
        item_id -> Int8,
        boost_time -> Int8,
        boost_value -> Float4,
        buff_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_builder_recruitment_categories (item_id) {
        item_id -> Int8,
        recruit_time -> Int8,
        buff_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_fixed_treasure_chest_categories (item_id) {
        item_id -> Int8,
        price -> Float4,
        is_instantly_open -> Int2,
        option_values -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_fixed_treasure_chest_category_assets (aid) {
        aid -> Int8,
        item_id -> Int8,
        sub_item_id -> Int8,
        amounts -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_item_metadatas (item_id) {
        item_id -> Int8,
        name -> Varchar,
        thumbnail -> Varchar,
        description -> Varchar,
        overlay_status -> Int2,
        sub_item_type -> Int4,
        bag_type -> Int4,
        rarity_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_key_categories (item_id) {
        item_id -> Int8,
        chest_item_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_mall_assets (aid) {
        aid -> Int8,
        item_id -> Int8,
        sub_item_id -> Int8,
        amounts -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_malls (item_id) {
        item_id -> Int8,
        next_item_id -> Int8,
        level -> Int8,
        item_category -> Int4,
        price -> Float4,
        purchase_limit -> Int2,
        small_icon -> Varchar,
        gem_amounts -> Int4,
        food_amounts -> Int4,
        wood_amounts -> Int4,
        first_buy_handsel -> Int4,
        late_buy_handsel -> Int4,
        valid_period_day -> Int2,
        mall_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_product_numbers (item_id) {
        item_id -> Int8,
        product_number -> Varchar,
        platform_id -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_random_treasure_chest_categories (item_id) {
        item_id -> Int8,
        price -> Float4,
        is_instantly_open -> Int2,
        option_values -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_random_treasure_chest_category_assets (id) {
        id -> Int8,
        item_id -> Int8,
        sub_item_id -> Int8,
        amounts -> Int4,
        probability_value -> Float4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_random_treasure_chest_category_attribute_assets (id) {
        id -> Int8,
        item_id -> Int8,
        attribute_id -> Int4,
        amounts -> Int4,
        probability_value -> Float4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_resources_categories (item_id) {
        item_id -> Int8,
        rss_value -> Int4,
        attribute_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_speed_up_categories (item_id) {
        item_id -> Int8,
        speed_time -> Int4,
        attribute_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_starlight_sculpture_categories (item_id) {
        item_id -> Int8,
        exp_value -> Int4,
        probability_value -> Float4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    props_tome_of_knowledge_categories (item_id) {
        item_id -> Int8,
        exp_value -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    purchase_orders (oid) {
        oid -> Int8,
        obj_id -> Int8,
        obj_type -> Int2,
        uuid -> Int8,
        hash -> Varchar,
        product_number -> Varchar,
        pay_platform -> Int2,
        order_no -> Int8,
        status -> Int2,
        pay_time -> Int8,
        price -> Float4,
        request_receipt_data -> Text,
        response_receipt_data -> Text,
        deleted_time -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    quests_assets (id) {
        id -> Int8,
        quests_id -> Int8,
        item_id -> Int8,
        amounts -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    quests_attribute_assets (id) {
        id -> Int8,
        quests_id -> Int8,
        attribute_id -> Int4,
        amounts -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    quests_metadatas (quests_id) {
        quests_id -> Int8,
        name -> Varchar,
        thumbnail -> Varchar,
        description -> Varchar,
        quests_value -> Int4,
        quests_type -> Int4,
        sub_quests_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    quests_relations (id) {
        id -> Int8,
        quests_id -> Int8,
        quests_next_id -> Int8,
        endpoint -> Int4,
        quests_type -> Int4,
        sub_quests_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    server_lists (slid) {
        slid -> Int8,
        name -> Varchar,
        country_code -> Varchar,
        area -> Varchar,
        ip -> Varchar,
        port -> Int2,
        server_type -> Int2,
        state -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    servers (sid) {
        sid -> Int8,
        server_number -> Int4,
        name -> Varchar,
        person_count -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    shops (sid) {
        sid -> Int8,
        item_id -> Int8,
        bag_type -> Int4,
        sub_item_type -> Int4,
        order_value -> Int8,
        gems_needed -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    shop_vip_metadatas (shop_vip_id) {
        shop_vip_id -> Int8,
        level -> Int8,
        item_id -> Int8,
        bag_type -> Int4,
        sub_item_type -> Int4,
        cost_value -> Int4,
        attribute_id -> Int4,
        discount -> Float4,
        limit_amounts -> Int4,
        order_value -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    skill_fight_relateds (id) {
        id -> Int8,
        obj_id -> Int8,
        skill_id -> Int8,
        cool_down -> Int4,
        attack_power -> Int4,
        mana_power -> Int4,
        probability -> Int2,
        level -> Int2,
        level_experience -> Int4,
        obj_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    skills (id) {
        id -> Int8,
        thumbnail -> Varchar,
        skill_name -> Varchar,
        skill_description -> Varchar,
        model_path -> Varchar,
        cool_down -> Int4,
        attack_power -> Int4,
        mana_power -> Int4,
        level -> Int2,
        level_experience -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    system_configs (scid) {
        scid -> Int8,
        key -> Varchar,
        value -> Varchar,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_assets (asid) {
        asid -> Int8,
        uid -> Int8,
        gold_amounts -> Int4,
        gem_amounts -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
        food_amounts -> Int4,
        wood_amounts -> Int4,
        stone_amounts -> Int4,
    }
}

table! {
    user_buffs (bid) {
        bid -> Int8,
        obj_id -> Int8,
        obj_type -> Int2,
        buff_id -> Int8,
        buff_amounts -> Int4,
        buff_category -> Int4,
        buff_type -> Int4,
        sub_buff_type -> Int4,
        buff_source -> Int4,
        is_show -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_buy_props_mall_record_logs (rid) {
        rid -> Int8,
        item_id -> Int8,
        uuid -> Int8,
        level -> Int8,
        item_category -> Int4,
        purchase_limit -> Int2,
        expire_time -> Int8,
        latest_receive_time -> Int8,
        mall_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_buy_props_mall_records (rid) {
        rid -> Int8,
        item_id -> Int8,
        uuid -> Int8,
        level -> Int8,
        item_category -> Int4,
        purchase_limit -> Int2,
        expire_time -> Int8,
        latest_receive_time -> Int8,
        mall_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_buy_shop_vip_records (id) {
        id -> Int8,
        uuid -> Int8,
        shop_vip_id -> Int8,
        amounts -> Int4,
        week_time -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_counters (id) {
        id -> Int4,
    }
}

table! {
    user_equipments (id) {
        id -> Int8,
        eid -> Int8,
        uid -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_item_bags (bid) {
        bid -> Int8,
        uuid -> Int8,
        item_id -> Int8,
        overlay_status -> Int2,
        bag_type -> Int4,
        count -> Int4,
        order_value -> Int8,
        sub_item_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_link_accounts (lid) {
        lid -> Int8,
        uuid -> Int8,
        account_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_players (id) {
        id -> Int8,
        pid -> Int8,
        uid -> Int8,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        level -> Int2,
        star_level -> Int2,
        level_experience -> Int4,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_player_tracks (tid) {
        tid -> Int8,
        pid -> Int8,
        uid -> Int8,
        rotation_x -> Float4,
        rotation_y -> Float4,
        rotation_z -> Float4,
        location_x -> Float4,
        location_y -> Float4,
        location_z -> Float4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_quests (id) {
        id -> Int8,
        uuid -> Int8,
        quests_id -> Int8,
        quests_finished_value -> Int4,
        is_finished -> Int4,
        is_receive_award -> Int2,
        day_time -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_queues (qid) {
        qid -> Int8,
        uuid -> Int8,
        building_queue -> Int2,
        research_queue -> Int2,
        training_queue -> Int2,
        healing_queue -> Int2,
        armies_queue -> Int2,
        scout_queue -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    users (uuid) {
        uuid -> Int8,
        uid -> Int4,
        name -> Varchar,
        avatar -> Varchar,
        login_days -> Int4,
        server_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
        action_points -> Int4,
        max_action_points -> Int4,
        action_points_latest_timestamp -> Int8,
        login_time -> Timestamp,
    }
}

table! {
    user_vips (id) {
        id -> Int8,
        uuid -> Int8,
        level -> Int8,
        vip_points -> Int4,
        daily_treasure_chests_time -> Int8,
        free_everyday_treasure_chests_time -> Int8,
        special_privileges_treasure_chests_time -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    version_meta_operation_relations (id) {
        id -> Int8,
        operation_id -> Int8,
        version_id -> Int8,
        action_id -> Int8,
        action_type -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    version_meta_relations (operation_id) {
        operation_id -> Int8,
        version_id -> Int8,
        update_type -> Int2,
        table_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    versions (version_id) {
        version_id -> Int8,
        last_version_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    vip_buffs (id) {
        id -> Int8,
        level -> Int8,
        buff_id -> Int8,
        is_new_mark -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    vip_daily_login_treasure_chests (id) {
        id -> Int8,
        continuous_login_days -> Int4,
        today_vip_points -> Int4,
        tomorrow_vip_points -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    vip_levels (level) {
        level -> Int8,
        vip_points_needed -> Int4,
        free_treasure_chest_item_id -> Int8,
        pay_treasure_chest_item_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklists,
    buff_metadatas,
    categories,
    chat_groups,
    chat_groups_uids,
    chat_messages,
    chat_user_unread_counts,
    enemys,
    equipment_kinds,
    equipments,
    friends,
    gem_relateds,
    gems,
    player_mount_equipments,
    players,
    props_action_points_categories,
    props_boost_categories,
    props_builder_recruitment_categories,
    props_fixed_treasure_chest_categories,
    props_fixed_treasure_chest_category_assets,
    props_item_metadatas,
    props_key_categories,
    props_mall_assets,
    props_malls,
    props_product_numbers,
    props_random_treasure_chest_categories,
    props_random_treasure_chest_category_assets,
    props_random_treasure_chest_category_attribute_assets,
    props_resources_categories,
    props_speed_up_categories,
    props_starlight_sculpture_categories,
    props_tome_of_knowledge_categories,
    purchase_orders,
    quests_assets,
    quests_attribute_assets,
    quests_metadatas,
    quests_relations,
    server_lists,
    servers,
    shops,
    shop_vip_metadatas,
    skill_fight_relateds,
    skills,
    system_configs,
    user_assets,
    user_buffs,
    user_buy_props_mall_record_logs,
    user_buy_props_mall_records,
    user_buy_shop_vip_records,
    user_counters,
    user_equipments,
    user_item_bags,
    user_link_accounts,
    user_players,
    user_player_tracks,
    user_quests,
    user_queues,
    users,
    user_vips,
    version_meta_operation_relations,
    version_meta_relations,
    versions,
    vip_buffs,
    vip_daily_login_treasure_chests,
    vip_levels,
);
