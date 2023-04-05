use axum::{Router, routing, Extension};
use sqlx::PgPool;

use crate::{control, service::{users::HashUserService, hash::BcryptHashService}, repository::users::PgUserRepository};

pub fn users_router(pool: &PgPool) -> Router {
    let user_service = HashUserService::new(PgUserRepository::new(pool), BcryptHashService::new());

    let handler = routing::post(control::users::post_users::<HashUserService<PgUserRepository, BcryptHashService>>)
        .layer(Extension(user_service));
    
    Router::new()
        .route("/", handler)
}
