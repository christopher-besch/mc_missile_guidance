use std::pin::Pin;
use std::time::Duration;

use guidance::guidance_server::{Guidance, GuidanceServer};
use guidance::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, WarHead,
};
use guidance::{ControlInput, Missile, MissileHardwareConfig, MissileState};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod guidance {
    tonic::include_proto!("mcmissile.guidance");
}

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
        println!("get_guidance");

        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            let mut id = 0;
            while let Some(missile_state) = stream.next().await {
                let missile_state = missile_state?;
                println!("{:?}", missile_state);
                let missile_hardware_config = MissileHardwareConfig {warhead:WarHead::TntM as i32,player_name_regex:"".to_string(),target_entity_regex:"".to_string(), airframe: Airframe::DefaultAirframe as i32, motor: Motor::SingleStageM as i32, battery: Battery::LiIonM as i32, seeker: Seeker::NoSeeker as i32, inertial_system: InertialSystem::DefaultImu as i32};
                println!("sending {:?}: {:?}", id, missile_state);
                id += 1;
                // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                yield ControlInput {
                    // TODO: id handling
                    id: id,
                    hardware_config :Some(missile_hardware_config    ),
                    pitch_turn: 0.0,
                    yaw_turn: 0.0,
                    explode: false,
                    disarm: false,
                };
            }
            println!("finished connection");
        };
        Ok(Response::new(Box::pin(output) as Self::GetGuidanceStream))
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
