use crate::core::{errors::ApiError, from_request::AuthenticatedUser, DbPool};

use super::controller::{add_role, get_user_role_in_scope, update_role};
use super::interfaces::RoleAssignRequest;

pub async fn assign_role(
    user_assign_request: RoleAssignRequest,
    requesting_user: AuthenticatedUser, // The user making the request
    pool: &DbPool,
) -> Result<(), ApiError> {
    // unpack values from request
    let RoleAssignRequest {
        target_user_id,
        scope_id,
        role,
    } = user_assign_request;

    // Then, get the role of requesting user
    let requesting_user_role = requesting_user.role_in_scope(scope_id).ok_or_else(|| {
        ApiError::Unauthorized(format!("User : {}", requesting_user.user_id()).into())
    })?;

    // First, check if the requesting user has permission to assign roles
    if role.can_be_created_by(requesting_user_role) {
        // If authorized, check if user already has a role for this scope,
        // If he has, then we have to update instead of create
        let current_user_role = get_user_role_in_scope(target_user_id, scope_id, pool).await?;

        // Update or create
        if let Some(current_role) = current_user_role {
            update_role(current_role.id, role, pool).await?
        } else {
            add_role(target_user_id, scope_id, role, pool).await?;
        }

        Ok(())
    } else {
        Err(ApiError::Unauthorized(
            format!("User : {}", requesting_user.user_id()).into(),
        ))
    }
}
