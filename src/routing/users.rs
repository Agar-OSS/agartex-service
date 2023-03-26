use axum::{Router, routing, Extension};
use sqlx::PgPool;

use crate::{control, service::users::PgUserService, repository::users::PgUserRepository};

pub fn users_router(pool: &PgPool) -> Router {
    let user_service = PgUserService::new(PgUserRepository::new(pool));

    let handler = routing::post(control::users::post_users::<PgUserService>)
        .layer(Extension(user_service));
    
    Router::new()
        .route("/", handler)
}
