use serde::{Deserialize, Serialize};

use super::model::RoleType;

#[derive(Deserialize, Serialize)]
pub struct RoleAssignRequest {
    pub target_user_id: i64,
    pub service_id: i64,
    pub role: RoleType,
}
