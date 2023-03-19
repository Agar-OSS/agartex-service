use std::net::SocketAddr;

use database::get_conn_pool;
use hyper::Error;
use routing::get_main_router;
use tracing::{error, info};

// declare child modules
mod constants;
mod control;
mod routing;
mod database;
mod domain;
mod service;
mod auth;

#[tracing::instrument]
pub fn setup() {
    tracing_subscriber::fmt().init();
    info!("Logging setup complete!");
}

#[tracing::instrument]
pub async fn run() -> Result<(), Error> {
    let pool = match get_conn_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}!", err);
            return Ok(());
        }
    };

    let (auth_config, session_store) = auth::auth_setup(&pool).await;

    info!("Running server!");
    axum::Server::try_bind(&SocketAddr::from(constants::SERVER_URL))?
            .serve(get_main_router(&pool, auth_config, session_store).into_make_service())
            .await
}
