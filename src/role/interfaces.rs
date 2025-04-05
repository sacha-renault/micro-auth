use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use super::model::RoleType;

#[derive(Deserialize, Serialize, Constructor)]
pub struct RoleAssignRequest {
    pub target_user_id: i64,
    pub scope_id: i64,
    pub role: RoleType,
}
