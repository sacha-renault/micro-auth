use chrono::Utc;

use super::interfaces::ScopeCreationRequest;
use super::model::Scope;

use crate::core::{errors, DbPool};

/// Validates a scope creation request
///
/// # Parameters
/// * `scope` - The scope creation request to validate
///
/// # Returns
/// * `Result<(), errors::ValidationError>` - Ok if valid, ValidationError otherwise
fn validate_scope_creation(
    scope: &ScopeCreationRequest,
) -> Result<(), errors::ValidationError> {
    // get name
    let name = &scope.name;

    // Ensure long enough
    if name.len() < 3 {
        return Err(errors::ValidationError::NameTooShort);
    }

    // Ensure not too long
    if name.len() > 24 {
        return Err(errors::ValidationError::NameTooLong);
    }

    // Ensure no weird ass chars
    if !name.chars().all(|c| c.is_alphanumeric()) {
        return Err(errors::ValidationError::InvalidCharacters);
    }
    Ok(())
}

/// Creates a new scope in the database after validation
///
/// # Parameters
/// * `scope` - The scope creation request
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<i64, errors::ApiError>` - The ID of the newly created scope or an error
pub async fn add_scope(
    scope: ScopeCreationRequest,
    pool: &DbPool,
) -> Result<i64, errors::ApiError> {
    // Validate the scope
    validate_scope_creation(&scope)?;

    // Insert the scope and return it id
    let query_str =
        "INSERT INTO scopes (name, created_at, is_active) VALUES (?, ?, 1)";
    let result = sqlx::query(query_str)
        .bind(&scope.name)
        .bind(Utc::now().naive_utc())
        .execute(pool)
        .await?;

    // Get the last inserted ID
    let id = result.last_insert_rowid();
    Ok(id)
}

/// Retrieves a scope by its ID
///
/// # Parameters
/// * `id` - The scope ID to lookup
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<Scope, errors::ApiError>` - The scope if found or an error
pub async fn get_scope_by_id(id: i64, pool: &DbPool) -> Result<Scope, errors::ApiError> {
    // Fetch from database
    let scope = sqlx::query_as::<_, Scope>("SELECT * FROM scopes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    // Return Scope directly in Ok variant or NotFoundError
    match scope {
        Some(scope) => Ok(scope),
        None => Err(errors::NotFoundError(format!("Scope with id {} not found", id)).into()),
    }
}

/// Retrieves a scope by its name
///
/// # Parameters
/// * `name` - The scope name to lookup
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<Scope, errors::ApiError>` - The scope if found or an error
pub async fn get_scope_by_name(name: &str, pool: &DbPool) -> Result<Scope, errors::ApiError> {
    // Fetch from database
    let scope = sqlx::query_as::<_, Scope>("SELECT * FROM scopes WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    // Return Scope directly in Ok variant or NotFoundError
    match scope {
        Some(scope) => Ok(scope),
        None => Err(errors::NotFoundError(format!("Scope with name {} not found", name)).into()),
    }
}
