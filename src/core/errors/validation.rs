use derive_more::From;
use serde::Serialize;

#[derive(Debug, From, Serialize)]
pub enum ValidationError {
    NameTooShort,
    NameTooLong,
    InvalidCharacters,
}
