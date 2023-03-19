use std::fmt::Debug;

use axum::{Extension, Json};
use axum_database_sessions::SessionPgPool;
use axum_sessions_auth::AuthSession;
use hyper::StatusCode;
use sqlx::PgPool;
use tracing::info;

use crate::{service::sessions::{SessionService, SessionCreationError}, auth::AuthUser, domain::users::Credentials};

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>, 
    auth: AuthSession<AuthUser, i32, SessionPgPool, PgPool>, 
    Json(credentials): Json<Credentials>
) -> Result<StatusCode, StatusCode> {
    info!("Attempting login for user {}", credentials.email);
    let id = match service.login(credentials).await {
        Err(SessionCreationError::NoUser) =>  {
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(SessionCreationError::Unknown) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
        Ok(id) => id
    };

    if let Some(AuthUser::Known(current_user)) = &auth.current_user {
        if current_user.id == id {
            auth.session.renew();
        }
    }

    auth.session.set_store(true);
    auth.login_user(id);
    auth.remember_user(true);
    return Ok(StatusCode::CREATED);
}
