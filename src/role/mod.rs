use interfaces::RoleAssignRequest;
use rocket::{serde::json::Json, Route, State};
use rocket_responder::{ok, ApiResponse};

use crate::core::{errors::ApiError, from_request::AuthenticatedUser, DbPool};

mod controller;
pub mod interfaces;
pub mod model;
pub mod services;

#[post("/create", data = "<user_assign_request>")]
pub async fn create_role(
    user: AuthenticatedUser,
    user_assign_request: Json<RoleAssignRequest>,
    pool: &State<DbPool>,
) -> ApiResponse<(), ApiError> {
    match services::assign_role(user_assign_request.into_inner(), user, pool).await {
        Ok(_) => ok(()),
        Err(err) => ApiResponse::from(err),
    }
}

pub fn routes() -> Vec<Route> {
    routes![create_role]
}
