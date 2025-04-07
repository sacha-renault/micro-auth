use crate::_tests::setup_test_db;
use crate::core::errors::{ApiError, ValidationError};
use crate::core::DbPool;
use crate::scope::interfaces::ScopeCreationRequest;

use super::controller::*;

async fn create_test_scope(pool: &DbPool) -> Result<i64, ApiError> {
    let request = ScopeCreationRequest {
        name: "testscope".to_string(),
    };

    add_scope(request, pool).await
}

#[sqlx::test]
async fn test_add_scope() {
    let pool = setup_test_db().await;

    let scope_id = create_test_scope(&pool).await.unwrap();
    assert!(scope_id > 0);

    let scope = get_scope_by_id(scope_id, &pool).await.unwrap();
    assert_eq!(scope.name, "testscope");
    assert!(scope.is_active);
}

#[sqlx::test]
async fn test_add_scope_validation_name_too_short() {
    let pool = setup_test_db().await;

    let request = ScopeCreationRequest {
        name: "ab".to_string(), // Less than 3 characters
    };

    let result = add_scope(request, &pool).await.unwrap_err();
    assert!(matches!(
        result,
        ApiError::Validation(ValidationError::NameTooShort)
    ));
}

#[sqlx::test]
async fn test_add_scope_validation_name_too_long() {
    let pool = setup_test_db().await;

    let request = ScopeCreationRequest {
        name: "thisnameiswaytoolongforascopename".to_string(), // More than 24 characters
    };

    let result = add_scope(request, &pool).await.unwrap_err();
    assert!(matches!(
        result,
        ApiError::Validation(ValidationError::NameTooLong)
    ));
}

#[sqlx::test]
async fn test_add_scope_validation_invalid_characters() {
    let pool = setup_test_db().await;

    let request = ScopeCreationRequest {
        name: "invalid-scope!".to_string(), // Contains non-alphanumeric characters
    };

    let result = add_scope(request, &pool).await.unwrap_err();
    assert!(matches!(
        result,
        ApiError::Validation(ValidationError::InvalidCharacters)
    ));
}

#[sqlx::test]
async fn test_get_scope_by_id() {
    let pool = setup_test_db().await;
    let scope_id = create_test_scope(&pool).await.unwrap();

    let scope = get_scope_by_id(scope_id, &pool).await.unwrap();
    assert_eq!(scope.id, scope_id);
    assert_eq!(scope.name, "testscope");
}

#[sqlx::test]
async fn test_get_scope_by_id_not_found() {
    let pool = setup_test_db().await;
    let non_existent_id = 999;

    let result = get_scope_by_id(non_existent_id, &pool).await.unwrap_err();
    assert!(matches!(result, ApiError::NotFound(_)));
    if let ApiError::NotFound(err) = result {
        assert_eq!(
            err.0,
            format!("Scope with id {} not found", non_existent_id)
        );
    }
}

#[sqlx::test]
async fn test_get_scope_by_name() {
    let pool = setup_test_db().await;
    let _ = create_test_scope(&pool).await.unwrap();

    let scope = get_scope_by_name("testscope", &pool).await.unwrap();
    assert_eq!(scope.name, "testscope");
}

#[sqlx::test]
async fn test_get_scope_by_name_not_found() {
    let pool = setup_test_db().await;
    let non_existent_name = "nonexistentscope";

    let result = get_scope_by_name(non_existent_name, &pool)
        .await
        .unwrap_err();
    assert!(matches!(result, ApiError::NotFound(_)));
    if let ApiError::NotFound(err) = result {
        assert_eq!(
            err.0,
            format!("Scope with name {} not found", non_existent_name)
        );
    }
}
