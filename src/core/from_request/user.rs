use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::core::errors::ApiError;
use crate::role::model::UserRole;

pub struct UserFromRequest {
    user: String,         // We be replaced by User when struct exists
    roles: Vec<UserRole>, // User can have many role depending on the service
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserFromRequest {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /* .. */
        Outcome::Error((
            Status::Unauthorized,
            ApiError::NotFound(format!("User not found").into()),
        ))
    }
}
