use crate::core::{errors::ApiError, from_request::UserFromRequest, DbPool};

use super::controller::{add_role, get_user_role_in_service, update_role};
use super::interfaces::RoleAssignRequest;

pub async fn assign_role(
    user_assign_request: RoleAssignRequest,
    requesting_user: UserFromRequest, // The user making the request
    pool: &DbPool,
) -> Result<(), ApiError> {
    // unpack values from request
    let RoleAssignRequest {
        target_user_id,
        service_id,
        role,
    } = user_assign_request;

    // Then, get the role of requesting user
    let requesting_user_role = requesting_user.role_in_service(service_id).ok_or_else(|| {
        ApiError::Unauthorized(format!("User : {}", requesting_user.user_id()).into())
    })?;

    // First, check if the requesting user has permission to assign roles
    if role.can_by_created_by(requesting_user_role) {
        // If authorized, check if user already has a role for this service,
        // If he has, then we have to update instead of create
        let current_user_role = get_user_role_in_service(target_user_id, service_id, pool).await?;

        // Update or create
        if let Some(current_role) = current_user_role {
            update_role(current_role.id, role, pool).await?
        } else {
            add_role(target_user_id, service_id, role, pool).await?;
        }

        Ok(())
    } else {
        Err(ApiError::Unauthorized(
            format!("User : {}", requesting_user.user_id()).into(),
        ))
    }
}
