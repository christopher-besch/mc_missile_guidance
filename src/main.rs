mod guidance;
pub mod lookup_tables;

#[allow(clippy::all)]
pub mod guidance_grpc {
    tonic::include_proto!("mcmissile.guidance");
}

use anyhow::Context;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;

use guidance_grpc::guidance_server::{Guidance, GuidanceServer};
use guidance_grpc::{ControlInput, HealthRequest, HealthResponse, MissileState};
use tokio::signal::unix::{signal, SignalKind};
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
    async fn health_check(
        &self,
        _request: tonic::Request<HealthRequest>,
    ) -> std::result::Result<tonic::Response<HealthResponse>, tonic::Status> {
        Ok(Response::new(HealthResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Chris' mc_missile guidance server boot up");
    let port = env::var("PORT")
        .expect("the environment variable PORT must be provided")
        .parse::<u16>()
        .context("failed to convert PORT to u16")
        .unwrap();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let guidance = MyGuidance::default();

    // TODO: this still doesn't work with docker
    async fn await_shutdown() {
        // https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
        let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
        let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = signal_terminate.recv() => (),
            _ = signal_interrupt.recv() => (),
        };
    }

    Server::builder()
        .add_service(GuidanceServer::new(guidance))
        .serve_with_shutdown(addr, await_shutdown())
        .await?;

    Ok(())
}
