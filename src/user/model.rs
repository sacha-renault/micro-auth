use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    /// Id of the user
    pub id: i64,

    /// Name of the user
    pub email: String,

    /// First name of user
    pub first_name: String,

    /// Hash of the password
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// Family name of user
    pub name: String,

    /// When the user was created
    pub created_at: NaiveDateTime,

    /// When the user was last updated
    pub updated_at: Option<NaiveDateTime>,

    /// Whether the user is currently active
    pub is_active: bool,
}
