use chrono::Utc;

use super::interfaces::ServiceCreationRequest;
use super::model::Service;

use crate::core::{errors, DbPool, DbType};

/// Validates a service creation request
///
/// # Parameters
/// * `service` - The service creation request to validate
///
/// # Returns
/// * `Result<(), errors::ValidationError>` - Ok if valid, ValidationError otherwise
fn validate_service_creation(
    service: &ServiceCreationRequest,
) -> Result<(), errors::ValidationError> {
    // get name
    let name = &service.name;

    // Ensure long enough
    if name.len() < 3 {
        return Err(errors::ValidationError::NameTooShort);
    }

    // Ensure not too long
    if name.len() > 24 {
        return Err(errors::ValidationError::NameTooLong);
    }

    // Ensure no weird ass chars
    if !name.chars().all(|c| c.is_alphanumeric()) {
        return Err(errors::ValidationError::InvalidCharacters);
    }
    Ok(())
}

/// Creates a new service in the database after validation
///
/// # Parameters
/// * `service` - The service creation request
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<i64, errors::ApiError>` - The ID of the newly created service or an error
pub async fn add_service(
    service: ServiceCreationRequest,
    pool: &DbPool,
) -> Result<i64, errors::ApiError> {
    // Validate the service
    validate_service_creation(&service)?;

    // Insert the service and return it id
    let result = sqlx::query(
        "INSERT INTO services (name, permission_required, created_at) VALUES (?, ?, ?)",
    )
    .bind(service.name)
    .bind(service.permission_required)
    .bind(Utc::now().naive_utc())
    .execute(pool)
    .await?;

    // Get the last inserted ID
    let id = result.last_insert_rowid();
    Ok(id)
}

/// Retrieves a service by its ID
///
/// # Parameters
/// * `id` - The service ID to lookup
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<Service, errors::ApiError>` - The service if found or an error
pub async fn get_service_by_id(id: i64, pool: &DbPool) -> Result<Service, errors::ApiError> {
    // Fetch from database
    let service = sqlx::query_as::<DbType, Service>("SELECT * FROM services WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    // Return Service directly in Ok variant or NotFoundError
    match service {
        Some(service) => Ok(service),
        None => Err(errors::NotFoundError(format!("Service with id {} not found", id)).into()),
    }
}

/// Retrieves a service by its name
///
/// # Parameters
/// * `name` - The service name to lookup
/// * `pool` - SQLite connection pool
///
/// # Returns
/// * `Result<Service, errors::ApiError>` - The service if found or an error
pub async fn get_service_by_name(name: &str, pool: &DbPool) -> Result<Service, errors::ApiError> {
    // Fetch from database
    let service = sqlx::query_as::<DbType, Service>("SELECT * FROM services WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    // Return Service directly in Ok variant or NotFoundError
    match service {
        Some(service) => Ok(service),
        None => Err(errors::NotFoundError(format!("Service with name {} not found", name)).into()),
    }
}
