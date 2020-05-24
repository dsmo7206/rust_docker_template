use tokio::time::delay_for;
use std::time::Duration;

use echo_package::echo_service_client::EchoServiceClient;
use echo_package::EchoRequest;

pub mod echo_package {
    tonic::include_proto!("echo_package");
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
        delay_for(Duration::from_millis(1000)).await;
    }

    Ok(())
}