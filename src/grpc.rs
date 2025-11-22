use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;
use crate::AppState;

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_server::{LinkWithMentor, LinkWithMentorServer};
use pb::{PingRequest, PingResponse};

#[derive(Debug)]
pub struct MyLinkWithMentor {
    state: Arc<AppState>,
}

#[tonic::async_trait]
impl LinkWithMentor for MyLinkWithMentor {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        println!("Got a request: {:?}", request);

        // Example usage of state:
        // self.state.firebase.send_notification(...)

        let reply = PingResponse {
            message: format!("Pong: {}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}

pub async fn run(host: &str, port: u16, state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port).parse()?;
    let service = MyLinkWithMentor { state };

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(LinkWithMentorServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
