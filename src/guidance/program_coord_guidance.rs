use std::collections::HashMap;

use crate::guidance_grpc::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, Warhead,
};
use crate::guidance_grpc::{ControlInput, MissileHardwareConfig, MissileState};
use anyhow::{Context, Result};

use nalgebra as na;
type Vec3 = na::Vector3<f64>;

use crate::guidance::helper::{
    calc_pitch_yaw_turn_for_stationary_target, get_missile_pos, sphere_should_detonate,
};
use crate::guidance::MissileGuidance;

static THRUST: f64 = 0.4;
static TOP_ATTACK_HEIGHT: f64 = 50.0;
static TOP_ATTACK_TOWARDS_TARGET: f64 = 20.0;
static PROXIMITY_FUSE_DIST: f64 = 5.0;

#[derive(Debug, PartialEq)]
enum State {
    Launch,
    RisingUp,
    FlyingTowardsTarget,
}

#[derive(Debug)]
pub struct TargetCoordGuidance {
    target_coord: Vec3,
    top_attack_coord: Vec3,
    top_attack: bool,
    state: State,
    hardware_config: Option<MissileHardwareConfig>,
}

impl MissileGuidance for TargetCoordGuidance {
    async fn new(
        params: HashMap<String, String>,
        init_missile_state: &MissileState,
    ) -> Result<Self> {
        assert!(init_missile_state.time == 0);

        let pos = get_missile_pos(init_missile_state).await;
        let target_coords: Vec3 = Vec3::new(
            params.get("x").context("failed to find x")?.parse()?,
            params.get("y").context("failed to find y")?.parse()?,
            params.get("z").context("failed to find z")?.parse()?,
        );
        let horizontal_direction =
            Vec3::new(target_coords.x - pos.x, 0.0, target_coords.z - pos.z).normalize();
        let top_attack_coords = TOP_ATTACK_TOWARDS_TARGET * horizontal_direction
            + Vec3::new(pos.x, target_coords.y + TOP_ATTACK_HEIGHT, pos.z);

        let top_attack = params
            .get("top")
            .unwrap_or(&String::from("false"))
            .parse()?;
        Ok(Self {
            target_coord: target_coords,
            top_attack_coord: top_attack_coords,
            top_attack,
            state: State::Launch,
            hardware_config: Some(MissileHardwareConfig {
                warhead: Warhead::TntM as i32,
                airframe: Airframe::DefaultAirframe as i32,
                motor: Motor::SingleStageM as i32,
                battery: Battery::LiIonM as i32,
                seeker: Seeker::NoSeeker as i32,
                seeker_entity_name: "".to_string(),
                inertial_system: InertialSystem::DefaultImu as i32,
            }),
        })
    }

    async fn get_guidance(&mut self, missile_state: MissileState) -> ControlInput {
        println!("{:?}", missile_state);

        match self.state {
            // box pin to allow recursion
            State::Launch => Box::pin(self.launch(missile_state)).await,
            State::RisingUp => Box::pin(self.rising_up(missile_state)).await,
            State::FlyingTowardsTarget => Box::pin(self.flying_towards_target(missile_state)).await,
        }
    }
}

impl TargetCoordGuidance {
    async fn launch(&mut self, missile_state: MissileState) -> ControlInput {
        assert!(self.state == State::Launch);
        assert!(self.hardware_config.is_some());
        assert!(missile_state.time == 0);

        self.state = if self.top_attack {
            State::RisingUp
        } else {
            State::FlyingTowardsTarget
        };
        self.get_guidance(missile_state).await
    }

    async fn rising_up(&mut self, missile_state: MissileState) -> ControlInput {
        assert!(self.state == State::RisingUp);
        if missile_state.pos_y >= self.top_attack_coord.y {
            self.state = State::FlyingTowardsTarget;
            return self.get_guidance(missile_state).await;
        }

        let (pitch_turn, yaw_turn) = calc_pitch_yaw_turn_for_stationary_target(
            &missile_state,
            self.top_attack_coord,
            THRUST,
        )
        .await;
        ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn,
            yaw_turn,
            explode: sphere_should_detonate(&missile_state, self.target_coord, PROXIMITY_FUSE_DIST)
                .await,
            disarm: false,
        }
    }

    async fn flying_towards_target(&mut self, missile_state: MissileState) -> ControlInput {
        assert!(self.state == State::FlyingTowardsTarget);
        // There is no next state; we simply detonate at some point.
        let (pitch_turn, yaw_turn) =
            calc_pitch_yaw_turn_for_stationary_target(&missile_state, self.target_coord, THRUST)
                .await;
        ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn,
            yaw_turn,
            explode: sphere_should_detonate(&missile_state, self.target_coord, PROXIMITY_FUSE_DIST)
                .await,
            disarm: false,
        }
    }
}
