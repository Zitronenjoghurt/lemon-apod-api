mod services;

use crate::services::apod_service::ApodService;
use lemon_apod::apod_server::ApodServer;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod lemon_apod {
    tonic::include_proto!("lemon_apod");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    let addr = "[::]:50051".parse::<SocketAddr>()?;
    let greeter = ApodService::default();

    info!("Server starting...");
    let server = Server::builder()
        .add_service(ApodServer::new(greeter));

    server.serve_with_shutdown(addr, async {
        info!("Server started");
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        info!("Server shutting down...");
    }).await?;

    info!("Server shutdown complete");
    Ok(())
}
