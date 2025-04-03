pub mod controller;
pub mod interfaces;
pub mod model;

use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_responder::*;
use sqlx::{Pool, Sqlite};

use crate::core::errors::ApiError;

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
