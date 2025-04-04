use crate::core::{errors::ApiError, from_request::UserFromRequest, DbPool};

use super::controller::add_role;
use super::model::RoleType;

pub async fn assign_role(
    requesting_user: UserFromRequest, // The user making the request
    target_user_id: i64,              // The user getting the role
    service_id: i64,                  // The service the user will be granted the role
    role: RoleType,
    pool: &DbPool,
) -> Result<(), ApiError> {
    // First, check if the requesting user has permission to assign roles
    if !requesting_user.is_admin(service_id) {
        return Err(ApiError::Unauthorized(
            format!("User : {}", requesting_user.user_id()).into(),
        ));
    }

    // If authorized, proceed with role assignment
    add_role(target_user_id, service_id, role, pool).await?;

    Ok(())
}
