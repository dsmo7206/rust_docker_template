use rust_backend::echo_service_client::EchoServiceClient;
use rust_backend::EchoRequest;

pub mod rust_backend {
    tonic::include_proto!("rust_backend");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client connecting...");
    let mut client = EchoServiceClient::connect("http://server:50051").await?;

    loop {
        println!("Client sending request...");

        let request = tonic::Request::new(
            EchoRequest {
                message: "Test!".into()
            }
        );

        let response = client.echo(request).await?;

        println!("Client got response: {:?}", response);
        std::thread::sleep_ms(1000);
    }

    Ok(())
}