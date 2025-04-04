use crate::core::from_request::UserFromRequest;

pub mod controller;
pub mod interfaces;
pub mod model;

#[post("/create")]
pub async fn create_role(user: UserFromRequest) {}
