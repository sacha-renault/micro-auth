use sqlx::{Pool, Sqlite};

pub mod errors;
pub mod response;

pub type DbPool = Pool<Sqlite>;
