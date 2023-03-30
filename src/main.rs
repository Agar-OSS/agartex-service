use tracing::error;

#[tokio::main]
async fn main() {
    agartex_service::setup();
    if let Err(err) = agartex_service::run().await {
        error!(%err);
    }
}
