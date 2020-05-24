use tonic::{transport::Server, Request, Response, Status};
use echo_package::echo_service_server::{EchoService, EchoServiceServer};
use echo_package::{EchoRequest, EchoResponse};

const USE_IPV6: bool = false;
const PORT: u16 = 50051;

pub mod echo_package {
    tonic::include_proto!("echo_package");
}

#[derive(Debug, Default)]
pub struct MyEchoService {}

#[tonic::async_trait]
impl EchoService for MyEchoService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        let reply = echo_package::EchoResponse {
            message: format!("Hello {}", message)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = {
        if USE_IPV6 { format!("[::1]:{}", PORT) }
        else { format!("0.0.0.0:{}", PORT) }
    }.parse()?;

    let echo_service = MyEchoService::default();

    println!("Server running...");

    Server::builder()
        .add_service(EchoServiceServer::new(echo_service))
        .serve(address)
        .await?;

    Ok(())
}
