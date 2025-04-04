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
            is_active INTEGER NOT NULL DEFAULT 1,
            requires_admin INTERGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create a table that maps users to permissions for resources
    query(
        r#"
        CREATE TABLE IF NOT EXISTS user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            service_id INTEGER NOT NULL,
            role_type TEXT NOT NULL,
            FOREIGN KEY (service_id) REFERENCES services(id),
            UNIQUE (user_id, service_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // If everything went well
    Ok(())
}
