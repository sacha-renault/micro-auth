use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

use crate::core::errors::ApiError;
use crate::core::{jwt, DbPool};
use crate::role::model::{RoleType, UserRole};
use crate::user::model::User;
use crate::user::services;

#[derive(Debug)]
pub struct AuthenticatedUser {
    user: User,           // We be replaced by User when struct exists
    roles: Vec<UserRole>, // User can have many role depending on the scope
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract bearer token from Authorization header
        let token = req.headers().get_one("Authorization").and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header.trim_start_matches("Bearer ").trim())
            } else {
                None
            }
        });

        // Check if token exists
        let token = match token {
            Some(token) => token,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ApiError::Unauthorized(
                        "Missing or invalid authentication token".to_string().into(),
                    ),
                ));
            }
        };

        // Decode token
        let claims = match jwt::decode_token(token, jwt::SECRET) {
            Ok(claims) => claims,
            Err(err) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ApiError::Unauthorized(format!("Invalid authentication token {err}").into()),
                ));
            }
        };

        // Get the db managed state
        let pool = match req.guard::<&State<DbPool>>().await {
            Outcome::Success(pool) => pool,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    ApiError::Internal("Database connection error".to_string().into()),
                ));
            }
        };

        // Call user service to ensure the user exists
        let user = match services::get_user_by_id(claims.id, pool).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Outcome::Error((
                    Status::Unauthorized,
                    ApiError::Unauthorized(
                        format!("User with id {} does not exists", claims.id).into(),
                    ),
                ));
            }
            Err(err) => {
                return Outcome::Error((Status::InternalServerError, err));
            }
        };

        Outcome::Success(AuthenticatedUser {
            user,
            roles: vec![],
        })
    }
}

impl AuthenticatedUser {
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
