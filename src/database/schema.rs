use sqlx::query;
use sqlx::sqlite::SqlitePool;

pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create services table
    query(
        r#"
        CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP,
            is_active INTEGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create a table that maps users to permissions for resources
    query(
        r#"
        CREATE TABLE IF NOT EXISTS user_resource_permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            resource_type TEXT NOT NULL,
            resource_id INTEGER NOT NULL,
            can_read INTEGER NOT NULL DEFAULT 0,
            can_write INTEGER NOT NULL DEFAULT 0,
            can_create INTEGER NOT NULL DEFAULT 0,
            can_delete INTEGER NOT NULL DEFAULT 0,
            UNIQUE(user_id, resource_type, resource_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
