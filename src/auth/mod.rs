//! Auth module allow to create users and to authenticate

use rocket::{serde::json::Json, Route, State};
use rocket_responder::ApiResponse;

use crate::{
    core::{errors::ApiError, DbPool},
    user::{interfaces::UserCreationRequest, model::User},
};

/// Returns all auth-related routes for mounting in the application
pub fn routes() -> Vec<Route> {
    rocket::routes![register_user]
}

#[post("/register", data = "<user_request>")]
pub async fn register_user(
    user_request: Json<UserCreationRequest>,
    pool: &State<DbPool>,
) -> ApiResponse<User, ApiError> {
    todo!()
}
