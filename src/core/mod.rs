use sqlx::{Pool, Sqlite};

pub mod errors;
pub mod from_request;
pub mod response;

pub type DbPool = Pool<Sqlite>;
pub type DbType = Sqlite;
