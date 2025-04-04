use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RoleType {
    /// Can do everything
    /// One user only is created as root
    /// When service start for the first time
    /// Only one that can create a service
    Root,

    /// Can do everything within a service
    Admin,

    /// Basic user
    User,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserRole {
    pub id: i64,
    pub user_id: i64,
    pub service_id: i64,
    pub role_type: RoleType,
}
