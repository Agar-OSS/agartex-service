use axum::async_trait;
use sqlx::PgPool;
use tracing::{warn, error, info};

use crate::domain::users::{Credentials, User};

pub enum SessionCreationError {
    NoUser,
    Unknown
}

#[async_trait]
pub trait SessionService {
    async fn login(&self, credentials: Credentials) -> Result<i32, SessionCreationError>;
}

#[derive(Debug, Clone)]
pub struct PgSessionService {
    pool: PgPool
}

impl PgSessionService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: pool.clone()
        }
    }
}

#[async_trait]
impl SessionService for PgSessionService {
    #[tracing::instrument(skip(self, credentials), field(email = credentials.email))]
    async fn login(&self, credentials: Credentials) -> Result<i32, SessionCreationError> {
        info!("Checking if user with email {} exists...", credentials.email);
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&credentials.email)
            .fetch_optional(&self.pool)
            .await;

        let user = match user {
            Ok(Some(user)) => user,
            Ok(None) => {
                warn!("User with email {} does not exist!", credentials.email);
                return Err(SessionCreationError::NoUser);
            },
            Err(err) => {
                error!(%err);
                return Err(SessionCreationError::Unknown);
            }
        };

        match bcrypt::verify(credentials.password, &user.password_hash) {
            Err(err) => {
                error!(%err);
                return Err(SessionCreationError::Unknown);
            },
            Ok(false) => {
                warn!("Password for user with email {} doesn't match!", credentials.email);
                return Err(SessionCreationError::NoUser);
            },
            Ok(true) => Ok(user.id)
        }
    }
}
