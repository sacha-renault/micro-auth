//! User services is basically a forward to controller
//! But I wanna stick to my impl where controller is private to the module
//! Therefore, i need a `service` module where i can call the controller

use crate::core::{errors::ApiError, DbPool};

use super::{controller, interfaces::UserCreationRequest, model::User};

pub async fn create_user(user_request: UserCreationRequest, pool: &DbPool) -> Result<User, ApiError> {
    controller::create_user(user_request, pool).await
}

pub async fn get_user_by_email(email: &str, pool: &DbPool) -> Result<Option<User>, ApiError> {
    controller::get_user_by_email(email, pool).await
}

pub async fn get_user_by_id(id: i64, pool: &DbPool) -> Result<Option<User>, ApiError> {
    controller::get_user_by_id(id, pool).await
}