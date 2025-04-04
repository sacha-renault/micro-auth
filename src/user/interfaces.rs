use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserCreationRequest {
    /// Name of the user
    pub email: String,

    /// First name of user
    pub first_name: String,

    /// Family name of user
    pub name: String,

    /// Hash of the password
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserChangeRequest {
    /// Name of the user
    pub email: Option<String>,

    /// First name of user
    pub first_name: Option<String>,

    /// Family name of user
    pub name: Option<String>,

    /// Hash of the password
    pub password: Option<String>,
}
