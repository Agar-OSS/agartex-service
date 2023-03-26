use std::fmt::Debug;

use axum::Extension;
use hyper::StatusCode;
use tracing::info;

use crate::{domain::users::Credentials, service::users::{UserService, UserCreationError}, validation::ValidatedJson};

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_users<T: UserService + Debug>(Extension(service): Extension<T>, ValidatedJson(credentials): ValidatedJson<Credentials>) -> Result<StatusCode, StatusCode> {
    info!("Received registration attempt");
    match service.register(credentials).await {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(UserCreationError::DuplicateEmail) => Err(StatusCode::CONFLICT),
        Err(UserCreationError::Unknown) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
