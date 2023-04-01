use std::env;

use axum::{routing::get, Router};
use http::HeaderValue;
use sqlx::PgPool;
use tower_http::cors::{CorsLayer, Any};

use crate::{constants, domain::users::User, auth::AuthLayer, service::sessions::BcryptSessionService, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

use self::{users::users_router, sessions::sessions_router};

mod users;
mod sessions;

pub fn get_main_router(pool: &PgPool) -> Router {
    let auth = AuthLayer::new(BcryptSessionService::new(PgSessionRepository::new(pool), PgUserRepository::new(pool)));
    
    let authorized_handler = get(|user: User| async move { format!("Hello, {}", user.email) })
        .layer(auth);
    
    

    let cors = match env::var(constants::CLIENT_URL_ENV_VAR) {
        Ok(client_url) => CorsLayer::new()
            .allow_origin(client_url.parse::<HeaderValue>().unwrap())
            .allow_headers(Any),
        Err(_) => CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
    };

    Router::new()
        .nest("/users", users_router(pool))
        .nest("/sessions", sessions_router(pool))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/authorized", authorized_handler)
        .layer(cors)

    // setup CORS, if possible
    
}
