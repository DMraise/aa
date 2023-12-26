use tonic::{transport::Server, Request, Response, Status};

use example::oxion_server::{Oxion, OxionServer};
use example::{HelloRequest, HelloResponse};

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Debug, Default)]
pub struct OxionService {}

#[tonic::async_trait]
impl Oxion for OxionService {
    async fn send_message(
        &self, 
        request: Request<HelloRequest>,
    ) -> Result<Responce<HelloResponse>, Status>{
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = HelloResponse {
            message: format!("Sent {}", req.name).into(),
        };

        Ok(Responce::new(reply));
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = OxionService::default();


     Server::builder()
        .add_service(OxionServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}