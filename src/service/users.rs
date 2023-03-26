use axum::async_trait;
use tracing::{error, info};

use crate::{domain::users::Credentials, constants, repository::users::{UserInsertError, UserRepository, PgUserRepository}};

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
    repository: PgUserRepository,
    hash_cost: u32
}

impl PgUserService {
    pub fn new(repository: PgUserRepository) -> Self {
        Self {
            repository,
            hash_cost: constants::HASH_COST
        }
    }
}

#[async_trait]
impl UserService for PgUserService {
    #[tracing::instrument(skip(self, credentials), fields(email = credentials.email))]
    async fn register(&self, credentials: Credentials) -> Result<(), UserCreationError> {
        info!("Attempting to register user");
        let password_hash = match bcrypt::hash(credentials.password, self.hash_cost) {
            Ok(hash) => hash,
            Err(err) => {
                error!(%err);
                return Err(UserCreationError::Unknown);
            }
        };

        match self.repository.insert(Credentials { email: credentials.email, password: password_hash}).await {
            Ok(_) => {
                info!("Registration attempt succeeded");
                Ok(())
            },
            Err(UserInsertError::Duplicate) => Err(UserCreationError::DuplicateEmail),
            Err(UserInsertError::Unknown) => Err(UserCreationError::Unknown)
        }
    }
}
