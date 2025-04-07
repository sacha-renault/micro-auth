use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Scope {
    /// Id of the scope
    pub id: i64,

    /// Name of the scope
    pub name: String,

    /// Creation date
    pub created_at: NaiveDateTime,

    /// When the scope was last updated
    pub updated_at: Option<NaiveDateTime>,

    /// Whether the scope is currently active
    pub is_active: bool,
}
