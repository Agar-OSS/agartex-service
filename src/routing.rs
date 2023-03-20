use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::{domain::users::User, auth::AuthLayer, service::sessions::BcryptSessionService, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

use self::{users::users_router, sessions::sessions_router};

mod users;
mod sessions;

pub fn get_main_router(pool: &PgPool) -> Router {
    Router::new()
        .nest("/users", users_router(pool))
        .nest("/sessions", sessions_router(pool))
        .route("/", get(|| async { "Hello, World!" }))
}
