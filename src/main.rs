use std::pin::Pin;
use std::time::Duration;

use guidance::guidance_server::{Guidance, GuidanceServer};
use guidance::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, Warhead,
};
use guidance::{ControlInput, Missile, MissileHardwareConfig, MissileState};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod guidance {
    tonic::include_proto!("mcmissile.guidance");
}

#[derive(Debug, Default)]
pub struct MyGuidance {}

pub trait MissileGuidance {
    // The id of the ControlInput will be set by the caller.
    fn get_guidance(
        &mut self,
        missile_state: MissileState,
    ) -> impl std::future::Future<Output = ControlInput>;
}

#[derive(Debug, Default)]
pub struct StraightMissileGuidanceWOGravity {
    target_pitch: f64,
    target_yaw: f64,
    hardware_config: Option<MissileHardwareConfig>,
}

impl StraightMissileGuidanceWOGravity {
    async fn launch_missile(&mut self, missile_state: &MissileState) {
        assert!(missile_state.time == 0);
        self.hardware_config = Some(MissileHardwareConfig {
            warhead: Warhead::TntM as i32,
            player_name_regex: "".to_string(),
            target_entity_regex: "".to_string(),
            airframe: Airframe::DefaultAirframe as i32,
            motor: Motor::SingleStageM as i32,
            battery: Battery::LiIonM as i32,
            seeker: Seeker::NoSeeker as i32,
            inertial_system: InertialSystem::DefaultImu as i32,
        });
        self.target_pitch = missile_state.pitch;
        self.target_yaw = missile_state.yaw;

        println!(
            "target_pitch: {:?} target_yaw: {:?}",
            self.target_pitch, self.target_yaw
        );
    }
}

#[derive(Debug, Default)]
pub struct StraightMissileGuidance {
    target_pitch: f64,
    target_yaw: f64,
    hardware_config: Option<MissileHardwareConfig>,
}

impl StraightMissileGuidance {
    async fn launch_missile(&mut self, missile_state: &MissileState) {
        assert!(missile_state.time == 0);
        self.hardware_config = Some(MissileHardwareConfig {
            warhead: Warhead::TntM as i32,
            player_name_regex: "".to_string(),
            target_entity_regex: "".to_string(),
            airframe: Airframe::DefaultAirframe as i32,
            motor: Motor::SingleStageM as i32,
            battery: Battery::LiIonM as i32,
            seeker: Seeker::NoSeeker as i32,
            inertial_system: InertialSystem::DefaultImu as i32,
        });
        // TODO:
        self.target_pitch = missile_state.pitch;
        self.target_yaw = missile_state.yaw;

        println!(
            "target_pitch: {:?} target_yaw: {:?}",
            self.target_pitch, self.target_yaw
        );
    }
}

impl MissileGuidance for StraightMissileGuidance {
    async fn get_guidance(&mut self, missile_state: MissileState) -> ControlInput {
        // println!("{:?}", missile_state);
        if missile_state.time == 0 {
            self.launch_missile(&missile_state).await;
        }

        let mut control_input = ControlInput {
            // The id of the ControlInput will be set by the caller.
            id: 0,
            // the hardware_config will be set later if we're in the 0th tick.
            hardware_config: None,
            pitch_turn: self.target_pitch - missile_state.pitch,
            yaw_turn: self.target_yaw - missile_state.yaw,
            explode: false,
            // disarm: missile_state.time > 10,
            disarm: false,
        };
        if missile_state.time == 0 {
            assert!(self.hardware_config.is_some());
            control_input.hardware_config = std::mem::replace(&mut self.hardware_config, None);
        }
        return control_input;
    }
}

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
            let mut id: i32 = 0;
            let mut guidance = StraightMissileGuidance{
                target_pitch: 0.0,
                target_yaw: 0.0,
                hardware_config: None,
            };
            while let Some(missile_state) = stream.next().await {
                let missile_state = missile_state?;
                // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                let mut control_input = guidance.get_guidance(missile_state).await;
                id += 1;
                control_input.id = id;
                yield control_input;
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
