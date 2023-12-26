use example::oxion_client::OxionClient;
use example::HelloRequest;

pub mod example {
    tonic::include_proto!("example");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OxionClient::connect(
        "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        HelloRequest {
            name: "Hfidhifhjsif".to_owned(),
        }
    );

    let response = client.send_message(request).await?; 

    println!("RESPONSE={:?}", response);

    Ok(())
}