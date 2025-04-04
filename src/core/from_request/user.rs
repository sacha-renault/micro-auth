use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::core::errors::ApiError;
use crate::role::model::{RoleType, UserRole};

pub struct UserFromRequest {
    user: String,         // We be replaced by User when struct exists
    roles: Vec<UserRole>, // User can have many role depending on the service
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserFromRequest {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Error((
            Status::Unauthorized,
            ApiError::NotFound(format!("User not found").into()),
        ))
    }
}

impl UserFromRequest {
    fn _find_role_for_service(&self, service_id: i64) -> Option<&UserRole> {
        self.roles.iter().find(|role| role.service_id == service_id)
    }

    pub fn is_admin(&self, service_id: i64) -> bool {
        // find the role for the service
        let role = self._find_role_for_service(service_id);

        // Is the user admin ?
        match role {
            Some(role) => matches!(role.role_type, RoleType::Admin | RoleType::Root),
            None => false,
        }
    }

    pub fn is_user_in(&self, service_id: i64) -> bool {
        self._find_role_for_service(service_id).is_some()
    }

    pub fn user_id(&self) -> i64 {
        0 // self.user.id;
    }
}
