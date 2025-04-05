use derive_more::Into;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Into)]
pub struct ScopeCreationRequest {
    /// Name of the scope
    pub name: String,
}
