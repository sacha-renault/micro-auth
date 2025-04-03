mod validation;

use derive_more::From;
use serde::Serialize;

// use + reexport Error
pub use validation::ValidationError;

/// Represents all possible API errors
///
/// # Variants
/// * `Database` - Database-related errors
/// * `Validation` - Input validation errors
/// * `NotFound` - Resource not found errors
/// * `Unauthorized` - Authentication/authorization errors
/// * `Internal` - Internal server errors
/// * `Conflict` - Resource conflict errors
#[derive(Debug, From, Serialize)]
pub enum ApiError {
    Database(SerializableDbError),

    Validation(ValidationError),

    NotFound(NotFoundError),

    Unauthorized(UnauthorizedError),

    Internal(InternalError),

    Conflict(ConflictError),
}

/// Error returned when a requested resource is not found
#[derive(Serialize, Debug, From)]
pub struct NotFoundError(pub String);

/// Error returned for unauthorized access attempts
#[derive(Serialize, Debug, From)]
pub struct UnauthorizedError(pub String);

/// Error returned for internal server errors
#[derive(Serialize, Debug, From)]
pub struct InternalError(pub String);

/// Error returned when there's a resource conflict
#[derive(Serialize, Debug, From)]
pub struct ConflictError(pub String);

/// Wrapper for database errors to make them serializable
#[derive(Serialize, Debug, From)]
pub struct SerializableDbError(pub String);

/// Converts SQLx errors to ApiError
///
/// # Parameters
/// * `error` - The SQLx error to convert
///
/// # Returns
/// * `ApiError` - The converted API error
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::Database(SerializableDbError(format!(
            "Database error: {}",
            error.to_string()
        )))
    }
}
