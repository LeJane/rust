use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FrontDisplayVipBuff {
    pub id: i64,
    pub level: i64,
    pub buff_id: i64,
    pub is_new_mark: i16,
}
