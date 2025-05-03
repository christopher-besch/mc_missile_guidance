use approx::assert_relative_eq;

mod gravity_heading_data;

#[cfg(test)]
#[path = "./tests/lookup_test.rs"]
mod lookup_test;

fn get_lerp_params(list: &[f64], x: f64) -> (usize, usize, f64) {
    assert!(list.len() >= 2);
    let part_point = list.partition_point(|&y| y < x);
    let idx_left: usize;
    let idx_right: usize;

    // all points are to the right -> interpolate the first two points
    if part_point == 0 {
        println!(
            "lookup warning: x = {}, start of lookup_table = {}",
            x,
            list.first().unwrap()
        );
        (idx_left, idx_right) = (0, 1);
    }
    // all points are to the left -> interpolate the last two points
    else if part_point == list.len() {
        println!(
            "lookup warning: x = {}, end of lookup_table = {}",
            x,
            list.last().unwrap()
        );
        (idx_left, idx_right) = (list.len() - 2, list.len() - 1);

    // there are points to the left and to the right
    } else {
        (idx_left, idx_right) = (part_point - 1, part_point);
    }

    // Where should we linearly interpolate?
    let lerp_t = (x - list[idx_left]) / (list[idx_right] - list[idx_left]);
    assert_relative_eq!(lerp(list[idx_left], list[idx_right], lerp_t), x);
    (idx_left, idx_right, lerp_t)
}

// t = 0 -> left
// t = 1 -> right
// linearly in between
fn lerp(left: f64, right: f64, t: f64) -> f64 {
    left + t * (right - left)
}

// return approximate heading to aim the missile to reach the requested target_pitch
pub fn lookup_gravity_heading(gravity: f64, target_pitch: f64, thrust: f64) -> f64 {
    let effective_thrust = gravity_heading_data::GRAVITY * thrust / gravity;
    let (left_thrust_idx, right_thrust_idx, thrust_lerp_t) =
        get_lerp_params(&gravity_heading_data::THRUSTS, effective_thrust);
    let (left_target_pitch_idx, right_target_pitch_idx, target_pitch_lerp_t) =
        get_lerp_params(&gravity_heading_data::TARGET_PITCHES, target_pitch);

    let left_left_heading_pitch_idx =
        left_target_pitch_idx * gravity_heading_data::THRUSTS.len() + left_thrust_idx;
    let left_right_heading_pitch_idx =
        left_target_pitch_idx * gravity_heading_data::THRUSTS.len() + right_thrust_idx;
    let right_left_heading_pitch_idx =
        right_target_pitch_idx * gravity_heading_data::THRUSTS.len() + left_thrust_idx;
    let right_right_heading_pitch_idx =
        right_target_pitch_idx * gravity_heading_data::THRUSTS.len() + right_thrust_idx;

    lerp(
        lerp(
            gravity_heading_data::HEADING_PITCHES_LOOKUP[left_left_heading_pitch_idx],
            gravity_heading_data::HEADING_PITCHES_LOOKUP[left_right_heading_pitch_idx],
            thrust_lerp_t,
        ),
        lerp(
            gravity_heading_data::HEADING_PITCHES_LOOKUP[right_left_heading_pitch_idx],
            gravity_heading_data::HEADING_PITCHES_LOOKUP[right_right_heading_pitch_idx],
            thrust_lerp_t,
        ),
        target_pitch_lerp_t,
    )
    // gravity_heading_data::HEADING_PITCHES_LOOKUP[right_right_heading_pitch_idx]
}
