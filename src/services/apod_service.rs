use tonic::{Request, Response, Status};
use crate::lemon_apod::apod_server::Apod;
use crate::lemon_apod::{ApodResponse, Empty};

#[derive(Debug, Default)]
pub struct ApodService {}

#[tonic::async_trait]
impl Apod for ApodService {
    async fn get_today(&self, _request: Request<Empty>) -> Result<Response<ApodResponse>, Status> {
        let reply = ApodResponse {
            title: "Test".to_string(),
            apod_url: "apod url".to_string(),
            image_url: "image url".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn get_random(&self, _request: Request<Empty>) -> Result<Response<ApodResponse>, Status> {
        todo!()
    }
}