use mockall::predicate;

use crate::{service::hash::MockHashService, repository::users::MockUserRepository};

use super::*;

fn mock_email() -> String {
    String::from("password")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_hashed_password() -> String {
    String::from("hashed_password")
}

fn mock_credentials() -> Credentials {
    Credentials {
        email: mock_email(),
        password: mock_password()
    }
}

fn mock_hashed_credentials() -> Credentials {
    Credentials {
        email: mock_email(),
        password: mock_hashed_password()
    }
}

fn mock_error() -> anyhow::Error {
    anyhow::Error::msg("mock_error")
}

#[tokio::test]
async fn hash_impl_register_normal() {
    let mut repository = MockUserRepository::new();
    let mut hash_service = MockHashService::new();

    hash_service
        .expect_hash()
        .with(predicate::eq(mock_password()))
        .times(1)
        .returning(|_| Ok(mock_hashed_password()));

    repository
        .expect_insert()
        .with(predicate::eq(mock_hashed_credentials()))
        .times(1)
        .returning(|_| Ok(()));

    let service = HashUserService::new(repository, hash_service);

    assert_eq!(Ok(()), service.register(mock_credentials()).await);
}

#[tokio::test]
async fn hash_impl_register_hashing_error() {
    let mut repository = MockUserRepository::new();
    let mut hash_service = MockHashService::new();

    hash_service
        .expect_hash()
        .with(predicate::eq(mock_password()))
        .times(1)
        .returning(|_| Err(mock_error()));

    repository
        .expect_insert()
        .never();

    let service = HashUserService::new(repository, hash_service);

    assert_eq!(Err(UserCreationError::Unknown), service.register(mock_credentials()).await);
}

#[tokio::test]
async fn hash_impl_register_insertion_error_duplicate() {
    let mut repository = MockUserRepository::new();
    let mut hash_service = MockHashService::new();

    hash_service
        .expect_hash()
        .with(predicate::eq(mock_password()))
        .times(1)
        .returning(|_| Ok(mock_hashed_password()));

    repository
        .expect_insert()
        .with(predicate::eq(mock_hashed_credentials()))
        .times(1)
        .returning(|_| Err(UserInsertError::Duplicate));

    let service = HashUserService::new(repository, hash_service);

    assert_eq!(Err(UserCreationError::DuplicateEmail), service.register(mock_credentials()).await);
}

#[tokio::test]
async fn hash_impl_register_insertion_error_unknown() {
    let mut repository = MockUserRepository::new();
    let mut hash_service = MockHashService::new();

    hash_service
        .expect_hash()
        .with(predicate::eq(mock_password()))
        .times(1)
        .returning(|_| Ok(mock_hashed_password()));

    repository
        .expect_insert()
        .with(predicate::eq(mock_hashed_credentials()))
        .times(1)
        .returning(|_| Err(UserInsertError::Unknown));

    let service = HashUserService::new(repository, hash_service);

    assert_eq!(Err(UserCreationError::Unknown), service.register(mock_credentials()).await);
}
