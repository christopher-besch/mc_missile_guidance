mod gravity_heading_data;

#[cfg(test)]
#[path = "./tests/lookup_test.rs"]
mod lookup_test;

fn get_closest_idx(list: &[f64], x: f64) -> usize {
    assert!(list.len() >= 1);
    let part_point = list.partition_point(|&y| y < x);

    if part_point == list.len() {
        return list.len() - 1;
    }
    if part_point == 0 {
        return 0;
    }
    if (list[part_point - 1] - x).abs() < (list[part_point] - x).abs() {
        return part_point - 1;
    }
    return part_point;
}

// return approximate heading to aim the missile to reach the requested target_pitch
pub fn lookup_gravity_heading(gravity: f64, target_pitch: f64, thrust: f64) -> f64 {
    let effective_thrust = thrust / gravity;
    println!("{}", effective_thrust);
    let thrust_idx = get_closest_idx(&gravity_heading_data::THRUSTS, effective_thrust);
    let target_pitch_idx = get_closest_idx(&gravity_heading_data::TARGET_PITCHES, target_pitch);

    let heading_pitch_idx = target_pitch_idx * gravity_heading_data::THRUSTS.len() + thrust_idx;

    gravity_heading_data::HEADING_PITCHES_LOOKUP[heading_pitch_idx]
}
