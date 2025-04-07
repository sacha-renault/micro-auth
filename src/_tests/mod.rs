use sqlx::{Pool, Sqlite};

use crate::core::DbPool;
use crate::database::create_tables;

pub async fn setup_test_db() -> DbPool {
    let db_url = "sqlite::memory:";
    let pool = Pool::<Sqlite>::connect(db_url).await.unwrap();

    // Create users table for testing
    create_tables(&pool).await.unwrap();

    pool
}
