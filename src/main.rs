use agartex_service;
use tracing::error;

#[tokio::main]
async fn main() {
    // build application from command line arguments
    let config = match agartex_service::setup() {
        Ok(cfg) => cfg,
        Err(err) => {
            error!(%err);
            return;
        } 
    };

    // run server
    if let Err(err) = agartex_service::run(config).await {
        error!(%err);
    }
}
