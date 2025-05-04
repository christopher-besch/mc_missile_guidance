use crate::{guidance_grpc::MissileState, lookup_tables::lookup_gravity_heading};

use anyhow::{bail, Result};
use approx::{assert_relative_eq, relative_eq};

use nalgebra as na;

type Vec3 = na::Vector3<f64>;

static GRAVITY: f64 = 0.2;

pub async fn get_missile_pos(missile_state: &MissileState) -> Vec3 {
    Vec3::new(
        missile_state.pos_x,
        missile_state.pos_y,
        missile_state.pos_z,
    )
}

pub async fn get_missile_vel(missile_state: &MissileState) -> Vec3 {
    Vec3::new(
        missile_state.vel_x,
        missile_state.vel_y,
        missile_state.vel_z,
    )
}

pub async fn get_seeker_target_pos(missile_state: &MissileState) -> Result<Option<Vec3>> {
    if !missile_state.target_lock {
        bail!("no target lock");
    } else if !missile_state.target_visible {
        Ok(None)
    } else {
        Ok(Some(Vec3::new(
            missile_state.target_pos_x,
            missile_state.target_pos_y,
            missile_state.target_pos_z,
        )))
    }
}

// pub async fn get_seeker_target_vel(missile_state: &MissileState) -> Result<Option<Vec3>> {
//     if !missile_state.target_lock {
//         bail!("no target lock");
//     } else if !missile_state.target_visible {
//         Ok(None)
//     } else {
//         Ok(Some(Vec3::new(
//             missile_state.target_vel_x,
//             missile_state.target_vel_y,
//             missile_state.target_vel_z,
//         )))
//     }
// }

// The length of target_vel doesn't matter.
// The length of the returned acc is not guaranteed to be anything.
// This is not entirely correct as we loose a bit of our thrust due to gravity.
pub async fn calculate_acc_for_target_vel(cur_vel: Vec3, target_vel: Vec3, thrust: f64) -> Vec3 {
    // When the missile doesn't move, just head for the target.
    if relative_eq!(cur_vel.norm_squared(), 0.0) {
        return target_vel;
    }

    let radical = 4.0 * target_vel.dot(&cur_vel).powf(2.0)
        - 4.0 * target_vel.norm_squared() * (cur_vel.norm_squared() - thrust.powf(2.0));

    if radical <= 0.0 {
        let p = cur_vel.dot(&target_vel) / target_vel.norm_squared() * target_vel;
        p - cur_vel
    } else {
        let r1 =
            (2.0 * target_vel.dot(&cur_vel) + radical.sqrt()) / (2.0 * target_vel.norm_squared());
        // fun fact: this would be good opportunity for breaking
        // let r2 = (2.0 * target_vel.dot(&cur_vel) - radical.sqrt())
        //     / (2.0 * target_vel.norm_squared());
        let r = r1;
        let acc = r * target_vel - cur_vel;
        assert_relative_eq!(acc.norm(), thrust, epsilon = 0.0001);
        acc
    }
}

// Calculate the pitch and yaw of a vector as if it were the direction a player is looking in
// degrees.
pub async fn calc_pitch_yaw(vec: Vec3) -> (f64, f64) {
    // projected onto the horizontal plane
    let vec_horizontal = Vec3::new(vec.x, 0.0, vec.z);
    let mut pitch = vec_horizontal
        .normalize()
        .dot(&vec.normalize())
        .acos()
        .to_degrees();
    if vec.y < 0.0 {
        pitch *= -1.0;
    }
    let yaw = vec.x.atan2(vec.z).to_degrees();
    (pitch, yaw)
}

pub async fn calc_pitch_yaw_turn_for_stationary_target(
    missile_state: &MissileState,
    cur_target: Vec3,
    thrust: f64,
) -> (f64, f64) {
    let to_cur_target = cur_target - get_missile_pos(missile_state).await;
    let acc_to_cur_target =
        calculate_acc_for_target_vel(get_missile_vel(missile_state).await, to_cur_target, thrust)
            .await;
    let (target_pitch, target_yaw) = calc_pitch_yaw(acc_to_cur_target).await;
    let pitch_turn = lookup_gravity_heading(GRAVITY, target_pitch, thrust) - missile_state.pitch;
    let yaw_turn = target_yaw - missile_state.yaw;

    (pitch_turn, yaw_turn)
}

pub async fn sphere_should_detonate(
    missile_state: &MissileState,
    target_coord: Vec3,
    proximity_fuse_dist: f64,
) -> bool {
    // check if we're close enough to the target
    (target_coord - get_missile_pos(missile_state).await).norm_squared()
        <= proximity_fuse_dist.powf(2.0)
}
