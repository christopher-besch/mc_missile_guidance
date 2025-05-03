use std::collections::HashMap;

use crate::guidance_grpc::missile_hardware_config::{
    Airframe, Battery, InertialSystem, Motor, Seeker, Warhead,
};
use crate::guidance_grpc::{ControlInput, MissileHardwareConfig, MissileState};
use anyhow::{Context, Result};

use nalgebra as na;
type Vec3 = na::Vector3<f64>;

use crate::guidance::helper::{
    calc_pitch_yaw_turn_for_stationary_target, get_missile_pos, get_seeker_target_pos,
    sphere_should_detonate,
};
use crate::guidance::MissileGuidance;

static THRUST: f64 = 0.4;
static TOP_ATTACK_HEIGHT: f64 = 50.0;
static TOP_ATTACK_TOWARDS_TARGET: f64 = 20.0;
static PROXIMITY_FUSE_DIST: f64 = 5.0;

#[derive(Debug, PartialEq)]
enum State {
    Launch,
    LockTarget,
    RisingUp,
    FlyingTowardsTarget,
}

#[derive(Debug)]
pub struct StationaryEntityGuidance {
    // used when the target is not visible
    // set to Some when a lock has been acquired
    fallback_target_coord: Option<Vec3>,
    top_attack_coord: Option<Vec3>,
    top_attack: bool,
    state: State,
    hardware_config: Option<MissileHardwareConfig>,
}

impl MissileGuidance for StationaryEntityGuidance {
    async fn new(
        params: HashMap<String, String>,
        init_missile_state: &MissileState,
    ) -> Result<Self> {
        assert!(init_missile_state.time == 0);

        let seeker_entity_name = params.get("ent").cloned().unwrap_or_default();

        let top_attack = params
            .get("top")
            .unwrap_or(&String::from("false"))
            .parse()?;
        Ok(Self {
            fallback_target_coord: None,
            top_attack_coord: None,
            top_attack,
            state: State::Launch,
            hardware_config: Some(MissileHardwareConfig {
                warhead: Warhead::TntM as i32,
                airframe: Airframe::DefaultAirframe as i32,
                motor: Motor::SingleStageM as i32,
                battery: Battery::LiIonM as i32,
                seeker: Seeker::IrSeekerM as i32,
                seeker_entity_name,
                inertial_system: InertialSystem::DefaultImu as i32,
            }),
        })
    }

    async fn get_guidance(&mut self, missile_state: MissileState) -> ControlInput {
        println!("{:?}", missile_state);

        let control_input_res = match self.state {
            // box pin to allow recursion
            State::Launch => Box::pin(self.launch(missile_state)).await,
            State::LockTarget => Box::pin(self.lock_target(missile_state)).await,
            State::RisingUp => Box::pin(self.rising_up(missile_state)).await,
            State::FlyingTowardsTarget => Box::pin(self.flying_towards_target(missile_state)).await,
        };

        match control_input_res {
            Ok(i) => i,
            Err(e) => {
                eprintln!("failed in state {:?}: {}", self.state, e);
                ControlInput {
                    // set later
                    id: 0,
                    hardware_config: self.hardware_config.take(),
                    pitch_turn: 0.0,
                    yaw_turn: 0.0,
                    explode: false,
                    disarm: true,
                }
            }
        }
    }
}

impl StationaryEntityGuidance {
    async fn launch(&mut self, missile_state: MissileState) -> Result<ControlInput> {
        assert!(self.state == State::Launch);
        assert!(self.hardware_config.is_some());
        assert!(missile_state.time == 0);

        // do the actual stuff once the seeker head input is here
        self.state = State::LockTarget;

        Ok(ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn: 0.0,
            yaw_turn: 0.0,
            explode: false,
            disarm: false,
        })
    }

    async fn lock_target(&mut self, missile_state: MissileState) -> Result<ControlInput> {
        assert!(self.state == State::LockTarget);
        assert!(missile_state.time == 1);

        let pos = get_missile_pos(&missile_state).await;

        let target_coords = get_seeker_target_pos(&missile_state)
            .await?
            .context("entity must be visible on first tick")?;

        let horizontal_direction =
            Vec3::new(target_coords.x - pos.x, 0.0, target_coords.z - pos.z).normalize();
        let top_attack_coords = TOP_ATTACK_TOWARDS_TARGET * horizontal_direction
            + Vec3::new(pos.x, target_coords.y + TOP_ATTACK_HEIGHT, pos.z);

        self.fallback_target_coord = Some(target_coords);
        self.top_attack_coord = Some(top_attack_coords);

        self.state = if self.top_attack {
            State::RisingUp
        } else {
            State::FlyingTowardsTarget
        };
        Ok(self.get_guidance(missile_state).await)
    }

    async fn rising_up(&mut self, missile_state: MissileState) -> Result<ControlInput> {
        assert!(self.state == State::RisingUp);
        if missile_state.pos_y >= self.top_attack_coord.context("no lock")?.y {
            self.state = State::FlyingTowardsTarget;
            return Ok(self.get_guidance(missile_state).await);
        }

        let (pitch_turn, yaw_turn) = calc_pitch_yaw_turn_for_stationary_target(
            &missile_state,
            self.top_attack_coord.context("no lock")?,
            THRUST,
        )
        .await;
        Ok(ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn,
            yaw_turn,
            explode: self.should_explode(&missile_state).await?,
            disarm: false,
        })
    }

    async fn flying_towards_target(&mut self, missile_state: MissileState) -> Result<ControlInput> {
        assert!(self.state == State::FlyingTowardsTarget);
        // There is no next state; we simply detonate at some point.
        let (pitch_turn, yaw_turn) = calc_pitch_yaw_turn_for_stationary_target(
            &missile_state,
            self.get_target_coords(&missile_state).await?,
            THRUST,
        )
        .await;
        Ok(ControlInput {
            // set later
            id: 0,
            hardware_config: self.hardware_config.take(),
            pitch_turn,
            yaw_turn,
            explode: self.should_explode(&missile_state).await?,
            disarm: false,
        })
    }

    async fn get_target_coords(&self, missile_state: &MissileState) -> Result<Vec3> {
        Ok(get_seeker_target_pos(missile_state)
            .await
            .unwrap_or(None)
            .unwrap_or(self.fallback_target_coord.context("no lock")?))
    }

    async fn should_explode(&self, missile_state: &MissileState) -> Result<bool> {
        Ok(sphere_should_detonate(
            missile_state,
            self.get_target_coords(missile_state).await?,
            PROXIMITY_FUSE_DIST,
        )
        .await)
    }
}
