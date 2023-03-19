use axum::{routing, Router, Extension};
use axum_database_sessions::{SessionPgPool, SessionLayer};
use axum_sessions_auth::AuthSessionLayer;
use sqlx::PgPool;

use crate::{service::sessions::PgSessionService, control, auth::AuthUser};

pub fn sessions_router(pool: &PgPool, auth_layer: AuthSessionLayer<AuthUser, i32, SessionPgPool, PgPool>, session_layer: SessionLayer<SessionPgPool>) -> Router {
    let handler = routing::post(control::sessions::post_sessions::<PgSessionService>)
        .layer(Extension(PgSessionService::new(pool)))
        .layer(auth_layer)
        .layer(session_layer);
    Router::new().route("/", handler)
}
