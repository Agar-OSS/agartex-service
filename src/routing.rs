use axum::{routing::get, Router};
use axum_database_sessions::{SessionStore, SessionPgPool, SessionLayer};
use axum_sessions_auth::{AuthConfig, AuthSessionLayer};
use sqlx::PgPool;

use crate::auth::AuthUser;

use self::{users::users_router, sessions::sessions_router};

mod users;
mod sessions;

pub fn get_main_router(pool: &PgPool, auth_config: AuthConfig<i32>, session_store: SessionStore<SessionPgPool>) -> Router {
    let auth_layer = AuthSessionLayer::<AuthUser, i32, SessionPgPool, PgPool>::new(Some(pool.clone())).with_config(auth_config);
    let session_layer = SessionLayer::new(session_store);
    Router::new()
        .nest("/users", users_router(pool))
        .nest("/sessions", sessions_router(pool, auth_layer.clone(), session_layer.clone()))
        // .layer(session_layer)
        // .layer(auth_layer)
        .route("/", get(|| async { "Hello, World!" }))
}
