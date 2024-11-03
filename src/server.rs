mod services;

use tonic::transport::Server;
use lemon_apod::apod_server::{Apod, ApodServer};
use crate::services::apod_service::ApodService;

pub mod lemon_apod {
    tonic::include_proto!("lemon_apod");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = ApodService::default();

    Server::builder()
        .add_service(ApodServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
