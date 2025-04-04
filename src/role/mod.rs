use rocket::Route;
use rocket_responder::ApiResponse;

use crate::core::{errors::ApiError, from_request::UserFromRequest};

mod controller;
mod interfaces;
pub mod model;
pub mod services;

#[post("/create")]
pub async fn create_role(user: UserFromRequest) -> ApiResponse<(), ApiError> {
    todo!()
}

pub fn routes() -> Vec<Route> {
    routes![create_role]
}
