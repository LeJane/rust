use crate::utils::binary_read_helper::*;
use crate::FrontDisplayMetaVersion;
use anyhow::{bail, Result};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PageAndId {
    pub page: u16,
    pub id: u64,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Id {
    pub id: u64,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct PlayerIdAndUid {
    pub uid: u64,
    pub pid: u64,
}

pub trait BinaryEncode {
    fn encode(&self) -> Result<Vec<u8>>;
}

pub trait BinaryDecode<'a> {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Self>
        where
            Self: std::marker::Sized;
}

impl<'a> BinaryDecode<'a> for Vec<i64> {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<Self> {
        let array_length = binary_read_i32(cursor)?;
        let mut list = Vec::new();

        if array_length > 0 {
            let mut len = 0;
            loop {
                if len >= array_length {
                    break;
                }

                let data = binary_read_i64(cursor)?;

                list.push(data);

                len += 8;
            }
        }

        Ok(list)
    }
}

impl<'a> BinaryDecode<'a> for Vec<i32> {
    fn decode(cursor: &mut Cursor<&'a [u8]>, _bytes: &'a [u8]) -> Result<Self> {
        let array_length = binary_read_i32(cursor)?;
        let mut list = Vec::new();

        if array_length > 0 {
            let mut len = 0;
            loop {
                if len >= array_length {
                    break;
                }

                let data = binary_read_i32(cursor)?;

                list.push(data);

                len += 4;
            }
        }

        Ok(list)
    }
}

impl<'a, T: BinaryDecode<'a>> BinaryDecode<'a> for Vec<T> {
    fn decode(cursor: &mut Cursor<&'a [u8]>, bytes: &'a [u8]) -> Result<Self> {
        let array_length = binary_read_i32(cursor)?;
        let mut list = Vec::new();

        if array_length > 0 {
            let mut len = 0;
            loop {
                len += 4;

                if len >= array_length {
                    break;
                }

                let item_length = binary_read_i32(cursor)?;

                let data = T::decode(cursor, &bytes)?;

                list.push(data);

                len += item_length;
            }
        }

        Ok(list)
    }
}

pub fn deserialize_binary<'a, E: BinaryDecode<'a> + BinaryEncode>(
    cursor: &mut Cursor<&'a [u8]>,
    bytes: &'a [u8],
) -> Result<E> {
    E::decode(cursor, bytes)
}

//empty str->"".
impl BinaryEncode for &str {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, 0)?;

        Ok(encoded)
    }
}

impl BinaryEncode for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>> {
        let encoded_length = self.len();

        let mut data = Vec::with_capacity(encoded_length + 4);

        binary_write_i32(&mut data, encoded_length as i32)?;

        data.extend(self);

        Ok(data)
    }
}

impl BinaryEncode for i64 {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i64(&mut encoded, *self)?;

        Ok(encoded)
    }
}

impl BinaryEncode for i32 {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        binary_write_i32(&mut encoded, *self)?;

        Ok(encoded)
    }
}

impl<T: BinaryEncode> BinaryEncode for Vec<T> {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();

        self.into_iter()
            .map(|e| e.encode().and_then(|v| Ok(encoded.extend_from_slice(&v))))
            .count();

        let mut res = Vec::with_capacity(encoded.len() + 4);

        binary_write_i32(&mut res, encoded.len() as i32)?;

        res.extend_from_slice(&encoded);

        Ok(res)
    }
}

pub trait MetadataInstance {
    fn get_table_id() -> Result<i32>;
    fn get_single_instance(conn: &PgConnection, id: i64) -> Result<MetadataTypeEnum>;
    fn get_instance_list(conn: &PgConnection) -> Result<FrontDisplayMetaVersion>;
}

#[derive(Debug)]
pub enum TableIdEnum {
    PropsMall = 2043,
    PropsMallAsset = 2042,
    ServerList = 2006,
    Server = 2007,
    BuffMetadata = 2028,
    EquipmentKind = 2030,
    Enemy = 2029,
    Equipment = 2031,
    GemRelated = 2032,
    Gem = 2033,
    Player = 2034,
    PropsActionPointsCategory = 2035,
    PropsBoostCategory = 2036,
    PropsBuilderRecruitmentCategory = 2037,
    PropsFixedTreasureChestCategory = 2038,
    PropsFixedTreasureChestCategoryAsset = 2039,
    PropsItemMetadata = 2040,
    PropsKeyCategory = 2041,
    PropsProductNumber = 2044,
    PropsRandomTreasureChestCategory = 2045,
    PropsRandomTreasureChestCategoryAsset = 2046,
    PropsRandomTreasureChestCategoryAttributeAsset = 2047,
    PropsResourcesCategory = 2048,
    PropsSpeedUpCategory = 2049,
    PropsStarlightSculptureCategory = 2050,
    PropsTomeOfKnowledgeCategory = 2051,
    QuestsAsset = 2052,
    QuestsAttributeAsset = 2053,
    QuestsMetadata = 2054,
    QuestsRelation = 2055,
    ShopVipMetadata = 2056,
    Shop = 2057,
    SkillFightRelated = 2058,
    Skill = 2059,
    VipBuff = 2060,
    VipDailyLoginTreasureChest = 2061,
    VipLevel = 2062,
    Category = 2063,
}


impl TableIdEnum {
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn from_i32(_v: i32) {}
}

use crate::models::{
    buff_metadatas::BuffMetadata, enemys::Enemy, equipment_kinds::EquipmentKind,
    equipments::Equipment, gem_relateds::GemRelated, gems::Gem, players::Player,
    props_action_points_categories::PropsActionPointsCategory,
    props_boost_categories::PropsBoostCategory,
    props_builder_recruitment_categories::PropsBuilderRecruitmentCategory,
    props_fixed_treasure_chest_categories::PropsFixedTreasureChestCategory,
    props_fixed_treasure_chest_category_assets::PropsFixedTreasureChestCategoryAsset,
    props_item_metadatas::PropsItemMetadata, props_key_categories::PropsKeyCategory,
    props_mall_assets::PropsMallAsset, props_malls::PropsMall,
    props_product_numbers::PropsProductNumber,
    props_random_treasure_chest_categories::PropsRandomTreasureChestCategory,
    props_random_treasure_chest_category_assets::PropsRandomTreasureChestCategoryAsset,
    props_random_treasure_chest_category_attribute_assets::PropsRandomTreasureChestCategoryAttributeAsset,
    props_resources_categories::PropsResourcesCategory,
    props_speed_up_categories::PropsSpeedUpCategory,
    props_starlight_sculpture_categories::PropsStarlightSculptureCategory,
    props_tome_of_knowledge_categories::PropsTomeOfKnowledgeCategory, quests_assets::QuestsAsset,
    quests_attribute_assets::QuestsAttributeAsset, quests_metadatas::QuestsMetadata,
    quests_relations::QuestsRelation, server_lists::ServerList,
    shop_vip_metadatas::ShopVipMetadata, shops::Shop, skill_fight_relateds::SkillFightRelated,
    skills::Skill, vip_buffs::VipBuff, vip_daily_login_treasure_chests::VipDailyLoginTreasureChest,
    vip_levels::VipLevel,
    servers::Server,
    categories::Category,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MetadataTypeEnum {
    I64(i64),
    PropsMall(PropsMall),
    PropsMallAsset(PropsMallAsset),
    ServerList(ServerList),
    Server(Server),
    Shop(Shop),
    ShopVipMetadata(ShopVipMetadata),
    BuffMetadata(BuffMetadata),
    EquipmentKind(EquipmentKind),
    Enemy(Enemy),
    Equipment(Equipment),
    GemRelated(GemRelated),
    Gem(Gem),
    Player(Player),
    PropsActionPointsCategory(PropsActionPointsCategory),
    PropsBoostCategory(PropsBoostCategory),
    PropsBuilderRecruitmentCategory(PropsBuilderRecruitmentCategory),
    PropsFixedTreasureChestCategory(PropsFixedTreasureChestCategory),
    PropsFixedTreasureChestCategoryAsset(PropsFixedTreasureChestCategoryAsset),
    PropsItemMetadata(PropsItemMetadata),
    PropsKeyCategory(PropsKeyCategory),
    PropsProductNumber(PropsProductNumber),
    PropsRandomTreasureChestCategory(PropsRandomTreasureChestCategory),
    PropsRandomTreasureChestCategoryAsset(PropsRandomTreasureChestCategoryAsset),
    PropsRandomTreasureChestCategoryAttributeAsset(PropsRandomTreasureChestCategoryAttributeAsset),
    PropsResourcesCategory(PropsResourcesCategory),
    PropsSpeedUpCategory(PropsSpeedUpCategory),
    PropsStarlightSculptureCategory(PropsStarlightSculptureCategory),
    PropsTomeOfKnowledgeCategory(PropsTomeOfKnowledgeCategory),
    QuestsAsset(QuestsAsset),
    QuestsAttributeAsset(QuestsAttributeAsset),
    QuestsMetadata(QuestsMetadata),
    QuestsRelation(QuestsRelation),
    SkillFightRelated(SkillFightRelated),
    Skill(Skill),
    VipBuff(VipBuff),
    VipDailyLoginTreasureChest(VipDailyLoginTreasureChest),
    VipLevel(VipLevel),
    Category(Category),
}


impl BinaryEncode for MetadataTypeEnum {
    fn encode(&self) -> Result<Vec<u8>> {
        match self {
            MetadataTypeEnum::I64(v) => v.encode(),
            MetadataTypeEnum::PropsMall(v) => v.encode(),
            MetadataTypeEnum::PropsMallAsset(v) => v.encode(),
            MetadataTypeEnum::ServerList(v) => v.encode(),
            MetadataTypeEnum::Server(v) => v.encode(),
            MetadataTypeEnum::Shop(v) => v.encode(),
            MetadataTypeEnum::ShopVipMetadata(v) => v.encode(),
            MetadataTypeEnum::BuffMetadata(v) => v.encode(),
            MetadataTypeEnum::EquipmentKind(v) => v.encode(),
            MetadataTypeEnum::Enemy(v) => v.encode(),
            MetadataTypeEnum::Equipment(v) => v.encode(),
            MetadataTypeEnum::GemRelated(v) => v.encode(),
            MetadataTypeEnum::Gem(v) => v.encode(),
            MetadataTypeEnum::Player(v) => v.encode(),
            MetadataTypeEnum::PropsActionPointsCategory(v) => v.encode(),
            MetadataTypeEnum::PropsBoostCategory(v) => v.encode(),
            MetadataTypeEnum::PropsBuilderRecruitmentCategory(v) => v.encode(),
            MetadataTypeEnum::PropsFixedTreasureChestCategory(v) => v.encode(),
            MetadataTypeEnum::PropsFixedTreasureChestCategoryAsset(v) => v.encode(),
            MetadataTypeEnum::PropsItemMetadata(v) => v.encode(),
            MetadataTypeEnum::PropsKeyCategory(v) => v.encode(),
            MetadataTypeEnum::PropsProductNumber(v) => v.encode(),
            MetadataTypeEnum::PropsRandomTreasureChestCategory(v) => v.encode(),
            MetadataTypeEnum::PropsRandomTreasureChestCategoryAsset(v) => v.encode(),
            MetadataTypeEnum::PropsRandomTreasureChestCategoryAttributeAsset(v) => v.encode(),
            MetadataTypeEnum::PropsResourcesCategory(v) => v.encode(),
            MetadataTypeEnum::PropsSpeedUpCategory(v) => v.encode(),
            MetadataTypeEnum::PropsStarlightSculptureCategory(v) => v.encode(),
            MetadataTypeEnum::PropsTomeOfKnowledgeCategory(v) => v.encode(),
            MetadataTypeEnum::QuestsAsset(v) => v.encode(),
            MetadataTypeEnum::QuestsAttributeAsset(v) => v.encode(),
            MetadataTypeEnum::QuestsMetadata(v) => v.encode(),
            MetadataTypeEnum::QuestsRelation(v) => v.encode(),
            MetadataTypeEnum::SkillFightRelated(v) => v.encode(),
            MetadataTypeEnum::Skill(v) => v.encode(),
            MetadataTypeEnum::VipBuff(v) => v.encode(),
            MetadataTypeEnum::VipDailyLoginTreasureChest(v) => v.encode(),
            MetadataTypeEnum::VipLevel(v) => v.encode(),
            MetadataTypeEnum::Category(v) => v.encode(),
        }
    }
}

impl<'a> MetadataTypeEnum {
    pub fn decode(
        cursor: &mut Cursor<&'a [u8]>,
        bytes: &'a [u8],
        action_type: i32,
        table_id: i32,
    ) -> Result<Self> {
        if action_type == 3 {
            let v = binary_read_i64(cursor)?;

            return Ok(MetadataTypeEnum::I64(v));
        }

        let v = match table_id {
            2043 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsMall = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsMall(v)
            }
            2042 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsMallAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsMallAsset(v)
            }
            2006 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: ServerList = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::ServerList(v)
            }
            2007 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Server = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Server(v)
            }
            2028 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: BuffMetadata = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::BuffMetadata(v)
            }
            2030 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: EquipmentKind = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::EquipmentKind(v)
            }
            2029 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Enemy = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Enemy(v)
            }
            2031 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Equipment = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Equipment(v)
            }
            2032 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: GemRelated = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::GemRelated(v)
            }
            2033 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Gem = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Gem(v)
            }
            2034 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Player = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Player(v)
            }
            2035 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsActionPointsCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsActionPointsCategory(v)
            }
            2036 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsBoostCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsBoostCategory(v)
            }
            2037 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsBuilderRecruitmentCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsBuilderRecruitmentCategory(v)
            }
            2038 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsFixedTreasureChestCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsFixedTreasureChestCategory(v)
            }
            2039 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsFixedTreasureChestCategoryAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsFixedTreasureChestCategoryAsset(v)
            }
            2040 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsItemMetadata = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsItemMetadata(v)
            }
            2041 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsKeyCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsKeyCategory(v)
            }
            2044 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsProductNumber = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsProductNumber(v)
            }
            2045 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsRandomTreasureChestCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsRandomTreasureChestCategory(v)
            }
            2046 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsRandomTreasureChestCategoryAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsRandomTreasureChestCategoryAsset(v)
            }
            2047 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsRandomTreasureChestCategoryAttributeAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsRandomTreasureChestCategoryAttributeAsset(v)
            }
            2048 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsResourcesCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsResourcesCategory(v)
            }
            2049 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsSpeedUpCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsSpeedUpCategory(v)
            }
            2050 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsStarlightSculptureCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsStarlightSculptureCategory(v)
            }
            2051 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: PropsTomeOfKnowledgeCategory = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::PropsTomeOfKnowledgeCategory(v)
            }
            2052 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: QuestsAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::QuestsAsset(v)
            }
            2053 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: QuestsAttributeAsset = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::QuestsAttributeAsset(v)
            }
            2054 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: QuestsMetadata = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::QuestsMetadata(v)
            }
            2055 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: QuestsRelation = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::QuestsRelation(v)
            }
            2056 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: ShopVipMetadata = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::ShopVipMetadata(v)
            }
            2057 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Shop = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Shop(v)
            }
            2058 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: SkillFightRelated = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::SkillFightRelated(v)
            }
            2059 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Skill = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Skill(v)
            }
            2060 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: VipBuff = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::VipBuff(v)
            }
            2061 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: VipDailyLoginTreasureChest = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::VipDailyLoginTreasureChest(v)
            }
            2062 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: VipLevel = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::VipLevel(v)
            }
            2063 => {
                let _item_length = binary_read_i32(cursor)?;
                let v: Category = deserialize_binary(cursor, bytes)?;
                MetadataTypeEnum::Category(v)
            }
            _ => bail!("UnKnown table id."),
        };

        Ok(v)
    }
}
