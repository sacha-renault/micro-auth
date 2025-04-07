use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    sqlite::SqliteTypeInfo,
    Decode, Encode, Sqlite,
};

#[derive(Debug, Deserialize, Serialize, Clone, Encode, Decode)]
pub enum RoleType {
    /// Can do everything
    /// Has access to every scope
    /// Should actually not be use, unless to create
    /// Other admins
    Root,

    /// Can create or delete scopes
    Admin,

    /// Can edit resources in a certain scope
    Editor,

    /// Can just access a scope
    Member,
}

impl RoleType {
    pub fn can_be_created_by(&self, role: &RoleType) -> bool {
        match (self, role) {
            (Self::Root, _) => true,
            (Self::Admin, Self::Editor | Self::Member) => true,
            _ => false,
        }
    }
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

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct UserRole {
    pub id: i64,
    pub user_id: i64,
    pub scope_id: i64,
    pub role_type: RoleType,
}
