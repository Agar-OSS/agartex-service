use axum::async_trait;
use sqlx::PgPool;
use tracing::{error, info};

use crate::{domain::users::Credentials, constants};

pub enum UserCreationError {
    DuplicateEmail,
    Unknown
}

#[async_trait]
pub trait UserService {
    async fn register(&self, credentials: Credentials) -> Result<(), UserCreationError>;
}

#[derive(Debug, Clone)]
pub struct PgUserService {
    pool: PgPool,
    hash_cost: u32
}

impl PgUserService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: pool.clone(),
            hash_cost: constants::HASH_COST
        }
    }
}

#[async_trait]
impl UserService for PgUserService {
    #[tracing::instrument(skip(self, credentials), fields(email = credentials.email))]
    async fn register(&self, credentials: Credentials) -> Result<(), UserCreationError> {
        info!("Checking if user with email {} already exists...", credentials.email);
        let exists = sqlx::query("SELECT 1 FROM users WHERE email = $1")
            .bind(&credentials.email)
            .fetch_optional(&self.pool)
            .await;
        if let Ok(Some(_)) = exists {
            error!("User with email {} already exists!", credentials.email);
            return Err(UserCreationError::DuplicateEmail);
        } else if let Err(err) = exists {
            error!(%err);
            return Err(UserCreationError::Unknown);
        }
        
        let password_hash = match bcrypt::hash(credentials.password, self.hash_cost) {
            Ok(hash) => hash,
            Err(err) => {
                error!(%err);
                return Err(UserCreationError::Unknown);
            }
        };


        info!("Inserting user {} into database...", credentials.email);
        let result = sqlx::query("INSERT INTO users (email, password_hash) VALUES ($1, $2)")
            .bind(&credentials.email)
            .bind(&password_hash)
            .execute(&self.pool);
        match result.await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(%err);
                Err(UserCreationError::Unknown)
            }
        }
    }
}
