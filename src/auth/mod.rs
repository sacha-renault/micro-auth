//! Auth module allow to create users and to authenticate

pub mod interfaces;

use rocket::{serde::json::Json, Route, State};
use rocket_responder::*;

use interfaces::{AccessToken, UserLogin};

use crate::{
    core::{errors::ApiError, from_request::AuthenticatedUser, jwt, password, DbPool},
    revoked_token::{model::RevokedToken, services as token_services},
    user::{interfaces::UserCreationRequest, services as user_services},
};

/// Returns all auth-related routes for mounting in the application
pub fn routes() -> Vec<Route> {
    rocket::routes![register_user, login_user]
}

#[post("/register", data = "<user_request>")]
pub async fn register_user(
    user_request: Json<UserCreationRequest>,
    pool: &State<DbPool>,
) -> ApiResponse<String, ApiError> {
    match user_services::create_user(user_request.into_inner(), pool).await {
        Ok(user) => created(format!(
            "User {} {} was created with success!",
            user.first_name, user.name
        )),
        Err(err) => ApiResponse::from(err),
    }
}

#[post("/login", data = "<user_login>")]
pub async fn login_user(
    user_login: Json<UserLogin>,
    pool: &State<DbPool>,
) -> ApiResponse<AccessToken, ApiError> {
    // Get the user associated to the email
    let user = match user_services::get_user_by_email(&user_login.email, pool).await {
        // If there is a match, we will use this user
        Ok(Some(user)) => user,

        // If there is no match, we return unauthorized, shouldn't give any inforamtion about
        // it's the email that is wrong
        Ok(None) => {
            return unauthorized(ApiError::Unauthorized(
                format!("Wrong email or password").into(),
            ))
        }

        // The db just fucked up
        Err(err) => return ApiResponse::from(err),
    };

    // check if the pwd is correct
    if password::verify(&user_login.password, &user.password_hash) {
        // Password is verified, we will use our jwt function to create a token for this User
        match jwt::encode_token(user.id, jwt::SECRET) {
            Ok(token) => ok(AccessToken::new(token)),
            Err(err) => internal_server_error(ApiError::Internal(
                format!("Error creating the user token ... {err}").into(),
            )),
        }
    } else {
        unauthorized(ApiError::Unauthorized(
            format!("Wrong email or password").into(),
        ))
    }
}

#[post("/revoke")]
pub async fn revoke_token(
    user: AuthenticatedUser,
    pool: &State<DbPool>,
) -> ApiResponse<String, ApiError> {
    // Make the token to revoke
    let token_to_revoke = RevokedToken {
        token: user.token,
        expiration_date: user.expires_at,
    };

    // Call service to cancel it
    match token_services::revoke_token(token_to_revoke, pool).await {
        Ok(_) => ok("Token revoked successfully".to_string()),
        Err(err) => ApiResponse::from(err),
    }
}
