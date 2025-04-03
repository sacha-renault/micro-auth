use sqlx::{Pool, Sqlite};

use super::interfaces::ServiceCreationRequest;
use super::model::Service;

use crate::core::errors;

fn validate_service_creation(
    service: &ServiceCreationRequest,
) -> Result<(), errors::ValidationError> {
    // TODO validate the service and return a result with
    // More specific Err
    return Ok(());
}

pub async fn add_service(
    service: ServiceCreationRequest,
    pool: &Pool<Sqlite>,
) -> Result<i64, errors::ApiError> {
    // Validate the service
    validate_service_creation(&service)?;

    // Insert the service and return it id
    let result = sqlx::query("INSERT INTO services (name) VALUES (?)")
        .bind(service.0)
        .execute(pool)
        .await?;

    // Get the last inserted ID
    let id = result.last_insert_rowid();
    Ok(id)
}

pub async fn get_service_by_id(id: i64, pool: &Pool<Sqlite>) -> Result<Service, errors::ApiError> {
    // Fetch from database
    let service = sqlx::query_as::<Sqlite, Service>("SELECT id, name FROM services WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    // Return Service directly in Ok variant or NotFoundError
    match service {
        Some(service) => Ok(service),
        None => Err(errors::NotFoundError(format!("Service with id {} not found", id)).into()),
    }
}

pub async fn get_service_by_name(
    name: String,
    pool: &Pool<Sqlite>,
) -> Result<Service, errors::ApiError> {
    // Fetch from database
    let service = sqlx::query_as::<Sqlite, Service>("SELECT id, name FROM services WHERE name = ?")
        .bind(&name)
        .fetch_optional(pool)
        .await?;

    // Return Service directly in Ok variant or NotFoundError
    match service {
        Some(service) => Ok(service),
        None => Err(errors::NotFoundError(format!("Service with name {} not found", name)).into()),
    }
}
