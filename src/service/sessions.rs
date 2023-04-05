use axum::async_trait;
use mockall::automock;
use sqlx::types::chrono::{Utc, NaiveDateTime, DateTime};
use tracing::{warn, error, info};

use crate::{domain::{users::{Credentials, User}, sessions::Session}, auth, constants::SESSION_LENGTH_SECONDS, repository::{sessions::{SessionRepository, SessionInsertError, SessionGetError}, users::{UserRepository, UserGetError}}};

use super::hash::HashService;

#[derive(PartialEq, Debug)]
pub enum LoginError {
    NoUser,
    Unknown
}

#[derive(PartialEq, Debug)]
pub enum SessionVerifyError {
    Missing,
    Unknown
}

#[automock]
#[async_trait]
pub trait SessionService {
    async fn login(&self, credentials: Credentials) -> Result<Session, LoginError>;
    async fn verify(&self, id: &str) -> Result<User, SessionVerifyError>;
}

#[derive(Debug, Clone)]
pub struct HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    session_repository: S,
    user_repository: U,
    hash_service: H
}

impl<S, U, H> HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    pub fn new(session_repository: S, user_repository: U, hash_service: H) -> Self {
        Self { session_repository, user_repository, hash_service }
    }
}

#[async_trait]
impl<S, U, H> SessionService for HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    #[tracing::instrument(skip_all, field(email = credentials.email))]
    async fn login(&self, credentials: Credentials) -> Result<Session, LoginError> {
        info!("Attempting to login user");
        let user = match self.user_repository.get_by_email(&credentials.email).await {
            Ok(user) => user,
            Err(UserGetError::Missing) => {
                warn!("Login attempt failed");
                return Err(LoginError::NoUser)
            },
            Err(UserGetError::Unknown) => return Err(LoginError::Unknown)
        };

        match self.hash_service.verify(&credentials.password, &user.password_hash) {
            Err(err) => {
                error!(%err);
                return Err(LoginError::Unknown);
            },
            Ok(false) => {
                warn!("Login attempt failed");
                return Err(LoginError::NoUser);
            },
            Ok(true) => ()
        };

        let session = Session {
            id: auth::generate_session_id(),
            user,
            expires: Utc::now().timestamp() + SESSION_LENGTH_SECONDS
        };

        match self.session_repository.insert(&session).await {
            Ok(()) => {
                info!("Login attempt succeeded");
                Ok(session)
            }
            Err(SessionInsertError::Unknown) => Err(LoginError::Unknown)
        }
    }

    async fn verify(&self, id: &str) -> Result<User, SessionVerifyError> {
        let session = match self.session_repository.get(id).await {
            Ok(session) => session,
            Err(SessionGetError::Missing) => return Err(SessionVerifyError::Missing),
            Err(SessionGetError::Unknown) => return Err(SessionVerifyError::Unknown)
        };

        let expires = match NaiveDateTime::from_timestamp_opt(session.expires, 0) {
            Some(expires) => expires,
            None => {
                return Err(match self.session_repository.delete(id).await {
                    Ok(()) => SessionVerifyError::Missing,
                    Err(_) => SessionVerifyError::Unknown
                });
            }
        };

        if DateTime::<Utc>::from_utc(expires, Utc) < Utc::now() {
            return Err(match self.session_repository.delete(id).await {
                Ok(()) => SessionVerifyError::Missing,
                Err(_) => SessionVerifyError::Unknown
            });
        }

        Ok(session.user)
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::{repository::{sessions::{MockSessionRepository, SessionDeleteError}, users::MockUserRepository}, service::hash::MockHashService, constants};

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

    fn mock_user() -> User {
        User {
            id: 1,
            email: mock_email(),
            password_hash: mock_hashed_password()
        }
    }

    fn mock_error() -> anyhow::Error {
        anyhow::Error::msg("mock_error")
    }

    fn mock_session_id() -> String {
        String::from("session_id")
    }

    fn mock_ok_session() -> Session {
        Session {
            id: mock_session_id(),
            user: mock_user(),
            expires: Utc::now().timestamp() + constants::SESSION_LENGTH_SECONDS
        }
    }

    fn mock_out_of_range_timestamp_session() -> Session {
        Session {
            id: mock_session_id(),
            user: mock_user(),
            expires: 1000*1000*1000*1000*1000
        }
    }

    fn mock_expired_timestamp_session() -> Session {
        Session {
            id: mock_session_id(),
            user: mock_user(),
            expires: Utc::now().timestamp() - 100*1000
        }
    }

    #[tokio::test]
    async fn hash_impl_login_normal() -> Result<(), LoginError> {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Ok(mock_user()));

        hash_service
            .expect_verify()
            .with(predicate::eq(mock_password()), predicate::eq(mock_hashed_password()))
            .times(1)
            .returning(|_, _| Ok(true));

        session_repository
            .expect_insert()
            .times(1)
            .returning(|_| Ok(()));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        let session = service.login(mock_credentials()).await?;
        assert_eq!(session.user, mock_user());

        Ok(())
    }

    #[tokio::test]
    async fn hash_impl_login_get_user_error_missing() {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Err(UserGetError::Missing));

        hash_service
            .expect_verify()
            .never();

        session_repository
            .expect_insert()
            .never();

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(LoginError::NoUser), service.login(mock_credentials()).await);
    }

    #[tokio::test]
    async fn hash_impl_login_get_user_error_unknown() {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Err(UserGetError::Unknown));

        hash_service
            .expect_verify()
            .never();

        session_repository
            .expect_insert()
            .never();

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(LoginError::Unknown), service.login(mock_credentials()).await);
    }

    #[tokio::test]
    async fn hash_impl_login_hash_verify_false() {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Ok(mock_user()));

        hash_service
            .expect_verify()
            .with(predicate::eq(mock_password()), predicate::eq(mock_hashed_password()))
            .times(1)
            .returning(|_, _| Ok(false));

        session_repository
            .expect_insert()
            .never();

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(LoginError::NoUser), service.login(mock_credentials()).await);
    }

    #[tokio::test]
    async fn hash_impl_login_hash_verify_error() {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Ok(mock_user()));

        hash_service
            .expect_verify()
            .with(predicate::eq(mock_password()), predicate::eq(mock_hashed_password()))
            .times(1)
            .returning(|_, _| Err(mock_error()));

        session_repository
            .expect_insert()
            .never();

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(LoginError::Unknown), service.login(mock_credentials()).await);
    }

    #[tokio::test]
    async fn hash_impl_login_session_insert_error() {
        let mut session_repository = MockSessionRepository::new();
        let mut user_repository = MockUserRepository::new();
        let mut hash_service = MockHashService::new();

        user_repository
            .expect_get_by_email()
            .with(predicate::eq(mock_email()))
            .times(1)
            .returning(|_| Ok(mock_user()));

        hash_service
            .expect_verify()
            .with(predicate::eq(mock_password()), predicate::eq(mock_hashed_password()))
            .times(1)
            .returning(|_, _| Ok(true));

        session_repository
            .expect_insert()
            .times(1)
            .returning(|_| Err(SessionInsertError::Unknown));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(LoginError::Unknown), service.login(mock_credentials()).await);
    }

    #[tokio::test]
    async fn hash_impl_verify_normal() {
        let mut session_repository = MockSessionRepository::new();
        let user_repository = MockUserRepository::new();
        let hash_service = MockHashService::new();

        session_repository
            .expect_get()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(mock_ok_session()));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Ok(mock_user()), service.verify(&mock_session_id()).await);
    }

    #[tokio::test]
    async fn hash_impl_verify_invalid_timestamp() {
        let mut session_repository = MockSessionRepository::new();
        let user_repository = MockUserRepository::new();
        let hash_service = MockHashService::new();

        session_repository
            .expect_get()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(mock_out_of_range_timestamp_session()));

        session_repository
            .expect_delete()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(()));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(SessionVerifyError::Missing), service.verify(&mock_session_id()).await);
    }

    #[tokio::test]
    async fn hash_impl_verify_invalid_timestamp_delete_error() {
        let mut session_repository = MockSessionRepository::new();
        let user_repository = MockUserRepository::new();
        let hash_service = MockHashService::new();

        session_repository
            .expect_get()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(mock_out_of_range_timestamp_session()));

        session_repository
            .expect_delete()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Err(SessionDeleteError::Unknown));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(SessionVerifyError::Unknown), service.verify(&mock_session_id()).await);
    }

    #[tokio::test]
    async fn hash_impl_verify_expired_timestamp() {
        let mut session_repository = MockSessionRepository::new();
        let user_repository = MockUserRepository::new();
        let hash_service = MockHashService::new();

        session_repository
            .expect_get()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(mock_expired_timestamp_session()));

        session_repository
            .expect_delete()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(()));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(SessionVerifyError::Missing), service.verify(&mock_session_id()).await);
    }

    #[tokio::test]
    async fn hash_impl_verify_expired_timestamp_delete_error() {
        let mut session_repository = MockSessionRepository::new();
        let user_repository = MockUserRepository::new();
        let hash_service = MockHashService::new();

        session_repository
            .expect_get()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Ok(mock_expired_timestamp_session()));

        session_repository
            .expect_delete()
            .with(predicate::eq(mock_session_id()))
            .times(1)
            .returning(|_| Err(SessionDeleteError::Unknown));

        let service = HashSessionService::new(session_repository, user_repository, hash_service);

        assert_eq!(Err(SessionVerifyError::Unknown), service.verify(&mock_session_id()).await);
    }
}
