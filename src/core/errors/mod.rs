mod validation;

use derive_more::From;
use serde::Serialize;

// use + reexport Error
pub use validation::ValidationError;

#[derive(Debug, From, Serialize)]
pub enum ApiError {
    Database(SerializableDbError),

    Validation(ValidationError),

    NotFound(NotFoundError),

    Unauthorized(UnauthorizedError),

    Internal(InternalError),

    Conflict(ConflictError),
}

#[derive(Serialize, Debug, From)]
pub struct NotFoundError(pub String);

#[derive(Serialize, Debug, From)]
pub struct UnauthorizedError(pub String);

#[derive(Serialize, Debug, From)]
pub struct InternalError(pub String);

#[derive(Serialize, Debug, From)]
pub struct ConflictError(pub String);

#[derive(Serialize, Debug, From)]
pub struct SerializableDbError(pub String);

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::Database(SerializableDbError(format!(
            "Database error: {}",
            error.to_string()
        )))
    }
}
