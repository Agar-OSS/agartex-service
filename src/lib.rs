use std::net::SocketAddr;

use database::create_conn_pool;
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
mod repository;
mod auth;

#[tracing::instrument]
pub fn setup() {
    tracing_subscriber::fmt().init();
    info!("Logging setup complete!");
}

#[tracing::instrument]
pub async fn run() -> Result<(), Error> {
    let pool = match create_conn_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}", err);
            return Ok(());
        }
    };

    info!("Running server!");
    axum::Server::try_bind(&SocketAddr::from(constants::SERVER_URL))?
            .serve(get_main_router(&pool).into_make_service())
            .await
}
