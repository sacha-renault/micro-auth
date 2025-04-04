//! Auth is basically the service that links
//! Users, roles and service

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

#[post("/register/<service_id>", data = "<user_request>")]
pub async fn register_user(
    user_request: Json<UserCreationRequest>,
    service_id: i64,
    pool: &State<DbPool>,
) -> ApiResponse<User, ApiError> {
    // First, ensure service is okay creating users without admin rights
    todo!()
}
