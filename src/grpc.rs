use tonic::{transport::Server, Request, Response, Status};

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_server::{LinkWithMentor, LinkWithMentorServer};
use pb::{PingRequest, PingResponse};

#[derive(Debug, Default)]
pub struct MyLinkWithMentor {}

#[tonic::async_trait]
impl LinkWithMentor for MyLinkWithMentor {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = PingResponse {
            message: format!("Pong: {}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}

pub async fn run(host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port).parse()?;
    let service = MyLinkWithMentor::default();

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(LinkWithMentorServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
