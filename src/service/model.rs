use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Service {
    pub id: i64,
    pub name: String,
}
