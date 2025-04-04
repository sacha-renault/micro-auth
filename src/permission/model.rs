use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum ResourceType {
    /// Permission to create, see or do anything with a service
    #[default]
    Service,

    /// Permission to see and create users
    User,
    //
    //
    // ... More to come later
}

#[derive(Debug, Clone, Default)]
pub struct Resource {
    pub id: i64,
    pub rtype: ResourceType,
}

#[derive(Debug, Builder, Default)]
pub struct AllowedActions {
    pub resource: Resource,

    #[builder(default = false)]
    pub read: bool,

    #[builder(default = false)]
    pub write: bool, // eq. modify

    #[builder(default = false)]
    pub create: bool,

    #[builder(default = false)]
    pub delete: bool,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Permission {
    /// Id of the service
    pub id: i64,

    /// User id the permission is granted to
    pub user_id: i64,

    /// Resource type
    pub resource_type: ResourceType,

    /// Id of the resource
    pub resource_id: i64,

    /// If user can read the resource
    pub can_read: i64,

    /// If user can write the resource
    pub can_write: i64,

    /// If user can create the resource
    pub can_create: i64,

    /// If user can delete the resource
    pub can_delete: i64,
}
