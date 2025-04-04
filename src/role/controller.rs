use crate::core::{errors::ApiError, DbPool};

use super::model::RoleType;

pub async fn add_role(
    user_id: i64,
    service_id: i64,
    role: RoleType,
    pool: &DbPool,
) -> Result<(), ApiError> {
    sqlx::query("INSERT INTO user_roles (user_id, service_id, role_type) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(service_id)
        .bind(role) // This assumes RoleType implements sqlx::Type
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_role(
    user_id: i64,
    service_id: i64,
    role: RoleType,
    pool: &DbPool,
) -> Result<(), ApiError> {
    let result =
        sqlx::query("UPDATE user_roles SET role_type = ? WHERE user_id = ? AND service_id = ?")
            .bind(role)
            .bind(user_id)
            .bind(service_id)
            .execute(pool)
            .await?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        // No role found to update, return an error or handle accordingly
        return Err(ApiError::from(sqlx::Error::RowNotFound));
    }

    Ok(())
}

pub async fn delete_role(user_id: i64, service_id: i64, pool: &DbPool) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM user_roles WHERE user_id = ? AND service_id = ?")
        .bind(user_id)
        .bind(service_id)
        .execute(pool)
        .await?;

    // Check if any row was affected
    if result.rows_affected() == 0 {
        // No role found to delete, return an error or handle accordingly
        return Err(ApiError::from(sqlx::Error::RowNotFound));
    }

    Ok(())
}
