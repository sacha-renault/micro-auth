use rocket::{http::Status, serde::json::Json};
use rocket_responder::ApiResponse;

use super::errors::ApiError;

impl<T> From<ApiError> for ApiResponse<T, ApiError> {
    fn from(error: ApiError) -> Self {
        let status = match &error {
            ApiError::Database(_) => Status::InternalServerError,
            ApiError::Validation(_) => Status::BadRequest,
            ApiError::NotFound(_) => Status::NotFound,
            ApiError::Unauthorized(_) => Status::Unauthorized,
            ApiError::Internal(_) => Status::InternalServerError,
            ApiError::Conflict(_) => Status::Conflict,
        };

        ApiResponse::Err(status, Json(error))
    }
}
