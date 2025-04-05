use sqlx::{Pool, Sqlite};

pub mod errors;
pub mod from_request;
pub mod password;
pub mod response;
pub mod jwt;

pub type DbPool = Pool<Sqlite>;
pub type DbType = Sqlite;
