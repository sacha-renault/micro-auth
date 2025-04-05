use crate::core::{errors::ApiError, DbPool};

use super::{controller, interfaces::UserCreationRequest, model::User};

pub async fn create_user(
    user_request: UserCreationRequest,
    pool: &DbPool,
) -> Result<User, ApiError> {
    match controller::get_user_by_email(&user_request.email, pool).await? {
        Some(user) => Err(ApiError::Conflict(
            format!("User with email {} already exists in database", user.email).into(),
        )),
        None => controller::create_user(user_request, pool).await,
    }
}

pub async fn get_user_by_email(email: &str, pool: &DbPool) -> Result<Option<User>, ApiError> {
    controller::get_user_by_email(email, pool).await
}

pub async fn get_user_by_id(id: i64, pool: &DbPool) -> Result<Option<User>, ApiError> {
    controller::get_user_by_id(id, pool).await
}
