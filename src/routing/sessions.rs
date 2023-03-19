use axum::{routing, Router, Extension, http::HeaderValue};
use axum_database_sessions::{SessionPgPool, SessionLayer};
use axum_sessions_auth::AuthSessionLayer;
use hyper::Method;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{service::sessions::PgSessionService, control, auth::AuthUser, constants};

pub fn sessions_router(pool: &PgPool, auth_layer: AuthSessionLayer<AuthUser, i32, SessionPgPool, PgPool>, session_layer: SessionLayer<SessionPgPool>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(constants::CLIENT_URL.parse::<HeaderValue>().unwrap());

    let handler = routing::post(control::sessions::post_sessions::<PgSessionService>)
        .layer(Extension(PgSessionService::new(pool)))
        .layer(auth_layer)
        .layer(session_layer);
    
    Router::new()
        .route("/", handler)
        .layer(cors)
}
