use constants::SERVER_CONFIG;
use hyper::Error;
use routing::get_main_router;
use tracing::{error, subscriber::SetGlobalDefaultError};
use tracing_subscriber::FmtSubscriber;

// declare child modules
mod constants;
mod routing;

#[tracing::instrument]
pub fn setup() -> Result<(), SetGlobalDefaultError> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
}

#[tracing::instrument]
pub async fn run() -> Result<(), Error> {
    // run it with hyper
    if let Ok(config) = SERVER_CONFIG.verify() {
        axum::Server::try_bind(&config.addr)?
            .serve(get_main_router().into_make_service())
            .await
    } else {
        error!("Bad server configuration!");
        Ok(())
    }
}
