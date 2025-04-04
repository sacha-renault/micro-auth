use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    sqlite::SqliteTypeInfo,
    Encode, Sqlite,
};

#[derive(Debug, Deserialize, Serialize, Clone, Encode)]
pub enum RoleType {
    /// Can do everything
    /// One user only is created as root
    /// When service start for the first time
    /// Only one that can create a service
    Root,

    /// Can do everything within a service
    Admin,

    /// Basic user
    User,
}

/// Impl type especially for SQLite, might have to change or use #[cfg(sqlite)]
impl Type<Sqlite> for RoleType {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type<Sqlite>>::compatible(ty)
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserRole {
    pub id: i64,
    pub user_id: i64,
    pub service_id: i64,
    pub role_type: RoleType,
}
