use std::collections::HashMap;

use crate::guidance_grpc::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, Warhead,
};
use crate::guidance_grpc::{ControlInput, MissileHardwareConfig, MissileState};
use crate::lookup_tables::lookup_gravity_heading;
use anyhow::{Context, Result};

use na::Vector3;
use nalgebra as na;

use super::MissileGuidance;

static GRAVITY: f64 = 0.2;
static THRUST: f64 = 0.4;
static TOP_ATTACK_HEIGHT: f64 = 50.0;
static TOP_ATTACK_TOWARDS_TARGET: f64 = 20.0;
static PROXIMITY_FUSE_DIST: f64 = 5.0;

#[derive(Debug)]
enum State {
    Launch,
    RisingUp,
    FlyingTowardsTarget,
}

#[derive(Debug)]
pub struct TargetCoordGuidance {
    target_coord: Vector3<f64>,
    top_attack_coord: Vector3<f64>,
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

        let pos = Self::get_missile_pos(init_missile_state);
        let target_coords: Vector3<f64> = Vector3::new(
            params.get("x").context("failed to find x")?.parse()?,
            params.get("y").context("failed to find y")?.parse()?,
            params.get("z").context("failed to find z")?.parse()?,
        );
        let horizontal_direction =
            Vector3::new(target_coords.x - pos.x, 0.0, target_coords.z - pos.z).normalize();
        let top_attack_coords = TOP_ATTACK_TOWARDS_TARGET * horizontal_direction
            + Vector3::new(pos.x, target_coords.y + TOP_ATTACK_HEIGHT, pos.z);

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
        if missile_state.pos_y >= self.top_attack_coord.y {
            self.state = State::FlyingTowardsTarget;
            return self.get_guidance(missile_state).await;
        }
        self.flying_towards_coord(missile_state, self.top_attack_coord)
            .await
    }

    async fn flying_towards_target(&mut self, missile_state: MissileState) -> ControlInput {
        // There is not next state; we simply detonate at some point.
        self.flying_towards_coord(missile_state, self.target_coord)
            .await
    }

    async fn flying_towards_coord(
        &mut self,
        missile_state: MissileState,
        cur_target: na::Vector3<f64>,
    ) -> ControlInput {
        let to_cur_target = cur_target - Self::get_missile_pos(&missile_state);
        // TODO: account for velocity
        let (target_pitch, target_yaw) = Self::calc_pitch_yaw(to_cur_target);
        let pitch_turn =
            lookup_gravity_heading(GRAVITY, target_pitch, THRUST) - missile_state.pitch;
        let yaw_turn = target_yaw - missile_state.yaw;

        ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn,
            yaw_turn,
            explode: self.should_detonate(&missile_state).await,
            disarm: false,
        }
    }

    fn get_missile_pos(missile_state: &MissileState) -> na::Vector3<f64> {
        Vector3::new(
            missile_state.pos_x,
            missile_state.pos_y,
            missile_state.pos_z,
        )
    }

    // Calculate the pitch and yaw of a vector as if it were the direction a player is looking in
    // degrees.
    fn calc_pitch_yaw(vec: na::Vector3<f64>) -> (f64, f64) {
        println!("{:?}", vec);
        // projected onto the horizontal plane
        let vec_horizontal = na::Vector3::new(vec.x, 0.0, vec.z);
        println!("{:?}", vec_horizontal);
        let mut pitch = vec_horizontal
            .normalize()
            .dot(&vec.normalize())
            .acos()
            .to_degrees();
        if vec.y < 0.0 {
            pitch *= -1.0;
        }
        println!("{:?}", pitch);
        let yaw = vec.x.atan2(vec.z).to_degrees();
        println!("{:?}", yaw);
        (pitch, yaw)
    }

    async fn should_detonate(&mut self, missile_state: &MissileState) -> bool {
        // check if we're close enough to the target
        (self.target_coord - Self::get_missile_pos(missile_state)).norm_squared()
            <= PROXIMITY_FUSE_DIST * PROXIMITY_FUSE_DIST
    }
}
