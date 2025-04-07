//! To respect architectural puprose
//! Controller is private, therefore,
//! services here just forward the controller functions

use crate::core::{errors::ApiError, DbPool};

use super::{controller, model::RevokedToken};

pub async fn is_token_revoked(token: &str, pool: &DbPool) -> Result<bool, ApiError> {
    Ok(controller::is_token_revoked(token, pool).await?)
}

pub async fn revoke_token(revoke: RevokedToken, pool: &DbPool) -> Result<(), ApiError> {
    Ok(controller::revoke_token(revoke, pool).await?)
}

pub async fn maintainance(pool: &DbPool) -> Result<(), ApiError> {
    Ok(controller::maintainance(pool).await?)
}
