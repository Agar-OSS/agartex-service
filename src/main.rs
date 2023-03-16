use tracing::error;

#[tokio::main]
async fn main() {
    // build application from command line arguments
    if let Err(err) = agartex_service::setup() {
        error!(%err);
        return;
    }
    // run server
    if let Err(err) = agartex_service::run().await {
        error!(%err);
    }
}
