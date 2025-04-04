use crate::core::{errors::ApiError, password, DbPool};

use super::{
    interfaces::{UserChangeRequest, UserCreationRequest},
    model::User,
};
use chrono::Utc;

/// Create a new user in the database
pub async fn create_user(
    user_request: UserCreationRequest,
    pool: &DbPool,
) -> Result<User, ApiError> {
    // get now date
    let now = Utc::now().naive_utc();

    // unpack the request attrs
    let UserCreationRequest {
        email,
        first_name,
        name,
        password,
    } = user_request;

    // Hash the password
    let password_hash = password::hash(&password).map_err(|err| {
        ApiError::Internal(format!("Couldn't hash password, err : {}", err).into())
    })?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, first_name, name, password_hash, created_at, is_active) 
         VALUES (?, ?, ?, ?, ?, ?) 
         RETURNING *",
    )
    .bind(email)
    .bind(first_name)
    .bind(name)
    .bind(password_hash)
    .bind(now)
    .bind(true)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Get a user by ID
pub async fn get_user_by_id(id: i64, pool: &DbPool) -> Result<Option<User>, ApiError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Get a user by email
pub async fn get_user_by_email(email: &str, pool: &DbPool) -> Result<Option<User>, ApiError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Get all users
pub async fn get_all_users(pool: &DbPool) -> Result<Vec<User>, ApiError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

/// Get active users
pub async fn get_active_users(pool: &DbPool) -> Result<Vec<User>, ApiError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users WHERE is_active = TRUE")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

/// Update a user's information
pub async fn update_user(
    id: i64,
    user_change_request: UserChangeRequest,
    pool: &DbPool,
) -> Result<User, ApiError> {
    // First, fetch the current user
    let current_user = get_user_by_id(id, pool)
        .await?
        .ok_or_else(|| ApiError::from(sqlx::Error::RowNotFound))?;

    // Prepare update query with values or existing values if not provided
    let now = Utc::now().naive_utc();

    // Unpack value from request change
    let UserChangeRequest {
        email,
        first_name,
        name,
        password,
    } = user_change_request;

    // Hash the password
    let password_hash = match password {
        Some(password) => password::hash(&password).map_err(|err| {
            ApiError::Internal(format!("Couldn't hash password, err : {}", err).into())
        })?,
        None => current_user.password_hash,
    };

    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
         SET email = ?, 
             first_name = ?, 
             name = ?,
             password_hash = ?,
             updated_at = ?,
             is_active = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(email.unwrap_or(current_user.email))
    .bind(first_name.unwrap_or(current_user.first_name))
    .bind(name.unwrap_or(current_user.name))
    .bind(password_hash)
    .bind(now)
    .bind(current_user.is_active)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Delete a user by ID
pub async fn delete_user(id: i64, pool: &DbPool) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        return Err(ApiError::from(sqlx::Error::RowNotFound));
    }

    Ok(())
}

/// Deactivate a user (set is_active to false)
pub async fn deactivate_user(id: i64, pool: &DbPool) -> Result<User, ApiError> {
    let now = Utc::now().naive_utc();

    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
         SET is_active = FALSE,
             updated_at = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Activate a user (set is_active to true)
pub async fn activate_user(id: i64, pool: &DbPool) -> Result<User, ApiError> {
    let now = Utc::now().naive_utc();

    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
         SET is_active = TRUE,
             updated_at = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Validate user credentials
pub async fn validate_credentials(
    email: &str,
    password: &str,
    pool: &DbPool,
) -> Result<Option<User>, ApiError> {
    // Fetch the user by email
    let user = match get_user_by_email(email, pool).await? {
        Some(user) => user,
        None => return Ok(None),
    };

    // Verify the password
    if password::verify(password, &user.password_hash) {
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
