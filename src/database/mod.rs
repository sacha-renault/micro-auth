use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;

use crate::core::DbPool;

pub async fn open_or_create_db(path: &str) -> Result<DbPool, sqlx::Error> {
    // Ensure parent directory exists
    if let Some(parent) = Path::new(path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        }
    }

    // Configure SQLite connection options
    let options =
        SqliteConnectOptions::from_str(&format!("sqlite:{}", path))?.create_if_missing(true);

    // Create the connection pool with options
    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
