use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Into)]
pub struct ServiceCreationRequest {
    pub name: String,
}
