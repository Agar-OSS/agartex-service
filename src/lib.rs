use std::net::SocketAddr;
use clap::Parser;
use hyper::Error;
use routing::{get_main_router};
use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::FmtSubscriber;

// declare child modules
mod routing;

#[derive(Debug, Parser)]
pub struct LaunchConfig {
    pub addr: SocketAddr
}

pub fn setup() -> Result<LaunchConfig, SetGlobalDefaultError> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(LaunchConfig::parse())
}

#[tracing::instrument]
pub async fn run(config: LaunchConfig) -> Result<(), Error> {
    // run it with hyper
    axum::Server::try_bind(&config.addr)?
        .serve(get_main_router().into_make_service())
        .await
}