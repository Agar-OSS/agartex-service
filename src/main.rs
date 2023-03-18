use tracing::error;

#[tokio::main]
async fn main() {
    // build application from command line arguments
    // if let Err(err) = agartex_service::setup() {
    //     error!(%err);
    //     return;
    // }
    agartex_service::setup();
    // run server
    if let Err(err) = agartex_service::run().await {
        error!(%err);
    }
}
