use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Service {
    /// Id of the service
    pub id: i64,

    /// Name of the service
    pub name: String,

    /// Permission required to create a new
    /// in this service
    pub permission_required: i64,

    /// Creation date
    created_at: NaiveDateTime,

    /// When the service was last updated
    pub updated_at: Option<NaiveDateTime>,

    /// Whether the service is currently active
    pub is_active: bool,
}
