use axum::async_trait;
use mockall::automock;
use tracing::{error, info};

use crate::{domain::users::Credentials, repository::users::{UserInsertError, UserRepository}};

use super::hash::HashService;

#[derive(PartialEq, Debug)]
pub enum UserCreationError {
    DuplicateEmail,
    Unknown
}

#[automock]
#[async_trait]
pub trait UserService {
    async fn register(&self, credentials: Credentials) -> Result<(), UserCreationError>;
}

#[derive(Debug, Clone)]
pub struct HashUserService<U, H>
where
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    repository: U,
    hash_service: H
}

impl<U, H> HashUserService<U, H>
where
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    pub fn new(repository: U, hash_service: H) -> Self {
        Self {
            repository,
            hash_service
        }
    }
}

#[async_trait]
impl<U, H> UserService for HashUserService<U, H>
where
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    #[tracing::instrument(skip(self, credentials), fields(email = credentials.email))]
    async fn register(&self, credentials: Credentials) -> Result<(), UserCreationError> {
        info!("Attempting to register user");
        let password_hash = match self.hash_service.hash(&credentials.password) {
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


#[cfg(test)]
mod tests;
