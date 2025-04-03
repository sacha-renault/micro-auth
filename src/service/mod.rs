pub mod controller;
pub mod interfaces;
pub mod model;

use rocket::serde::json::Json;
use rocket::{post, Route, State};
use rocket_responder::*;
use sqlx::{Pool, Sqlite};

use crate::core::errors::ApiError;

/// Returns all service-related routes for mounting in the application
pub fn routes() -> Vec<Route> {
    rocket::routes![add_service, get_service_by_id, get_service_by_name]
}

/// Creates a new service in the database
///
/// # Parameters
/// * `service_request` - JSON payload containing service creation details
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<i64>` - Service ID on success or error response on failure
#[post("/create", data = "<service_request>")]
pub async fn add_service(
    service_request: Json<interfaces::ServiceCreationRequest>,
    pool: &State<Pool<Sqlite>>,
) -> ApiResponse<i64, ApiError> {
    match controller::add_service(service_request.0, pool).await {
        Ok(id) => ok(id),
        Err(err) => ApiResponse::from(err),
    }
}

/// Retrieves a service by its numeric ID
///
/// # Parameters
/// * `id` - Numeric identifier of the service
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<model::Service>` - Service data on success or error response on failure
#[get("/id/<id>")]
pub async fn get_service_by_id(
    id: i64,
    pool: &State<Pool<Sqlite>>,
) -> ApiResponse<model::Service, ApiError> {
    match controller::get_service_by_id(id, pool).await {
        Ok(service) => ok(service),
        Err(err) => ApiResponse::from(err),
    }
}

/// Retrieves a service by its name
///
/// # Parameters
/// * `name` - Name of the service to retrieve
/// * `pool` - Database connection pool
///
/// # Returns
/// * `ApiResponse<model::Service>` - Service data on success or error response on failure
#[get("/name/<name>")]
pub async fn get_service_by_name(
    name: &str,
    pool: &State<Pool<Sqlite>>,
) -> ApiResponse<model::Service, ApiError> {
    match controller::get_service_by_name(name, pool).await {
        Ok(service) => ok(service),
        Err(err) => ApiResponse::from(err),
    }
}
