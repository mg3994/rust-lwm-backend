use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_client::LinkWithMentorClient;
use pb::{PingRequest, PingResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkWithMentor Client - Testing Tool\n");

    // Test gRPC connection
    println!("📡 Testing gRPC connection...");
    test_grpc().await?;

    println!("\n✅ All tests completed!");
    Ok(())
}

async fn test_grpc() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LinkWithMentorClient::connect("http://127.0.0.1:3001").await?;

    let request = tonic::Request::new(PingRequest {
        message: "Hello from client!".into(),
    });

    let response = client.ping(request).await?;
    println!("✅ gRPC Ping Response: {}", response.into_inner().message);

    Ok(())
}
