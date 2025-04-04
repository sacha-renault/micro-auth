use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Into)]
pub struct ServiceCreationRequest {
    /// Name of the service
    pub name: String,
}
