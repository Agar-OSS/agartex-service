use axum::{Router, routing, Extension};
use sqlx::PgPool;

use crate::{control, service::users::PgUserService};

pub fn users_router(pool: &PgPool) -> Router {
    let handler = routing::post(control::users::post_users::<PgUserService>)
        .layer(Extension(PgUserService::new(pool)));
    Router::new().route("/", handler)
}
