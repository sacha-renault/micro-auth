use sqlx::query;
use sqlx::sqlite::SqlitePool;

pub async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Create users table
    query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            first_name TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            name TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP,
            is_active INTEGER NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create scopes table
    query(
        r#"
        CREATE TABLE IF NOT EXISTS scopes (
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
        CREATE TABLE IF NOT EXISTS user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            scope_id INTEGER NOT NULL,
            role_type TEXT NOT NULL,
            FOREIGN KEY (scope_id) REFERENCES scopes(id),
            UNIQUE (user_id, scope_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create revoked_tokens table
    query(
        r#"
        CREATE TABLE IF NOT EXISTS revoked_tokens (
            token TEXT PRIMARY KEY,
            expiration_date TIMESTAMP NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // If everything went well
    Ok(())
}
