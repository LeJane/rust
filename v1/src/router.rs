use crate::controller;
use crate::RouterRegister;
use std::sync::Arc;

pub fn build_routers() -> Arc<RouterRegister> {
    let mut routers = RouterRegister::new();
    routers.add(1000, controller::server_list::get_server_list);
    routers.add(1001, controller::users::create_user);
    routers.add(1002, controller::user_equipments::get_user_equipment_list);
    routers.add(1003, controller::user_players::get_user_default_player_data);
    routers.add(1004, controller::equipment_kinds::get_equipment_kind_list);
    routers.add(1005, controller::equipments::get_shop_equipment_list_by_kid);
    routers.add(1006, controller::user_players::get_player_collection_data);
    routers.add(1007, controller::enemys::get_enemy_list_data);
    routers.add(1008, controller::users::get_user_base_info);
    routers.add(1009, controller::user_assets::get_user_asset_info);
    routers.add(1010, controller::user_assets::update_user_asset_info);
    routers.add(1011, controller::users::update_user_action_force);
    routers.add(
        1012,
        controller::user_player_track::insert_or_update_player_track,
    );
    routers.add(1013, controller::user_player_track::get_player_track);
    routers.add(
        1014,
        controller::player_mount_equipments::mount_user_player_equipment,
    );
    routers.add(
        1015,
        controller::player_mount_equipments::umount_user_player_equipment,
    );
    routers.add(
        1016,
        controller::player_mount_equipments::switch_user_player_equipment,
    );
    routers.add(1017, controller::users::update_user_name);
    routers.add(1018, controller::user_equipments::user_buy_equipment);
    routers.add(1019, controller::friends::get_user_friend_list);
    routers.add(1020, controller::friends::get_user_new_friend_list);
    routers.add(1021, controller::group_chats::get_group_chat_list);
    routers.add(1022, controller::group_chats::create_group_chat);
    routers.add(1023, controller::group_chats::add_member_to_group_chat);
    routers.add(1024, controller::group_chats::del_member_from_group_chat);
    routers.add(1025, controller::group_chats::update_group_chat_name);
    routers.add(1026, controller::blacklist::get_user_black_list);
    routers.add(1027, controller::blacklist::add_user_to_black_list);
    routers.add(1028, controller::friends::update_friend_state);
    routers.add(1029, controller::friends::send_friend_request);
    routers.add(1030, controller::friends::del_friend);
    routers.add(1031, controller::group_chats::exit_group_chat);
    routers.add(1032, controller::group_chats::get_group_chat_detail);
    routers.add(1033, controller::blacklist::del_black_list);
    routers.add(1034, controller::props_malls::get_first_recharge_gift_data);
    routers.add(1035, controller::props_malls::get_props_mall_metadata_list);
    routers.add(1036, controller::props_malls::get_supply_station_list);
    routers.add(1037, controller::friends::search_friend_by_uid);
    routers.add(1038, controller::friends::get_special_user_info);
    routers.add(1039, controller::props_malls::get_super_value_bundle_list);
    routers.add(1040, controller::props_malls::get_daily_special_offer_list);

    routers.add(1041, controller::users::get_user_action_points);
    routers.add(1042, controller::user_item_bags::get_user_item_bag_list);
    routers.add(1043, controller::user_vips::get_user_vip_data);
    routers.add(
        1044,
        controller::user_vips::receive_vip_exlusive_free_treasure_chest,
    );
    routers.add(
        1045,
        controller::user_vips::receive_vip_daily_login_treasure_chest,
    );
    routers.add(1046, controller::user_vips::get_buy_vip_points_data_list);
    routers.add(1047, controller::user_vips::buy_vip_points_by_gem_or_item);
    routers.add(1048, controller::server_list::get_server_utc_time);
    routers.add(1049, controller::shops::get_shop_list);
    routers.add(1050, controller::shops::buy_shop_item_by_gems);
    routers.add(1051, controller::shops::get_shop_vip_metadata_list);
    routers.add(1052, controller::shops::buy_shop_vip_item);

   //new
    routers.add(1053, controller::equipments::get_equipment_list);
    routers.add(1054, controller::buff_metadatas::get_buff_metadata_list);
    routers.add(1055, controller::gem_relateds::get_gem_relation_list);
    routers.add(1056, controller::gems::get_gem_list);
    routers.add(1057, controller::players::get_player_list);
    routers.add(1058, controller::props_items::get_item_metadata_list);
    routers.add(1059, controller::props_items::get_action_points_props_list);
    routers.add(1060, controller::props_items::get_boost_props_list);
    routers.add(1061, controller::props_items::get_builder_recruitment_props_list);
    routers.add(1062, controller::props_items::get_fixed_treasure_chest_list);
    routers.add(1063, controller::props_items::get_fixed_treasure_chest_asset_list);
    routers.add(1064, controller::props_items::get_key_props_list);
    routers.add(1065, controller::props_items::get_props_product_number_list);
    routers.add(1066, controller::props_items::get_random_treasure_chest_list);
    routers.add(1067, controller::props_items::get_random_treasure_chest_asset_list);
    routers.add(1068, controller::props_items::get_random_treasure_chest_attribute_asset_list);
    routers.add(1069, controller::props_items::get_resources_props_list);
    routers.add(1070, controller::props_items::get_speed_up_props_list);
    routers.add(1071, controller::props_items::get_starlight_sculpture_props_list);
    routers.add(1072, controller::props_items::get_tome_of_knowledge_props_list);
    routers.add(1073, controller::props_malls::get_props_mall_asset_list);
    routers.add(1074, controller::quests::get_quests_metadata_list);
    routers.add(1075, controller::quests::get_quest_asset_list);
    routers.add(1076, controller::quests::get_quest_attribute_asset_list);
    routers.add(1077, controller::quests::get_quests_relation_list);
    routers.add(1078, controller::servers::get_servers);
    routers.add(1079, controller::skill_relateds::get_skill_relation_list);
    routers.add(1080, controller::skills::get_skill_list);
    routers.add(1081, controller::versions::get_version_update_data);
    routers.add(1082, controller::vips::get_vip_buff_list);
    routers.add(1083, controller::vips::get_daily_login_treasure_chest_list);
    routers.add(1084, controller::vips::get_vip_level_list);
    routers.add(1085, controller::categories::get_category_metadata_list);

    Arc::new(routers)
}
