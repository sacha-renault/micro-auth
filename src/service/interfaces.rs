use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Into)]
pub struct ServiceCreationRequest {
    /// Name of the service
    pub name: String,

    /// If the service requires an admin
    /// to create the new users
    pub requires_admin: bool,
}
