use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::prelude::FromRow;

#[derive(Deserialize, Debug, FromRow)]
pub struct RevokedToken {
    pub token: String,
    pub expiration_date: NaiveDateTime,
}
