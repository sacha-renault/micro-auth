use crate::_tests::setup_test_db;
use crate::core::errors::ApiError;
use crate::core::{password, DbPool};
use crate::user::interfaces::UserCreationRequest;

use super::model::User;
use super::services::*;

pub async fn create_test_user(pool: &DbPool) -> Result<User, ApiError> {
    let request = UserCreationRequest {
        email: "user@example.com".to_string(),
        first_name: "New".to_string(),
        name: "User".to_string(),
        password: "securepass123".to_string(),
    };

    create_user(request, pool).await
}

#[sqlx::test]
async fn test_create_user() {
    let pool = setup_test_db().await;

    let user = create_test_user(&pool).await.unwrap();

    assert_eq!(user.email, "user@example.com".to_string());
    assert_eq!(user.first_name, "New".to_string());
    assert_eq!(user.name, "User".to_string());
    assert!(password::verify(
        &"securepass123".to_string(),
        &user.password_hash
    ));
    assert!(user.is_active);
}

#[sqlx::test]
async fn test_create_user_conflict() {
    let pool = setup_test_db().await;
    let _ = create_test_user(&pool).await.unwrap();
    let result = create_test_user(&pool).await.unwrap_err();
    assert!(matches!(result, ApiError::Conflict(_)));
}

#[sqlx::test]
async fn test_get_user_by_email() {
    let pool = setup_test_db().await;
    let _ = create_test_user(&pool).await.unwrap();
    let user_opt = get_user_by_email("user@example.com", &pool).await.unwrap();
    assert!(user_opt.is_some());
}

#[sqlx::test]
async fn test_get_user_by_email_no_user() {
    let pool = setup_test_db().await;
    let user_opt = get_user_by_email("user@example.com", &pool).await.unwrap();
    assert!(user_opt.is_none());
}

#[sqlx::test]
async fn test_get_user_by_id() {
    let pool = setup_test_db().await;
    let user = create_test_user(&pool).await.unwrap();
    let user_opt = get_user_by_id(user.id, &pool).await.unwrap();
    assert!(user_opt.is_some());
}

#[sqlx::test]
async fn test_get_user_by_id_no_user() {
    let pool = setup_test_db().await;
    let user_opt = get_user_by_id(1, &pool).await.unwrap();
    assert!(user_opt.is_none());
}
