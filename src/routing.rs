use axum::{routing::get, Router};

pub fn get_main_router() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
