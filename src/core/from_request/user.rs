use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::core::errors::ApiError;
use crate::role::model::{RoleType, UserRole};
use crate::user::model::User;

pub struct UserFromRequest {
    user: User,           // We be replaced by User when struct exists
    roles: Vec<UserRole>, // User can have many role depending on the scope
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
    pub fn is_root(&self) -> bool {
        self.user_id() == 1 // only one user is created with id 1, the root
    }

    pub fn is_admin_in_scope(&self, scope_id: i64) -> bool {
        // find the role for the scope
        let role_type = self.role_in_scope(scope_id);

        // Is the user admin ?
        match role_type {
            Some(role) => matches!(role, RoleType::Admin | RoleType::Root),
            None => false,
        }
    }

    pub fn is_user_in_scope(&self, scope_id: i64) -> bool {
        self.role_in_scope(scope_id).is_some() || self.is_root()
    }

    pub fn user_id(&self) -> i64 {
        self.user.id
    }

    pub fn role_in_scope(&self, scope_id: i64) -> Option<&RoleType> {
        self.roles
            .iter()
            .find(|role| role.scope_id == scope_id)
            .map(|user_role| &user_role.role_type)
    }
}
