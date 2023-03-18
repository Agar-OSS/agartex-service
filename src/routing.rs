use axum::{routing::get, Router};
use sqlx::PgPool;

use self::users::users_router;

mod users;

pub fn get_main_router(pool: &PgPool) -> Router {
    Router::new().nest("/users", users_router(pool))
        .route("/", get(|| async { "Hello, World!" }))
}
