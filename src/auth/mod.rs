//! Auth module allow to create users and to authenticate

use rocket::{serde::json::Json, Route, State};
use rocket_responder::*;

use crate::{
    core::{errors::ApiError, DbPool},
    user::{interfaces::UserCreationRequest, model::User, services},
};

/// Returns all auth-related routes for mounting in the application
pub fn routes() -> Vec<Route> {
    rocket::routes![register_user]
}

#[post("/register", data = "<user_request>")]
pub async fn register_user(
    user_request: Json<UserCreationRequest>,
    pool: &State<DbPool>,
) -> ApiResponse<String, ApiError> {
    match services::create_user(user_request.into_inner(), pool).await {
        Ok(user) => created(format!("User {} {} was created with success!", user.first_name, user.name)),
        Err(err) => ApiResponse::from(err)
    }
}

#[post("/register", data = "<user_login>")]
pub async fn login_user(user_login: Json<UserCreationRequest>, pool: &State<DbPool>) -> ApiResponse<String, ApiError> {
    todo!()
}
