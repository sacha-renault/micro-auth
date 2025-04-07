mod controller;
pub mod interfaces;
pub mod model;
pub mod services;

#[cfg(test)]
mod tests;

use interfaces::RoleAssignRequest;
use model::{RoleType, UserRole};
use rocket::{serde::json::Json, Route, State};
use rocket_responder::{ok, ApiResponse};

use crate::core::{errors::ApiError, from_request::AuthenticatedUser, DbPool};

#[post("/create", data = "<user_role>")]
pub async fn create_role(
    user: AuthenticatedUser,
    user_role: Json<RoleAssignRequest>,
    pool: &State<DbPool>,
) -> ApiResponse<(), ApiError> {
    match services::assign_role(user_role.into_inner(), user, pool).await {
        Ok(_) => ok(()),
        Err(err) => ApiResponse::from(err),
    }
}

#[get("/user_role_in_scope/<scope_id>")]
pub async fn get_user_role_in_scope(
    scope_id: i64,
    user: AuthenticatedUser,
) -> ApiResponse<UserRole, ApiError> {
    // Case user is root,
    // It has for any scope the adm rights
    if user.is_root() {
        ok(UserRole {
            id: 0,
            user_id: user.user_id(),
            scope_id: scope_id,
            role_type: RoleType::Admin,
        })
    } else if let Some(role) = user.roles.iter().find(|scope| scope.id == scope_id) {
        ok(role.clone())
    } else {
        ApiResponse::from(ApiError::Unauthorized(
            format!("User {} is not authorized in this scope", user.user_id()).into(),
        ))
    }
}

pub fn routes() -> Vec<Route> {
    routes![create_role, get_user_role_in_scope]
}
