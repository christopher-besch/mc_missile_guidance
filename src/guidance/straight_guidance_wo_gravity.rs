use std::collections::HashMap;

use crate::guidance_grpc::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, Warhead,
};
use crate::guidance_grpc::{ControlInput, MissileHardwareConfig, MissileState};
use anyhow::Result;

use super::MissileGuidance;

#[derive(Debug)]
pub struct StraightGuidanceWOGravity {
    // TODO: remove pub
    pub target_pitch: f64,
    pub target_yaw: f64,
    pub hardware_config: Option<MissileHardwareConfig>,
}

impl MissileGuidance for StraightGuidanceWOGravity {
    async fn new(
        _params: HashMap<String, String>,
        _initial_missile_state: &MissileState,
    ) -> Result<Self> {
        Ok(Self {
            target_pitch: 0.0,
            target_yaw: 0.0,
            hardware_config: None,
        })
    }

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
            control_input.hardware_config = self.hardware_config.take();
        }
        control_input
    }
}

impl StraightGuidanceWOGravity {
    async fn launch_missile(&mut self, missile_state: &MissileState) {
        assert!(missile_state.time == 0);
        self.hardware_config = Some(MissileHardwareConfig {
            warhead: Warhead::TntM as i32,
            airframe: Airframe::DefaultAirframe as i32,
            motor: Motor::SingleStageM as i32,
            battery: Battery::LiIonM as i32,
            seeker: Seeker::IrSeekerM as i32,
            seeker_entity_name: "".to_string(),
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
