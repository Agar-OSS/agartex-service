use axum::async_trait;
use sqlx::PgPool;

use crate::domain::users::{User, Credentials};

pub enum UserGetError {
    Missing,
    Unknown
}

pub enum UserInsertError {
    Unknown
}

#[async_trait]
pub trait UserRepository {
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError>;
    async fn insert(&self, credentials: Credentials) -> Result<(), UserInsertError>;
}

#[derive(Debug, Clone)]
pub struct PgUserRepository {
    pub pool: PgPool
}


impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    #[tracing::instrument]
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                tracing::warn!("User with email {} does not exist!", email);
                Err(UserGetError::Missing)
            },
            Err(err) => {
                tracing::error!(%err);
                Err(UserGetError::Unknown)
            }
        }
    }

    #[tracing::instrument]
    async fn insert(&self, credentials: Credentials) -> Result<(), UserInsertError> {
        let result = sqlx::query("INSERT INTO users (email, password_hash) VALUES ($1, $2)")
            .bind(&credentials.email)
            .bind(&credentials.password)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!(%err);
                Err(UserInsertError::Unknown)
            }
        }
    }
}

