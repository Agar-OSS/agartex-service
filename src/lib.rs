use constants::SERVER_CONFIG;
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

#[tracing::instrument]
pub fn setup() {
    tracing_subscriber::fmt().init();
    info!("Loggin setup complete!");
}

#[tracing::instrument]
pub async fn run() -> Result<(), Error> {
    info!("Parsing config!");
    let config = if let Ok(config) = SERVER_CONFIG.verify() {
        info!("Parsed config successfully!");
        config
    } else {
        error!("Bad server configuration!");
        return Ok(());
    };
    let pool = match get_conn_pool(&config).await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}!", err);
            return Ok(());
        }
    };

    info!("Running server!");
    axum::Server::try_bind(&config.addr)?
            .serve(get_main_router(&pool).into_make_service())
            .await
}
