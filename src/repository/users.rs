use axum::async_trait;
use mockall::automock;
use sqlx::PgPool;
use tracing::error;

use crate::domain::users::{User, Credentials};

pub enum UserGetError {
    Missing,
    Unknown
}

pub enum UserInsertError {
    Duplicate,
    Unknown
}

#[automock]
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
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    #[tracing::instrument(skip(self))]
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError> {
        let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(UserGetError::Missing),
            Err(err) => {
                error!(%err);
                Err(UserGetError::Unknown)
            }
        }
    }

    #[tracing::instrument(skip_all, fields(email = credentials.email))]
    async fn insert(&self, credentials: Credentials) -> Result<(), UserInsertError> {
        let result = sqlx::query(
            "INSERT INTO users (email, password_hash) 
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
        ")
            .bind(&credentials.email)
            .bind(&credentials.password)
            .execute(&self.pool)
            .await;

        match result {
            Ok(result) => {
                if result.rows_affected() > 0 { Ok(()) } else { Err(UserInsertError::Duplicate) }
            },
            Err(err) => {
                error!(%err);
                Err(UserInsertError::Unknown)
            }
        }
    }
}

