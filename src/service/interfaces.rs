use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Into)]
pub struct ServiceCreationRequest {
    /// Name of the service
    pub name: String,

    /// Permission required to create a new
    /// in this service
    pub permission_required: i64,
}
