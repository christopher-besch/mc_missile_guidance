use std::time::Duration;

use guidance::guidance_server::{Guidance, GuidanceServer};
use guidance::{ControlInput, Missile, MissileHardwareConfig, MissileState};
use tonic::{transport::Server, Request, Response, Status};

pub mod guidance {
    tonic::include_proto!("mcmissile.guidance");
}

#[derive(Debug, Default)]
pub struct MyGuidance {}

#[tonic::async_trait]
impl Guidance for MyGuidance {
    async fn register_missile(
        &self,
        missile: Request<Missile>,
    ) -> Result<Response<MissileHardwareConfig>, Status> {
        println!("registered a missile: {:?}", missile);
        tokio::time::sleep(Duration::from_millis(5)).await;

        let missile_hardware_config = MissileHardwareConfig {
            player_name_regex: "".to_string(),
            target_entity_regex: "".to_string(),
        };
        Ok(Response::new(missile_hardware_config))
    }

    async fn get_guidance(
        &self,
        missile_state: Request<MissileState>,
    ) -> Result<Response<ControlInput>, Status> {
        println!("get guidance for: {:?}", missile_state);
        tokio::time::sleep(Duration::from_millis(5)).await;

        let control_input = ControlInput {
            pitch_turn: 0.0,
            yaw_turn: 0.0,
            explode: false,
            disarm: false,
        };
        Ok(Response::new(control_input))
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
