use crate::core::{errors::ApiError, DbPool};

use super::model::{RoleType, UserRole};

pub async fn add_role(
    user_id: i64,
    scope_id: i64,
    role: RoleType,
    pool: &DbPool,
) -> Result<(), ApiError> {
    sqlx::query("INSERT INTO user_roles (user_id, scope_id, role_type) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(scope_id)
        .bind(role) // This assumes RoleType implements sqlx::Type
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_role(role_id: i64, role: RoleType, pool: &DbPool) -> Result<(), ApiError> {
    let result = sqlx::query("UPDATE user_roles SET role_type = ? WHERE id = ?")
        .bind(role)
        .bind(role_id)
        .execute(pool)
        .await?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        // No role found to update, return an error or handle accordingly
        return Err(ApiError::from(sqlx::Error::RowNotFound));
    }

    Ok(())
}

pub async fn delete_role(user_id: i64, scope_id: i64, pool: &DbPool) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM user_roles WHERE user_id = ? AND scope_id = ?")
        .bind(user_id)
        .bind(scope_id)
        .execute(pool)
        .await?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        // No role found to delete, return an error or handle accordingly
        return Err(ApiError::from(sqlx::Error::RowNotFound));
    }

    Ok(())
}

pub async fn get_user_role_in_scope(
    user_id: i64,
    scope_id: i64,
    pool: &DbPool,
) -> Result<Option<UserRole>, ApiError> {
    let result = sqlx::query_as::<_, UserRole>(
        "SELECT * FROM user_roles WHERE user_id = ? AND scope_id = ?",
    )
    .bind(user_id)
    .bind(scope_id)
    .fetch_optional(pool)
    .await?;

    Ok(result)
}
