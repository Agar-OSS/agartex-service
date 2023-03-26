use axum::async_trait;
use sqlx::types::chrono::{Utc, NaiveDateTime, DateTime};
use tracing::{warn, error, info};

use crate::{domain::{users::{Credentials, User}, sessions::Session}, auth, constants::SESSION_LENGTH_SECONDS, repository::{sessions::{SessionRepository, SessionInsertError, SessionGetError}, users::{UserRepository, UserGetError}}};

pub enum LoginError {
    NoUser,
    Unknown
}

pub enum SessionVerifyError {
    Missing,
    Unknown
}

#[async_trait]
pub trait SessionService {
    async fn login(&self, credentials: Credentials) -> Result<Session, LoginError>;
    async fn verify(&self, id: &str) -> Result<User, SessionVerifyError>;
}

#[derive(Debug, Clone)]
pub struct BcryptSessionService<S, U>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync
{
    session_repository: S,
    user_repository: U
}

impl<S, U> BcryptSessionService<S, U>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync
{
    pub fn new(session_repository: S, user_repository: U) -> Self {
        Self { session_repository, user_repository }
    }
}

#[async_trait]
impl<S, U> SessionService for BcryptSessionService<S, U>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync
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

        match bcrypt::verify(credentials.password, &user.password_hash) {
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
