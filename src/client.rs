use example::oxion_client::OxionClient;
use example::HelloRequest;
use std::time::Instant;

pub mod example {
    tonic::include_proto!("example");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time Instant::now();
    let mut client = OxionClient::connect(
        "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        HelloRequest {
            name: "Hfidhifhjsif".to_owned(),
        }
    );

    let response = client.send_message(request).await?; 
    let end_time = Instant::now();
    println!("RESPONSE={:?}", response);
    let elapsed = end_time - start_time;
    println!("Время выполнения: {:?}", elapsed);
    Ok(())
}