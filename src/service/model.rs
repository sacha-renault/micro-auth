use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Service {
    /// Id of the service
    pub id: i64,

    /// Name of the service
    pub name: String,

    /// Creation date
    pub created_at: NaiveDateTime,

    /// When the service was last updated
    pub updated_at: Option<NaiveDateTime>,

    /// Whether the service is currently active
    pub is_active: bool,

    /// If the service requires an admin
    /// to create the new users
    pub requires_admin: bool,
}
