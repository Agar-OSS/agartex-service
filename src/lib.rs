use database::create_conn_pool;
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
mod validation;

#[tracing::instrument]
pub fn setup() {
    tracing_subscriber::fmt().init();
    info!("Logging setup complete!");
    info!("Loaded environment variables")
}



#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    let pool = match create_conn_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            error!("Could not connect to database:\n{:?}", err);
            return Err(anyhow::Error::from(err))
        }
    };

    info!("Running server!");
    match axum::Server::try_bind(&constants::SERVER_URL)?
        .serve(get_main_router(&pool).into_make_service())
        .await {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::Error::from(err))
    }
}
