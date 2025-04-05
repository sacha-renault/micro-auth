use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Constructor)]
pub struct AccessToken {
    pub access_token: String,
}