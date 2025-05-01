mod guidance;
pub mod lookup_tables;

#[allow(clippy::all)]
pub mod guidance_grpc {
    tonic::include_proto!("mcmissile.guidance");
}

use std::pin::Pin;

use guidance_grpc::guidance_server::{Guidance, GuidanceServer};
use guidance_grpc::{ControlInput, MissileState};
use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGuidance {}

#[tonic::async_trait]
impl Guidance for MyGuidance {
    type GetGuidanceStream =
        Pin<Box<dyn Stream<Item = Result<ControlInput, Status>> + Send + 'static>>;
    async fn get_guidance(
        &self,
        request: Request<tonic::Streaming<MissileState>>,
    ) -> Result<Response<Self::GetGuidanceStream>, Status> {
        let missile_state_stream = request.into_inner();
        let guidance_stream = guidance::get_guidance_stream(missile_state_stream);
        Ok(Response::new(
            Box::pin(guidance_stream) as Self::GetGuidanceStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let addr = "0.0.0.0:42069".parse()?;
    let guidance = MyGuidance::default();

    Server::builder()
        .add_service(GuidanceServer::new(guidance))
        .serve(addr)
        .await?;

    Ok(())
}
