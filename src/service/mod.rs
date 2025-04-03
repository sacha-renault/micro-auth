pub mod controller;
pub mod interfaces;
pub mod model;

use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_responder::ApiResponse;
use rocket_responder::*;
use sqlx::{Pool, Sqlite};

use crate::core::errors::{ApiError, *};

#[post("/add_serice", data = "<service_request>")]
pub async fn add_service(
    service_request: Json<interfaces::ServiceCreationRequest>,
    pool: &State<Pool<Sqlite>>,
) -> ApiResponse<i64, ApiError> {
    match controller::add_service(service_request.0, pool).await {
        Ok(id) => ok(id),
        Err(ApiError::Database(err)) => internal_server_error(ApiError::Database(err)),
        _ => internal_server_error(ApiError::Internal("Unknown error".to_string().into())),
    }
}
