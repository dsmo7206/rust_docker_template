use tonic::{transport::Server, Request, Response, Status};
use rust_backend::echo_service_server::{EchoService, EchoServiceServer};
use rust_backend::{EchoRequest, EchoResponse};

pub mod rust_backend {
    tonic::include_proto!("rust_backend");
}

#[derive(Debug, Default)]
pub struct MyEchoService {
    i: u32
}

#[tonic::async_trait]
impl EchoService for MyEchoService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        let reply = rust_backend::EchoResponse {
            message: format!("Hello {}", message)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let address = "[::1]:50051".parse()?;
    let address = "0.0.0.0:50051".parse()?;
    let echo_service = MyEchoService::default();

    println!("Server running...");

    Server::builder()
        .add_service(EchoServiceServer::new(echo_service))
        .serve(address)
        .await?;

    Ok(())
}
