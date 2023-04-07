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

#[cfg(test)]
mod tests;
