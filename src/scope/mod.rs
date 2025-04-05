mod controller;
mod interfaces;
mod model;

use rocket::serde::json::Json;
use rocket::{post, Route, State};
use rocket_responder::*;

use crate::core::errors::ApiError;
use crate::core::from_request::UserContext;
use crate::core::DbPool;

/// Returns all scope-related routes for mounting in the application
pub fn routes() -> Vec<Route> {
    rocket::routes![add_scope, get_scope_by_id, get_scope_by_name]
}

/// Creates a new scope in the database
///
/// # Parameters
/// * `scope_request` - JSON payload containing scope creation details
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<i64>` - Scope ID on success or error response on failure
#[post("/create", data = "<scope_request>")]
pub async fn add_scope(
    scope_request: Json<interfaces::ScopeCreationRequest>,
    user: UserContext,
    pool: &State<DbPool>,
) -> ApiResponse<i64, ApiError> {
    // Creation of scope requires root priviledge
    if !user.is_root() {
        return unauthorized(ApiError::Unauthorized(format!("Root priviledge are required").into()));
    }

    // Otherwise we can create it
    match controller::add_scope(scope_request.into_inner(), pool).await {
        Ok(id) => ok(id),
        Err(err) => ApiResponse::from(err),
    }
}

/// Retrieves a scope by its numeric ID
///
/// # Parameters
/// * `id` - Numeric identifier of the scope
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<model::Scope>` - Scope data on success or error response on failure
#[get("/id/<id>")]
pub async fn get_scope_by_id(
    id: i64,
    pool: &State<DbPool>,
) -> ApiResponse<model::Scope, ApiError> {
    match controller::get_scope_by_id(id, pool).await {
        Ok(scope) => ok(scope),
        Err(err) => ApiResponse::from(err),
    }
}

/// Retrieves a scope by its name
///
/// # Parameters
/// * `name` - Name of the scope to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<model::Scope>` - Scope data on success or error response on failure
#[get("/name/<name>")]
pub async fn get_scope_by_name(
    name: &str,
    pool: &State<DbPool>,
) -> ApiResponse<model::Scope, ApiError> {
    match controller::get_scope_by_name(name, pool).await {
        Ok(scope) => ok(scope),
        Err(err) => ApiResponse::from(err),
    }
}
