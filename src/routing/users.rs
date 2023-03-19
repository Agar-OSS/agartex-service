use axum::{Router, routing, Extension, http::HeaderValue};
use hyper::Method;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{control, service::users::PgUserService, constants};

pub fn users_router(pool: &PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(constants::CLIENT_URL.parse::<HeaderValue>().unwrap());

    let handler = routing::post(control::users::post_users::<PgUserService>)
        .layer(Extension(PgUserService::new(pool)));
    
    Router::new()
        .route("/", handler)
        .layer(cors)
}
