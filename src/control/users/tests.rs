use axum::Extension;
use http::StatusCode;
use mockall::predicate;

use crate::{service::users::MockUserService, domain::users::Credentials, validation::ValidatedJson};

use super::*;

fn mock_email() -> String {
    String::from("email")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_credentials() -> Credentials {
    Credentials {
        email: mock_email(),
        password: mock_password()
    }
}

#[tokio::test]
async fn post_users_normal() {
    let mut user_service = MockUserService::new();

    user_service
        .expect_register()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Ok(()));

    assert_eq!(Ok(StatusCode::CREATED), post_users(Extension(user_service), ValidatedJson(mock_credentials())).await)
}

#[tokio::test]
async fn post_users_duplicate_error() {
    let mut user_service = MockUserService::new();

    user_service
        .expect_register()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(UserCreationError::DuplicateEmail));

    assert_eq!(Err(StatusCode::CONFLICT), post_users(Extension(user_service), ValidatedJson(mock_credentials())).await)
}

#[tokio::test]
async fn post_users_service_unknown_error() {
    let mut user_service = MockUserService::new();

    user_service
        .expect_register()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(UserCreationError::Unknown));

    assert_eq!(Err(StatusCode::INTERNAL_SERVER_ERROR), post_users(Extension(user_service), ValidatedJson(mock_credentials())).await)
}
