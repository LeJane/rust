use crate::default_log_pre;
use crate::facades::{user_item_bags, users};
use crate::utils::binary_read_helper::binary_read_i64;
use crate::{ReqContext, ResponseResult};
use anyhow::anyhow;
use function_name::named;
use tracing::info;

#[named]
pub async fn get_user_item_bag_list(req: ReqContext) -> ResponseResult {
    let slave_conn = req.db_conn(false)?;

    let mut cursor = std::io::Cursor::new(req.body.as_slice());
    let uid = binary_read_i64(&mut cursor)?;
    if uid <= 0 {
        return Err(anyhow!("invalid uuid."));
    }

    if !users::find_user_exists(&slave_conn, uid)? {
        return Err(anyhow!("user not exists."));
    }

    info!(
        "{}\tsubmit content\tuuid:{}",
        default_log_pre!(req.code, uid),
        uid
    );

    let list = user_item_bags::get_front_display_user_item_bag_list(&slave_conn, uid)?;

    req.get_bincode(200, "Success", list)
}
