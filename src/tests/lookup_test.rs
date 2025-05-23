use approx::assert_relative_eq;

use crate::lookup_tables::lookup_gravity_heading;

#[test]
fn lookup_gravity_heading_test() {
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.2),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.2),
        -88.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.2),
        -86.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.2),
        -83.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.2),
        -82.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.2),
        -80.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.2),
        -78.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.2),
        -76.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.2),
        -74.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.2),
        -72.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.2),
        -70.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.2),
        -68.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.2),
        -65.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.2),
        -64.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.2),
        -62.000000000000014,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.2),
        -60.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.2),
        -58.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.2),
        -56.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.2),
        -54.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.2),
        -52.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.2),
        -49.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.2),
        -47.999999999999986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.2),
        -46.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.2),
        -44.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.2),
        -42.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.2),
        -39.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.2),
        -37.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.2),
        -35.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.2),
        -34.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.2),
        -31.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.2),
        -29.999999999999986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.2),
        -28.00000000000002,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.2),
        -26.000000000000007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.2),
        -24.000000000000004,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.2),
        -22.000000000000007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.2),
        -19.999999999999996,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.2),
        -18.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.2),
        -15.999999999999991,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.2),
        -13.999999999999996,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.2),
        -12.000000000000005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.2),
        -10.000000000000005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.2),
        -7.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.2),
        -6.000000000000009,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.2),
        -3.999999999999995,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.2),
        -2.0000000000000058,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.2),
        6.36110689788582e-15,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.2),
        2.000000000000009,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.2),
        3.999999999999997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.2),
        6.000000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -41.0, 0.2), 8.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.2),
        10.000000000000007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.2),
        12.000000000000007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.2),
        13.999999999999998,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.2),
        15.999999999999996,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -36.0, 0.2), 18.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.2),
        20.000000000000007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.2),
        21.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.2),
        24.000000000000004,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -32.0, 0.2), 26.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.2),
        28.000000000000004,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.2),
        30.000000000000004,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -29.0, 0.2), 32.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, -28.0, 0.2), 34.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.2),
        36.000000000000014,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -26.0, 0.2), 38.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.2),
        40.000000000000014,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.2),
        41.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.2),
        44.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.2),
        46.000000000000014,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.2),
        48.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.2),
        50.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -19.0, 0.2), 52.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.2),
        53.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.2),
        55.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.2),
        57.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.2),
        60.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -14.0, 0.2), 62.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.2),
        64.00000000000003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.2),
        65.99999999999997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.2),
        68.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -10.0, 0.2), 70.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.2),
        72.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.2),
        74.00000000000004,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, -7.0, 0.2), 76.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.2),
        77.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.2),
        79.99999999999997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.2),
        81.99999999999993,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.2),
        83.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.2),
        85.99999999999997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.2),
        88.00000000000013,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.2),
        89.99999957116742,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.2),
        90.0000000000003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.2),
        89.99999999999986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.2),
        89.99999999999993,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.2),
        89.99999999999991,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.2),
        90.00000000000006,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.2),
        90.00000000000003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.2),
        90.00000000000003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.2),
        90.00000000000003,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 12.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 13.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 14.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.2),
        89.99999999999997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 25.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 27.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 28.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 31.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 32.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.2),
        89.99999999999999,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 35.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 38.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 39.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 40.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 42.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 44.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 45.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 46.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 47.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 48.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 49.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 50.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.2),
        90.00000000000001,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 52.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 53.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 54.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 55.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 56.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 57.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 58.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 59.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 60.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 61.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 62.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 63.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 64.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 65.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 66.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 67.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 68.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 69.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 70.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 71.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 72.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 73.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 74.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 75.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 76.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 77.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 78.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 79.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 80.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 81.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 82.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 83.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 84.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 85.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 86.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 87.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 88.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 89.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(lookup_gravity_heading(0.2, 90.0, 0.2), 90.0, epsilon = 3.0);
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.2888888888888889),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.2888888888888889),
        -88.30771061061847,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.2888888888888889),
        -86.61553106096476,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.2888888888888889),
        -84.92357130164147,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.2888888888888889),
        -83.23194150509792,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.2888888888888889),
        -81.54075217679538,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.2888888888888889),
        -79.85011426666107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.2888888888888889),
        -78.16013928092393,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.2888888888888889),
        -76.47093939442271,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.2888888888888889),
        -74.78262756347372,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.2888888888888889),
        -73.09531763938132,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.2888888888888889),
        -71.40912448266936,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.2888888888888889),
        -69.7241640781061,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.2888888888888889),
        -68.04055365058831,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.2888888888888889),
        -66.3584117819424,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.2888888888888889),
        -64.67785852869245,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.2888888888888889),
        -62.999015540833604,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.2888888888888889),
        -61.32200618163917,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.2888888888888889),
        -59.64695564851692,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.2888888888888889),
        -57.973991094915064,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.2888888888888889),
        -56.30324175326362,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.2888888888888889),
        -54.63483905891809,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.2888888888888889),
        -52.96891677505364,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.2888888888888889),
        -51.30561111843522,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.2888888888888889),
        -49.64506088596593,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.2888888888888889),
        -47.987407581888306,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.2888888888888889),
        -46.33279554548474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.2888888888888889),
        -44.68137207909046,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.2888888888888889),
        -43.03328757619783,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.2888888888888889),
        -41.38869564939142,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.2888888888888889),
        -39.74775325781234,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.2888888888888889),
        -38.11062083380365,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.2888888888888889),
        -36.47746240833963,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.2888888888888889),
        -34.848445734788804,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.2888888888888889),
        -33.22374241050131,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.2888888888888889),
        -31.603527995651095,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.2888888888888889),
        -29.987982128695837,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.2888888888888889),
        -28.377288637747252,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.2888888888888889),
        -26.77163564706943,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.2888888888888889),
        -25.171215677843122,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.2888888888888889),
        -23.576225742250763,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.2888888888888889),
        -21.986867429849443,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.2888888888888889),
        -20.403346985108286,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.2888888888888889),
        -18.825875374892767,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.2888888888888889),
        -17.254668344583006,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.2888888888888889),
        -15.689946461414817,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.2888888888888889),
        -14.131935143535003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.2888888888888889),
        -12.580864673165136,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.2888888888888889),
        -11.03697019217323,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.2888888888888889),
        -9.500491678262348,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.2888888888888889),
        -7.971673899900018,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.2888888888888889),
        -6.450766348036609,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.2888888888888889),
        -4.938023142594752,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.2888888888888889),
        -3.433702911660608,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.2888888888888889),
        -1.9380686412727026,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.2888888888888889),
        -0.45138749368944137,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.2888888888888889),
        1.0260694079742358,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.2888888888888889),
        2.4940272308136704,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.2888888888888889),
        3.9522077209083397,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.2888888888888889),
        5.4003295317023,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.2888888888888889),
        6.8381085937462105,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.2888888888888889),
        8.26525852736823,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.2888888888888889),
        9.681491099606266,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.2888888888888889),
        11.086516726448933,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.2888888888888889),
        12.480045021094936,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.2888888888888889),
        13.861785388548343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.2888888888888889),
        15.231447666420506,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.2888888888888889),
        16.588742811307988,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.2888888888888889),
        17.933383629562428,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.2888888888888889),
        19.265085550666107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.2888888888888889),
        20.583567440782048,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.2888888888888889),
        21.888552453366366,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.2888888888888889),
        23.17976891302369,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.2888888888888889),
        24.45695122806446,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.2888888888888889),
        25.719840826499315,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.2888888888888889),
        26.968187109495968,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.2888888888888889),
        28.20174841564464,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.2888888888888889),
        29.42029298874633,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.2888888888888889),
        30.62359994127412,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.2888888888888889),
        31.81146020517843,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.2888888888888889),
        32.98367746133166,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.2888888888888889),
        34.140069038653955,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.2888888888888889),
        35.2804667738431,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.2888888888888889),
        36.40471782266283,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.2888888888888889),
        37.51268541393217,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.2888888888888889),
        38.60424953771179,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.2888888888888889),
        39.67930755969963,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.2888888888888889),
        40.73777475452702,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.2888888888888889),
        41.77958475147647,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.2888888888888889),
        42.804689887112026,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.2888888888888889),
        43.81306146040315,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.2888888888888889),
        44.804689887112026,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.2888888888888889),
        45.77958475147648,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.2888888888888889),
        46.737774754527024,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.2888888888888889),
        47.67930755969963,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.2888888888888889),
        48.6042495377118,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.2888888888888889),
        49.512685413932175,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.2888888888888889),
        50.40471782266284,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.2888888888888889),
        51.28046677384311,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.2888888888888889),
        52.14006903865395,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.2888888888888889),
        52.983677461331666,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.2888888888888889),
        53.811460205178435,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.2888888888888889),
        54.62359994127412,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.2888888888888889),
        55.42029298874633,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.2888888888888889),
        56.20174841564464,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.2888888888888889),
        56.96818710949598,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.2888888888888889),
        57.71984082649932,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.2888888888888889),
        58.45695122806447,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.2888888888888889),
        59.179768913023686,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.2888888888888889),
        59.888552453366366,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.2888888888888889),
        60.58356744078205,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.2888888888888889),
        61.26508555066611,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.2888888888888889),
        61.93338362956242,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.2888888888888889),
        62.588742811308,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.2888888888888889),
        63.23144766642051,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.2888888888888889),
        63.86178538854835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.2888888888888889),
        64.48004502109494,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.2888888888888889),
        65.08651672644893,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.2888888888888889),
        65.68149109960626,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.2888888888888889),
        66.26525852736823,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.2888888888888889),
        66.8381085937462,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.2888888888888889),
        67.4003295317023,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.2888888888888889),
        67.95220772090833,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.2888888888888889),
        68.49402723081367,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.2888888888888889),
        69.02606940797425,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.2888888888888889),
        69.54861250631055,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.2888888888888889),
        70.0619313587273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.2888888888888889),
        70.5662970883394,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.2888888888888889),
        71.06197685740524,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.2888888888888889),
        71.54923365196339,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.2888888888888889),
        72.02832610009997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.2888888888888889),
        72.49950832173765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.2888888888888889),
        72.96302980782677,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.2888888888888889),
        73.41913532683488,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.2888888888888889),
        73.868064856465,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.2888888888888889),
        74.31005353858518,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.2888888888888889),
        74.745331655417,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.2888888888888889),
        75.17412462510725,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.2888888888888889),
        75.59665301489173,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.2888888888888889),
        76.01313257015055,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.2888888888888889),
        76.42377425774926,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.2888888888888889),
        76.82878432215689,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.2888888888888889),
        77.22836435293057,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.2888888888888889),
        77.62271136225273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.2888888888888889),
        78.01201787130417,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.2888888888888889),
        78.39647200434891,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.2888888888888889),
        78.7762575894987,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.2888888888888889),
        79.1515542652112,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.2888888888888889),
        79.52253759166038,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.2888888888888889),
        79.88937916619636,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.2888888888888889),
        80.25224674218765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.2888888888888889),
        80.61130435060858,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.2888888888888889),
        80.96671242380218,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.2888888888888889),
        81.31862792090953,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.2888888888888889),
        81.66720445451527,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.2888888888888889),
        82.0125924181117,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.2888888888888889),
        82.35493911403407,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.2888888888888889),
        82.69438888156479,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.2888888888888889),
        83.03108322494637,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.2888888888888889),
        83.3651609410819,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.2888888888888889),
        83.69675824673638,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.2888888888888889),
        84.02600890508494,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.2888888888888889),
        84.35304435148308,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.2888888888888889),
        84.67799381836083,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.2888888888888889),
        85.0009844591664,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.2888888888888889),
        85.32214147130755,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.2888888888888889),
        85.64158821805762,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.2888888888888889),
        85.9594463494117,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.2888888888888889),
        86.27583592189389,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.2888888888888889),
        86.59087551733063,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.2888888888888889),
        86.90468236061868,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.2888888888888889),
        87.21737243652628,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.2888888888888889),
        87.52906060557729,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.2888888888888889),
        87.83986071907607,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.2888888888888889),
        88.14988573333895,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.2888888888888889),
        88.45924782320463,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.2888888888888889),
        88.76805849490209,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.2888888888888889),
        89.07642869835851,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.2888888888888889),
        89.38446893903524,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.2888888888888889),
        89.69228938938153,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.2888888888888889),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.37777777777777777),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.37777777777777777),
        -88.4706075804679,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.37777777777777777),
        -86.94133124274278,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.37777777777777777),
        -85.41228712243797,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.37777777777777777),
        -83.88359146270874,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.37777777777777777),
        -82.35536066784586,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.37777777777777777),
        -80.82771135665512,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.37777777777777777),
        -79.30076041554894,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.37777777777777777),
        -77.77462505127457,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.37777777777777777),
        -76.24942284320096,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.37777777777777777),
        -74.72527179508332,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.37777777777777777),
        -73.20229038622122,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.37777777777777777),
        -71.68059762192195,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.37777777777777777),
        -70.16031308317699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.37777777777777777),
        -68.641556975454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.37777777777777777),
        -67.12445017650235,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.37777777777777777),
        -65.60911428306359,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.37777777777777777),
        -64.0956716563727,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.37777777777777777),
        -62.58424546632945,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.37777777777777777),
        -61.074959734211454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.37777777777777777),
        -59.567939373794324,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.37777777777777777),
        -58.06331023073518,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.37777777777777777),
        -56.5611991200688,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.37777777777777777),
        -55.06173386165649,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.37777777777777777),
        -53.56504331341947,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.37777777777777777),
        -52.07125740217933,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.37777777777777777),
        -50.580507151918965,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.37777777777777777),
        -49.09292470926823,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.37777777777777777),
        -47.60864336600901,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.37777777777777777),
        -46.127797578384936,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.37777777777777777),
        -44.650522982991866,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.37777777777777777),
        -43.176956409015816,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.37777777777777777),
        -41.707235886576015,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.37777777777777777),
        -40.24150065092283,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.37777777777777777),
        -38.779891142231065,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.37777777777777777),
        -37.32254900072278,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.37777777777777777),
        -35.869617056846344,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.37777777777777777),
        -34.42123931623293,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.37777777777777777),
        -32.977560939146485,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.37777777777777777),
        -31.538728214140296,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.37777777777777777),
        -30.104888525630564,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.37777777777777777),
        -28.67619031509724,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.37777777777777777),
        -27.25278303562354,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.37777777777777777),
        -25.834817099488866,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.37777777777777777),
        -24.42244381853586,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.37777777777777777),
        -23.0158153370396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.37777777777777777),
        -21.615084556818488,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.37777777777777777),
        -20.220405054339313,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.37777777777777777),
        -18.83193098958621,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.37777777777777777),
        -17.449817006483045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.37777777777777777),
        -16.07421812468207,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.37777777777777777),
        -14.705289622559421,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.37777777777777777),
        -13.343186911288372,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.37777777777777777),
        -11.988065399896787,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.37777777777777777),
        -10.640080351253733,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.37777777777777777),
        -9.299386728973358,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.37777777777777777),
        -7.96613903527127,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.37777777777777777),
        -6.640491139859417,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.37777777777777777),
        -5.322596100020937,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.37777777777777777),
        -4.012605972064299,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.37777777777777777),
        -2.7106716144189975,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.37777777777777777),
        -1.4169424826996744,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.37777777777777777),
        -0.1315664171339731,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.37777777777777777),
        1.1453105771805476,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.37777777777777777),
        2.413544556652855,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.37777777777777777),
        3.6729938715977983,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.37777777777777777),
        4.92351940397399,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.37777777777777777),
        6.164984809703185,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.37777777777777777),
        7.397256764741737,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.37777777777777777),
        8.62020521400525,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.37777777777777777),
        9.833703622180687,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.37777777777777777),
        11.037629225397643,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.37777777777777777),
        12.231863282672892,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.37777777777777777),
        13.416291325990898,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.37777777777777777),
        14.59080340783879,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.37777777777777777),
        15.755294344977745,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.37777777777777777),
        16.909663957205733,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.37777777777777777),
        18.053817299848944,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.37777777777777777),
        19.18766488871246,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.37777777777777777),
        20.311122916224903,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.37777777777777777),
        21.42411345752794,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.37777777777777777),
        22.526564665289552,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.37777777777777777),
        23.618410952060085,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.37777777777777777),
        24.6995931590428,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.37777777777777777),
        25.770058710214844,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.37777777777777777),
        26.829761750810874,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.37777777777777777),
        27.87866326926854,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.37777777777777777),
        28.916731201832462,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.37777777777777777),
        29.943940519120126,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.37777777777777777),
        30.960273294067832,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.37777777777777777),
        31.96571875079696,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.37777777777777777),
        32.96027329406783,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.37777777777777777),
        33.943940519120126,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.37777777777777777),
        34.91673120183246,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.37777777777777777),
        35.87866326926854,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.37777777777777777),
        36.829761750810874,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.37777777777777777),
        37.77005871021484,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.37777777777777777),
        38.6995931590428,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.37777777777777777),
        39.61841095206009,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.37777777777777777),
        40.52656466528955,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.37777777777777777),
        41.42411345752794,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.37777777777777777),
        42.311122916224896,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.37777777777777777),
        43.18766488871246,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.37777777777777777),
        44.05381729984895,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.37777777777777777),
        44.90966395720573,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.37777777777777777),
        45.755294344977756,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.37777777777777777),
        46.590803407838784,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.37777777777777777),
        47.4162913259909,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.37777777777777777),
        48.23186328267289,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.37777777777777777),
        49.03762922539765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.37777777777777777),
        49.833703622180685,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.37777777777777777),
        50.620205214005246,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.37777777777777777),
        51.39725676474175,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.37777777777777777),
        52.16498480970319,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.37777777777777777),
        52.92351940397399,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.37777777777777777),
        53.672993871597804,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.37777777777777777),
        54.413544556652866,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.37777777777777777),
        55.14531057718055,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.37777777777777777),
        55.86843358286604,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.37777777777777777),
        56.583057517300325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.37777777777777777),
        57.289328385581,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.37777777777777777),
        57.98739402793571,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.37777777777777777),
        58.67740389997906,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.37777777777777777),
        59.35950886014058,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.37777777777777777),
        60.033860964728746,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.37777777777777777),
        60.70061327102664,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.37777777777777777),
        61.35991964874627,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.37777777777777777),
        62.01193460010322,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.37777777777777777),
        62.65681308871164,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.37777777777777777),
        63.29471037744057,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.37777777777777777),
        63.925781875317924,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.37777777777777777),
        64.55018299351697,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.37777777777777777),
        65.1680690104138,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.37777777777777777),
        65.7795949456607,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.37777777777777777),
        66.3849154431815,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.37777777777777777),
        66.9841846629604,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.37777777777777777),
        67.57755618146416,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.37777777777777777),
        68.16518290051114,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.37777777777777777),
        68.74721696437646,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.37777777777777777),
        69.32380968490276,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.37777777777777777),
        69.89511147436943,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.37777777777777777),
        70.46127178585971,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.37777777777777777),
        71.02243906085351,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.37777777777777777),
        71.57876068376707,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.37777777777777777),
        72.13038294315365,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.37777777777777777),
        72.67745099927723,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.37777777777777777),
        73.22010885776893,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.37777777777777777),
        73.75849934907717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.37777777777777777),
        74.292764113424,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.37777777777777777),
        74.8230435909842,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.37777777777777777),
        75.34947701700811,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.37777777777777777),
        75.87220242161506,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.37777777777777777),
        76.391356633991,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.37777777777777777),
        76.90707529073177,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.37777777777777777),
        77.41949284808103,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.37777777777777777),
        77.92874259782067,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.37777777777777777),
        78.43495668658053,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.37777777777777777),
        78.93826613834352,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.37777777777777777),
        79.4388008799312,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.37777777777777777),
        79.9366897692648,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.37777777777777777),
        80.43206062620567,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.37777777777777777),
        80.92504026578854,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.37777777777777777),
        81.41575453367057,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.37777777777777777),
        81.9043283436273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.37777777777777777),
        82.39088571693642,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.37777777777777777),
        82.87554982349766,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.37777777777777777),
        83.35844302454602,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.37777777777777777),
        83.83968691682304,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.37777777777777777),
        84.31940237807804,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.37777777777777777),
        84.79770961377878,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.37777777777777777),
        85.27472820491668,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.37777777777777777),
        85.75057715679904,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.37777777777777777),
        86.22537494872543,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.37777777777777777),
        86.69923958445106,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.37777777777777777),
        87.17228864334488,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.37777777777777777),
        87.64463933215416,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.37777777777777777),
        88.11640853729128,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.37777777777777777),
        88.58771287756201,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.37777777777777777),
        89.05866875725721,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.37777777777777777),
        89.5293924195321,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.37777777777777777),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.4666666666666667),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.4666666666666667),
        -88.5714463335539,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.4666666666666667),
        -87.14299924409806,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.4666666666666667),
        -85.71476532979732,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.4666666666666667),
        -84.28685123108694,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.4666666666666667),
        -82.85936365161136,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.4666666666666667),
        -81.43240937892651,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.4666666666666667),
        -80.00609530488639,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.4666666666666667),
        -78.58052844563396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.4666666666666667),
        -77.15581596111555,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.4666666666666667),
        -75.73206517403722,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.4666666666666667),
        -74.30938358818095,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.4666666666666667),
        -72.88787890599663,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.4666666666666667),
        -71.46765904538567,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.4666666666666667),
        -70.04883215558996,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.4666666666666667),
        -68.63150663209929,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.4666666666666667),
        -67.21579113048824,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.4666666666666667),
        -65.80179457909297,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.4666666666666667),
        -64.38962619043593,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.4666666666666667),
        -62.97939547130593,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.4666666666666667),
        -61.57121223139891,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.4666666666666667),
        -60.16518659042367,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.4666666666666667),
        -58.76142898357527,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.4666666666666667),
        -57.36005016527758,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.4666666666666667),
        -55.9611612110955,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.4666666666666667),
        -54.564873517715704,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.4666666666666667),
        -53.17129880089469,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.4666666666666667),
        -51.78054909127166,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.4666666666666667),
        -50.39273672794355,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.4666666666666667),
        -49.00797434969941,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.4666666666666667),
        -47.62637488381143,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.4666666666666667),
        -46.248051532280435,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.4666666666666667),
        -44.873117755434386,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.4666666666666667),
        -43.50168725278047,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.4666666666666667),
        -42.13387394101211,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.4666666666666667),
        -40.769791929075474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.4666666666666667),
        -39.409555490202706,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.4666666666666667),
        -38.053279030822274,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.4666666666666667),
        -36.701077056261774,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.4666666666666667),
        -35.353064133162455,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.4666666666666667),
        -34.00935484853136,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.4666666666666667),
        -32.67006376536236,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.4666666666666667),
        -31.335305374765547,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.4666666666666667),
        -30.005194044551533,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.4666666666666667),
        -28.67984396422699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.4666666666666667),
        -27.35936908636698,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.4666666666666667),
        -26.0438830643406,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.4666666666666667),
        -24.73349918637803,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.4666666666666667),
        -23.42833030597977,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.4666666666666667),
        -22.128488768682264,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.4666666666666667),
        -20.834086335208433,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.4666666666666667),
        -19.545234101047193,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.4666666666666667),
        -18.262042412521918,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.4666666666666667),
        -16.98462077942495,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.4666666666666667),
        -15.71307778431302,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.4666666666666667),
        -14.44752098857662,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.4666666666666667),
        -13.188056835415757,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.4666666666666667),
        -11.93479054987362,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.4666666666666667),
        -10.687826036100159,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.4666666666666667),
        -9.447265772037282,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.4666666666666667),
        -8.213210701738184,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.4666666666666667),
        -6.98576012555349,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.4666666666666667),
        -5.765011588437261,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.4666666666666667),
        -4.551060766645919,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.4666666666666667),
        -3.3440013531223602,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.4666666666666667),
        -2.1439249418766915,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.4666666666666667),
        -0.9509209116926967,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.4666666666666667),
        0.23492369049392287,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.4666666666666667),
        1.4135242661837197,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.4666666666666667),
        2.5847987814923528,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.4666666666666667),
        3.748667882531537,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.4666666666666667),
        4.905055009593064,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.4666666666666667),
        6.053886509728045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.4666666666666667),
        7.195091747306974,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.4666666666666667),
        8.328603212142324,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.4666666666666667),
        9.454356624753245,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.4666666666666667),
        10.572291038353326,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.4666666666666667),
        11.682348937145736,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.4666666666666667),
        12.784476330516503,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.4666666666666667),
        13.878622842725994,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.4666666666666667),
        14.964741797710653,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.4666666666666667),
        16.042790298622002,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.4666666666666667),
        17.112729301747546,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.4666666666666667),
        18.17452368447878,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.4666666666666667),
        19.22814230701424,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.4666666666666667),
        20.273558067511452,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.4666666666666667),
        21.31074795042894,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.4666666666666667),
        22.339693067829913,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.4666666666666667),
        23.360378693450603,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.4666666666666667),
        24.37279428936997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.4666666666666667),
        25.376933525152303,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.4666666666666667),
        26.372794289369974,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.4666666666666667),
        27.3603786934506,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.4666666666666667),
        28.339693067829913,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.4666666666666667),
        29.31074795042894,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.4666666666666667),
        30.27355806751145,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.4666666666666667),
        31.22814230701425,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.4666666666666667),
        32.17452368447878,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.4666666666666667),
        33.11272930174755,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.4666666666666667),
        34.042790298622,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.4666666666666667),
        34.96474179771066,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.4666666666666667),
        35.878622842725996,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.4666666666666667),
        36.784476330516505,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.4666666666666667),
        37.68234893714573,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.4666666666666667),
        38.57229103835333,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.4666666666666667),
        39.45435662475325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.4666666666666667),
        40.32860321214233,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.4666666666666667),
        41.195091747306975,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.4666666666666667),
        42.053886509728045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.4666666666666667),
        42.90505500959306,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.4666666666666667),
        43.748667882531535,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.4666666666666667),
        44.58479878149236,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.4666666666666667),
        45.41352426618372,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.4666666666666667),
        46.23492369049392,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.4666666666666667),
        47.049079088307316,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.4666666666666667),
        47.85607505812331,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.4666666666666667),
        48.65599864687765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.4666666666666667),
        49.44893923335408,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.4666666666666667),
        50.23498841156274,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.4666666666666667),
        51.014239874446524,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.4666666666666667),
        51.78678929826181,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.4666666666666667),
        52.55273422796272,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.4666666666666667),
        53.312173963899845,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.4666666666666667),
        54.06520945012639,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.4666666666666667),
        54.81194316458424,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.4666666666666667),
        55.55247901142338,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.4666666666666667),
        56.28692221568699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.4666666666666667),
        57.015379220575056,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.4666666666666667),
        57.73795758747808,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.4666666666666667),
        58.45476589895281,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.4666666666666667),
        59.16591366479157,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.4666666666666667),
        59.87151123131775,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.4666666666666667),
        60.57166969402023,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.4666666666666667),
        61.266500813621974,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.4666666666666667),
        61.956116935659395,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.4666666666666667),
        62.64063091363302,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.4666666666666667),
        63.32015603577302,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.4666666666666667),
        63.99480595544848,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.4666666666666667),
        64.66469462523446,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.4666666666666667),
        65.32993623463763,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.4666666666666667),
        65.99064515146864,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.4666666666666667),
        66.64693586683754,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.4666666666666667),
        67.29892294373823,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.4666666666666667),
        67.94672096917772,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.4666666666666667),
        68.5904445097973,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.4666666666666667),
        69.23020807092453,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.4666666666666667),
        69.8661260589879,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.4666666666666667),
        70.49831274721954,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.4666666666666667),
        71.12688224456562,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.4666666666666667),
        71.75194846771959,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.4666666666666667),
        72.37362511618856,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.4666666666666667),
        72.99202565030059,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.4666666666666667),
        73.60726327205644,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.4666666666666667),
        74.21945090872835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.4666666666666667),
        74.82870119910531,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.4666666666666667),
        75.4351264822843,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.4666666666666667),
        76.03883878890451,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.4666666666666667),
        76.63994983472243,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.4666666666666667),
        77.23857101642474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.4666666666666667),
        77.83481340957631,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.4666666666666667),
        78.42878776860108,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.4666666666666667),
        79.02060452869407,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.4666666666666667),
        79.61037380956407,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.4666666666666667),
        80.19820542090703,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.4666666666666667),
        80.78420886951177,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.4666666666666667),
        81.36849336790073,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.4666666666666667),
        81.95116784441004,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.4666666666666667),
        82.53234095461434,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.4666666666666667),
        83.11212109400337,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.4666666666666667),
        83.69061641181905,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.4666666666666667),
        84.26793482596277,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.4666666666666667),
        84.84418403888445,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.4666666666666667),
        85.41947155436604,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.4666666666666667),
        85.99390469511363,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.4666666666666667),
        86.5675906210735,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.4666666666666667),
        87.14063634838865,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.4666666666666667),
        87.71314876891307,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.4666666666666667),
        88.28523467020268,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.4666666666666667),
        88.85700075590192,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.4666666666666667),
        89.4285536664461,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.4666666666666667),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.5555555555555556),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.5555555555555556),
        -88.64001590838043,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.5555555555555556),
        -87.28012726800968,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.5555555555555556),
        -85.92042953495442,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.5555555555555556),
        -84.56101817485616,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.5555555555555556),
        -83.20198866756598,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.5555555555555556),
        -81.84343651159672,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.5555555555555556),
        -80.48545722833073,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.5555555555555556),
        -79.12814636592283,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.5555555555555556),
        -77.771599502837,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.5555555555555556),
        -76.415912250956,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.5555555555555556),
        -75.06118025820237,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.5555555555555556),
        -73.70749921061031,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.5555555555555556),
        -72.35496483378672,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.5555555555555556),
        -71.0036728937008,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.5555555555555556),
        -69.65371919674106,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.5555555555555556),
        -68.30519958897894,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.5555555555555556),
        -66.95820995457818,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.5555555555555556),
        -65.6128462132899,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.5555555555555556),
        -64.26920431697282,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.5555555555555556),
        -62.92738024507917,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.5555555555555556),
        -61.587469999046796,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.5555555555555556),
        -60.24956959553897,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.5555555555555556),
        -58.913775058473725,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.5555555555555556),
        -57.580182409785785,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.5555555555555556),
        -56.24888765886466,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.5555555555555556),
        -54.919986790614054,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.5555555555555556),
        -53.59357575207883,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.5555555555555556),
        -52.269750437587184,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.5555555555555556),
        -50.94860667235773,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.5555555555555556),
        -49.63024019452258,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.5555555555555556),
        -48.31474663552022,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.5555555555555556),
        -47.00222149881394,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.5555555555555556),
        -45.692760136894414,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.5555555555555556),
        -44.38645772652805,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.5555555555555556),
        -43.08340924221538,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.5555555555555556),
        -41.78370942782822,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.5555555555555556),
        -40.48745276639708,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.5555555555555556),
        -39.19473344802521,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.5555555555555556),
        -37.90564533590956,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.5555555555555556),
        -36.620281930454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.5555555555555556),
        -35.33873633146524,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.5555555555555556),
        -34.06110119842693,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.5555555555555556),
        -32.78746870885363,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.5555555555555556),
        -31.517930514732193,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.5555555555555556),
        -30.252577697064417,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.5555555555555556),
        -28.991500718531704,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.5555555555555556),
        -27.734789374309337,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.5555555555555556),
        -26.48253274106533,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.5555555555555556),
        -25.23481912418649,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.5555555555555556),
        -23.991736003281716,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.5555555555555556),
        -22.753369976021215,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.5555555555555556),
        -21.51980670037789,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.5555555555555556),
        -20.291130835346,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.5555555555555556),
        -19.067425980220396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.5555555555555556),
        -17.84877461252835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.5555555555555556),
        -16.635258024714464,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.5555555555555556),
        -15.426956259687774,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.5555555555555556),
        -14.223948045348932,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.5555555555555556),
        -13.026310728223105,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.5555555555555556),
        -11.8341202063332,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.5555555555555556),
        -10.647450861455496,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.5555555555555556),
        -9.46637549090753,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.5555555555555556),
        -8.290965239025695,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.5555555555555556),
        -7.121289528496366,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.5555555555555556),
        -5.957415991711273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.5555555555555556),
        -4.799410402323468,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.5555555555555556),
        -3.6473366071853413,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.5555555555555556),
        -2.5012564588549884,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.5555555555555556),
        -1.3612297488607144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.5555555555555556),
        -0.22731414191675184,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.5555555555555556),
        0.9004348887147936,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.5555555555555556),
        2.021964124519315,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.5555555555555556),
        3.1372226634856784,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.5555555555555556),
        4.24616198012683,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.5555555555555556),
        5.348735983570461,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.5555555555555556),
        6.444901073528128,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.5555555555555556),
        7.53461619395474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.5555555555555556),
        8.617842884215191,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.5555555555555556),
        9.69454532758074,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.5555555555555556),
        10.76469039688451,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.5555555555555556),
        11.828247697173538,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.5555555555555556),
        12.88518960520343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.5555555555555556),
        13.935491305631759,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.5555555555555556),
        14.979130823776835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.5555555555555556),
        16.0160890548202,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.5555555555555556),
        17.046349789343505,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.5555555555555556),
        18.069899735103444,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.5555555555555556),
        19.086728534962283,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.5555555555555556),
        20.096828780905497,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.5555555555555556),
        21.100196024093023,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.5555555555555556),
        22.096828780905497,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.5555555555555556),
        23.086728534962287,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.5555555555555556),
        24.069899735103448,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.5555555555555556),
        25.046349789343502,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.5555555555555556),
        26.0160890548202,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.5555555555555556),
        26.979130823776835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.5555555555555556),
        27.93549130563176,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.5555555555555556),
        28.885189605203433,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.5555555555555556),
        29.828247697173538,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.5555555555555556),
        30.764690396884507,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.5555555555555556),
        31.694545327580734,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.5555555555555556),
        32.61784288421519,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.5555555555555556),
        33.53461619395474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.5555555555555556),
        34.44490107352812,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.5555555555555556),
        35.348735983570464,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.5555555555555556),
        36.24616198012683,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.5555555555555556),
        37.13722266348568,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.5555555555555556),
        38.02196412451932,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.5555555555555556),
        38.900434888714805,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.5555555555555556),
        39.77268585808325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.5555555555555556),
        40.638770251139285,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.5555555555555556),
        41.49874354114501,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.5555555555555556),
        42.35266339281466,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.5555555555555556),
        43.20058959767654,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.5555555555555556),
        44.042584008288735,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.5555555555555556),
        44.878710471503645,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.5555555555555556),
        45.709034760974305,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.5555555555555556),
        46.53362450909247,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.5555555555555556),
        47.35254913854451,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.5555555555555556),
        48.16587979366679,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.5555555555555556),
        48.9736892717769,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.5555555555555556),
        49.77605195465107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.5555555555555556),
        50.57304374031223,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.5555555555555556),
        51.364741975285554,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.5555555555555556),
        52.15122538747164,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.5555555555555556),
        52.93257401977961,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.5555555555555556),
        53.708869164654004,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.5555555555555556),
        54.48019329962211,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.5555555555555556),
        55.24663002397879,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.5555555555555556),
        56.00826399671828,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.5555555555555556),
        56.76518087581351,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.5555555555555556),
        57.51746725893467,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.5555555555555556),
        58.26521062569067,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.5555555555555556),
        59.00849928146829,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.5555555555555556),
        59.747422302935576,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.5555555555555556),
        60.48206948526781,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.5555555555555556),
        61.21253129114638,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.5555555555555556),
        61.93889880157308,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.5555555555555556),
        62.66126366853476,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.5555555555555556),
        63.379718069546,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.5555555555555556),
        64.09435466409045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.5555555555555556),
        64.8052665519748,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.5555555555555556),
        65.51254723360292,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.5555555555555556),
        66.21629057217177,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.5555555555555556),
        66.91659075778462,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.5555555555555556),
        67.61354227347196,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.5555555555555556),
        68.30723986310558,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.5555555555555556),
        68.99777850118609,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.5555555555555556),
        69.68525336447979,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.5555555555555556),
        70.36975980547741,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.5555555555555556),
        71.05139332764226,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.5555555555555556),
        71.73024956241281,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.5555555555555556),
        72.40642424792117,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.5555555555555556),
        73.08001320938595,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.5555555555555556),
        73.75111234113534,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.5555555555555556),
        74.41981759021422,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.5555555555555556),
        75.08622494152628,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.5555555555555556),
        75.75043040446104,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.5555555555555556),
        76.4125300009532,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.5555555555555556),
        77.07261975492082,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.5555555555555556),
        77.73079568302717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.5555555555555556),
        78.38715378671012,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.5555555555555556),
        79.04179004542183,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.5555555555555556),
        79.69480041102106,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.5555555555555556),
        80.34628080325895,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.5555555555555556),
        80.99632710629922,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.5555555555555556),
        81.6450351662133,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.5555555555555556),
        82.29250078938969,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.5555555555555556),
        82.93881974179762,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.5555555555555556),
        83.584087749044,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.5555555555555556),
        84.228400497163,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.5555555555555556),
        84.87185363407717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.5555555555555556),
        85.51454277166927,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.5555555555555556),
        86.1565634884033,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.5555555555555556),
        86.79801133243403,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.5555555555555556),
        87.43898182514386,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.5555555555555556),
        88.07957046504556,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.5555555555555556),
        88.71987273199032,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.5555555555555556),
        89.35998409161957,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.5555555555555556),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.6444444444444445),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.6444444444444445),
        -88.68966941092926,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.6444444444444445),
        -87.3794242522572,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.6444444444444445),
        -86.06934995090198,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.6444444444444445),
        -84.75953192677669,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.6444444444444445),
        -83.45005558917603,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.6444444444444445),
        -82.14100633303083,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.6444444444444445),
        -80.83246953498579,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.6444444444444445),
        -79.5245305492568,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.6444444444444445),
        -78.21727470322426,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.6444444444444445),
        -76.9107872927188,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.6444444444444445),
        -75.60515357695624,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.6444444444444445),
        -74.30045877307909,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.6444444444444445),
        -72.99678805026173,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.6444444444444445),
        -71.69422652333775,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.6444444444444445),
        -70.39285924590729,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.6444444444444445),
        -69.09277120288375,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.6444444444444445),
        -67.79404730243925,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.6444444444444445),
        -66.49677236730908,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.6444444444444445),
        -65.20103112541608,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.6444444444444445),
        -63.90690819977707,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.6444444444444445),
        -62.614488097653535,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.6444444444444445),
        -61.32385519891091,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.6444444444444445),
        -60.03509374355082,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.6444444444444445),
        -58.748287818382636,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.6444444444444445),
        -57.46352134280165,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.6444444444444445),
        -56.180878053642644,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.6444444444444445),
        -54.90044148907908,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.6444444444444445),
        -53.62229497154032,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.6444444444444445),
        -52.346521589620174,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.6444444444444445),
        -51.07320417895298,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.6444444444444445),
        -49.80242530203473,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.6444444444444445),
        -48.53426722696933,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.6444444444444445),
        -47.26881190512257,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.6444444444444445),
        -46.00614094766843,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.6444444444444445),
        -44.74633560101531,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.6444444444444445),
        -43.48947672110266,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.6444444444444445),
        -42.235644746561164,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.6444444444444445),
        -40.98491967073292,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.6444444444444445),
        -39.73738101255129,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.6444444444444445),
        -38.493107786283396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.6444444444444445),
        -37.25217847014209,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.6444444444444445),
        -36.01467097377739,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.6444444444444445),
        -34.78066260466172,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.6444444444444445),
        -33.55023003338707,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.6444444444444445),
        -32.323449257896016,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.6444444444444445),
        -31.10039556667319,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.6444444444444445),
        -29.881143500927557,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.6444444444444445),
        -28.665766815800723,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.6444444444444445),
        -27.454338440640512,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.6444444444444445),
        -26.24693043838377,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.6444444444444445),
        -25.043613964096856,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.6444444444444445),
        -23.844459222726705,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.6444444444444445),
        -22.6495354261201,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.6444444444444445),
        -21.458910749373096,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.6444444444444445),
        -20.27265228657721,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.6444444444444445),
        -19.09082600603316,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.6444444444444445),
        -17.913496705007542,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.6444444444444445),
        -16.74072796411185,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.6444444444444445),
        -15.57258210138721,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.6444444444444445),
        -14.409120126182318,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.6444444444444445),
        -13.250401692915556,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.6444444444444445),
        -12.096485054815776,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.6444444444444445),
        -10.94742701773936,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.6444444444444445),
        -9.803282894164228,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.6444444444444445),
        -8.664106457463799,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.6444444444444445),
        -7.529949896566501,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.6444444444444445),
        -6.400863771107968,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.6444444444444445),
        -5.276896967184831,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.6444444444444445),
        -4.1580966538200315,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.6444444444444445),
        -3.044508240250151,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.6444444444444445),
        -1.9361753341456795,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.6444444444444445),
        -0.8331397008746749,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.6444444444444445),
        0.26455877608015543,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.6444444444444445),
        1.3568821334425443,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.6444444444444445),
        2.4437943650946625,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.6444444444444445),
        3.5252614575555143,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.6444444444444445),
        4.601251423883464,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.6444444444444445),
        5.671734335904107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.6444444444444445),
        6.736682354668166,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.6444444444444445),
        7.796069759048454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.6444444444444445),
        8.849872972389361,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.6444444444444445),
        9.898070587127547,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.6444444444444445),
        10.940643387307972,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.6444444444444445),
        11.977574368925273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.6444444444444445),
        13.008848758026835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.6444444444444445),
        14.034454026520493,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.6444444444444445),
        15.05437990563678,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.6444444444444445),
        16.068618397002833,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.6444444444444445),
        17.077163781292537,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.6444444444444445),
        18.080012624425155,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.6444444444444445),
        19.077163781292533,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.6444444444444445),
        20.068618397002833,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.6444444444444445),
        21.054379905636782,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.6444444444444445),
        22.034454026520493,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.6444444444444445),
        23.008848758026836,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.6444444444444445),
        23.97757436892527,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.6444444444444445),
        24.940643387307976,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.6444444444444445),
        25.89807058712755,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.6444444444444445),
        26.849872972389356,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.6444444444444445),
        27.796069759048454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.6444444444444445),
        28.736682354668165,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.6444444444444445),
        29.6717343359041,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.6444444444444445),
        30.60125142388346,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.6444444444444445),
        31.525261457555512,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.6444444444444445),
        32.44379436509466,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.6444444444444445),
        33.35688213344255,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.6444444444444445),
        34.264558776080165,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.6444444444444445),
        35.166860299125325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.6444444444444445),
        36.06382466585433,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.6444444444444445),
        36.955491759749854,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.6444444444444445),
        37.84190334617997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.6444444444444445),
        38.72310303281517,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.6444444444444445),
        39.59913622889204,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.6444444444444445),
        40.4700501034335,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.6444444444444445),
        41.335893542536205,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.6444444444444445),
        42.19671710583577,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.6444444444444445),
        43.05257298226064,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.6444444444444445),
        43.903514945184234,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.6444444444444445),
        44.749598307084455,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.6444444444444445),
        45.590879873817684,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.6444444444444445),
        46.42741789861279,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.6444444444444445),
        47.259272035888145,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.6444444444444445),
        48.08650329499247,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.6444444444444445),
        48.90917399396685,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.6444444444444445),
        49.72734771342279,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.6444444444444445),
        50.541089250626904,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.6444444444444445),
        51.350464573879904,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.6444444444444445),
        52.15554077727329,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.6444444444444445),
        52.95638603590314,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.6444444444444445),
        53.75306956161622,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.6444444444444445),
        54.545661559359495,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.6444444444444445),
        55.33423318419928,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.6444444444444445),
        56.11885649907246,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.6444444444444445),
        56.89960443332681,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.6444444444444445),
        57.67655074210398,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.6444444444444445),
        58.44976996661294,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.6444444444444445),
        59.219337395338286,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.6444444444444445),
        59.985329026222615,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.6444444444444445),
        60.7478215298579,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.6444444444444445),
        61.50689221371661,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.6444444444444445),
        62.262618987448725,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.6444444444444445),
        63.015080329267086,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.6444444444444445),
        63.764355253438836,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.6444444444444445),
        64.51052327889735,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.6444444444444445),
        65.2536643989847,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.6444444444444445),
        65.99385905233159,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.6444444444444445),
        66.73118809487744,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.6444444444444445),
        67.4657327730307,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.6444444444444445),
        68.1975746979653,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.6444444444444445),
        68.92679582104701,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.6444444444444445),
        69.65347841037982,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.6444444444444445),
        70.37770502845969,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.6444444444444445),
        71.09955851092091,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.6444444444444445),
        71.81912194635736,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.6444444444444445),
        72.53647865719836,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.6444444444444445),
        73.25171218161738,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.6444444444444445),
        73.9649062564492,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.6444444444444445),
        74.6761448010891,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.6444444444444445),
        75.38551190234645,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.6444444444444445),
        76.09309180022294,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.6444444444444445),
        76.7989688745839,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.6444444444444445),
        77.50322763269092,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.6444444444444445),
        78.20595269756075,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.6444444444444445),
        78.90722879711626,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.6444444444444445),
        79.60714075409273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.6444444444444445),
        80.30577347666227,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.6444444444444445),
        81.00321194973829,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.6444444444444445),
        81.69954122692091,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.6444444444444445),
        82.39484642304375,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.6444444444444445),
        83.0892127072812,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.6444444444444445),
        83.78272529677574,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.6444444444444445),
        84.4754694507432,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.6444444444444445),
        85.16753046501422,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.6444444444444445),
        85.85899366696918,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.6444444444444445),
        86.54994441082398,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.6444444444444445),
        87.24046807322333,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.6444444444444445),
        87.930650049098,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.6444444444444445),
        88.6205757477428,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.6444444444444445),
        89.31033058907073,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.6444444444444445),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.7333333333333334),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.7333333333333334),
        -88.72728554356846,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.7333333333333334),
        -87.45464798336177,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.7333333333333334),
        -86.18216420785059,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.7333333333333334),
        -84.90991108996539,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.7333333333333334),
        -83.63796547924707,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.7333333333333334),
        -82.36640419390262,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.7333333333333334),
        -81.09530401273432,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.7333333333333334),
        -79.82474166691102,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.7333333333333334),
        -78.55479383155097,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.7333333333333334),
        -77.28553711708477,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.7333333333333334),
        -76.01704806036864,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.7333333333333334),
        -74.74940311551745,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.7333333333333334),
        -73.48267864442828,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.7333333333333334),
        -72.21695090696502,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.7333333333333334),
        -70.95229605077589,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.7333333333333334),
        -69.68879010071525,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.7333333333333334),
        -68.42650894784313,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.7333333333333334),
        -67.16552833797508,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.7333333333333334),
        -65.90592385975714,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.7333333333333334),
        -64.64777093224075,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.7333333333333334),
        -63.39114479193328,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.7333333333333334),
        -62.13612047930157,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.7333333333333334),
        -60.882772824706045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.7333333333333334),
        -59.63117643374494,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.7333333333333334),
        -58.381405671988496,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.7333333333333334),
        -57.13353464908525,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.7333333333333334),
        -55.887637202222855,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.7333333333333334),
        -54.64378687892852,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.7333333333333334),
        -53.402056919194344,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.7333333333333334),
        -52.16252023691571,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.7333333333333334),
        -50.92524940063161,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.7333333333333334),
        -49.69031661355813,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.7333333333333334),
        -48.45779369290809,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.7333333333333334),
        -47.22775204849157,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.7333333333333334),
        -46.00026266059409,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.7333333333333334),
        -44.77539605713192,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.7333333333333334),
        -43.55322229008523,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.7333333333333334),
        -42.333810911212815,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.7333333333333334),
        -41.117230947054146,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.7333333333333334),
        -39.90355087322695,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.7333333333333334),
        -38.6928385880311,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.7333333333333334),
        -37.485161385371946,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.7333333333333334),
        -36.28058592701893,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.7333333333333334),
        -35.0791782142182,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.7333333333333334),
        -33.88100355868,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.7333333333333334),
        -32.68612655296505,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.7333333333333334),
        -31.494611040296338,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.7333333333333334),
        -30.306520083825877,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.7333333333333334),
        -29.12191593538853,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.7333333333333334),
        -27.940860003777765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.7333333333333334),
        -26.763412822581117,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.7333333333333334),
        -25.58963401761579,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.7333333333333334),
        -24.41958227400735,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.7333333333333334),
        -23.2533153029575,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.7333333333333334),
        -22.090889808248985,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.7333333333333334),
        -20.93236145253863,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.7333333333333334),
        -19.777784823491597,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.7333333333333334),
        -18.627213399812536,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.7333333333333334),
        -17.48069951723114,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.7333333333333334),
        -16.338294334501995,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.7333333333333334),
        -15.200047799480274,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.7333333333333334),
        -14.066008615336678,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.7333333333333334),
        -12.936224206976684,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.7333333333333334),
        -11.81074068773015,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.7333333333333334),
        -10.689602826378986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.7333333333333334),
        -9.572854014591266,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.7333333333333334),
        -8.460536234830863,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.7333333333333334),
        -7.352690028812397,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.7333333333333334),
        -6.249354466571279,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.7333333333333334),
        -5.1505671162188325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.7333333333333334),
        -4.056364014452107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.7333333333333334),
        -2.9667796378874343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.7333333333333334),
        -1.8818468752861222,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.7333333333333334),
        -0.8015970007393333,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.7333333333333334),
        0.2739403521218054,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.7333333333333334),
        1.3447372148268482,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.7333333333333334),
        2.410767307614956,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.7333333333333334),
        3.4720060617720967,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.7333333333333334),
        4.528430640698782,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.7333333333333334),
        5.58001995965338,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.7333333333333334),
        6.626754704119007,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.7333333333333334),
        7.668617346745142,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.7333333333333334),
        8.705592162818599,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.7333333333333334),
        9.737665244222052,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.7333333333333334),
        10.764824511842217,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.7333333333333334),
        11.787059726393762,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.7333333333333334),
        12.804362497629192,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.7333333333333334),
        13.816726291909337,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.7333333333333334),
        14.824146438113416,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.7333333333333334),
        15.826620131872344,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.7333333333333334),
        16.824146438113416,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.7333333333333334),
        17.816726291909333,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.7333333333333334),
        18.804362497629196,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.7333333333333334),
        19.78705972639376,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.7333333333333334),
        20.764824511842217,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.7333333333333334),
        21.737665244222054,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.7333333333333334),
        22.705592162818597,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.7333333333333334),
        23.668617346745144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.7333333333333334),
        24.626754704119005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.7333333333333334),
        25.580019959653377,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.7333333333333334),
        26.528430640698783,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.7333333333333334),
        27.4720060617721,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.7333333333333334),
        28.41076730761496,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.7333333333333334),
        29.34473721482685,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.7333333333333334),
        30.273940352121805,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.7333333333333334),
        31.19840299926066,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.7333333333333334),
        32.11815312471388,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.7333333333333334),
        33.03322036211256,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.7333333333333334),
        33.94363598554789,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.7333333333333334),
        34.84943288378117,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.7333333333333334),
        35.75064553342872,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.7333333333333334),
        36.6473099711876,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.7333333333333334),
        37.53946376516914,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.7333333333333334),
        38.42714598540874,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.7333333333333334),
        39.31039717362102,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.7333333333333334),
        40.189259312269854,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.7333333333333334),
        41.06377579302332,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.7333333333333334),
        41.93399138466332,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.7333333333333334),
        42.79995220051973,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.7333333333333334),
        43.661705665498005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.7333333333333334),
        44.51930048276887,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.7333333333333334),
        45.37278660018747,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.7333333333333334),
        46.22221517650841,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.7333333333333334),
        47.06763854746138,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.7333333333333334),
        47.90911019175102,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.7333333333333334),
        48.746684697042504,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.7333333333333334),
        49.58041772599265,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.7333333333333334),
        50.410365982384214,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.7333333333333334),
        51.236587177418876,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.7333333333333334),
        52.059139996222235,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.7333333333333334),
        52.87808406461147,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.7333333333333334),
        53.69347991617412,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.7333333333333334),
        54.505388959703666,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.7333333333333334),
        55.31387344703494,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.7333333333333334),
        56.118996441319986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.7333333333333334),
        56.920821785781804,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.7333333333333334),
        57.71941407298108,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.7333333333333334),
        58.514838614628076,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.7333333333333334),
        59.3071614119689,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.7333333333333334),
        60.096449126773045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.7333333333333334),
        60.88276905294587,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.7333333333333334),
        61.666189088787185,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.7333333333333334),
        62.44677770991476,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.7333333333333334),
        63.22460394286808,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.7333333333333334),
        63.99973733940591,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.7333333333333334),
        64.77224795150843,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.7333333333333334),
        65.5422063070919,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.7333333333333334),
        66.30968338644189,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.7333333333333334),
        67.07475059936841,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.7333333333333334),
        67.83747976308429,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.7333333333333334),
        68.59794308080565,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.7333333333333334),
        69.35621312107148,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.7333333333333334),
        70.11236279777714,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.7333333333333334),
        70.86646535091475,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.7333333333333334),
        71.61859432801151,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.7333333333333334),
        72.36882356625507,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.7333333333333334),
        73.11722717529396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.7333333333333334),
        73.86387952069845,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.7333333333333334),
        74.60885520806671,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.7333333333333334),
        75.35222906775924,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.7333333333333334),
        76.09407614024285,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.7333333333333334),
        76.83447166202494,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.7333333333333334),
        77.57349105215687,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.7333333333333334),
        78.31120989928475,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.7333333333333334),
        79.04770394922413,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.7333333333333334),
        79.78304909303499,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.7333333333333334),
        80.51732135557174,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.7333333333333334),
        81.25059688448253,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.7333333333333334),
        81.98295193963135,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.7333333333333334),
        82.71446288291523,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.7333333333333334),
        83.44520616844903,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.7333333333333334),
        84.17525833308898,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.7333333333333334),
        84.90469598726568,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.7333333333333334),
        85.63359580609738,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.7333333333333334),
        86.36203452075294,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.7333333333333334),
        87.09008891003462,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.7333333333333334),
        87.81783579214941,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.7333333333333334),
        88.54535201663822,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.7333333333333334),
        89.27271445643154,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.7333333333333334),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.8222222222222222),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.8222222222222222),
        -88.7567683753512,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.8222222222222222),
        -87.51360646028301,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.8222222222222222),
        -86.27058395444095,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.8222222222222222),
        -85.02777053757745,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.8222222222222222),
        -83.78523585954818,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.8222222222222222),
        -82.54304953024,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.8222222222222222),
        -81.30128110940788,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.8222222222222222),
        -80.06000009639834,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.8222222222222222),
        -78.81927591973727,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.8222222222222222),
        -77.5791779265601,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.8222222222222222),
        -76.33977537186296,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.8222222222222222),
        -75.101137407553,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.8222222222222222),
        -73.86333307127772,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.8222222222222222),
        -72.62643127501222,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.8222222222222222),
        -71.390500793385,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.8222222222222222),
        -70.1556102517225,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.8222222222222222),
        -68.92182811379392,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.8222222222222222),
        -67.68922266923805,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.8222222222222222),
        -66.45786202065453,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.8222222222222222),
        -65.22781407034343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.8222222222222222),
        -63.99914650667634,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.8222222222222222),
        -62.77192679008493,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.8222222222222222),
        -61.54622213865204,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.8222222222222222),
        -60.322099513292855,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.8222222222222222),
        -59.09962560251332,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.8222222222222222),
        -57.878866806735296,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.8222222222222222),
        -56.65988922217828,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.8222222222222222),
        -55.44275862428879,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.8222222222222222),
        -54.227540450710094,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.8222222222222222),
        -53.014299783786015,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.8222222222222222),
        -51.80310133259372,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.8222222222222222),
        -50.59400941450208,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.8222222222222222),
        -49.387087936253664,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.8222222222222222),
        -48.182400374569504,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.8222222222222222),
        -46.98000975627773,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.8222222222222222),
        -45.779978637968696,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.8222222222222222),
        -44.58236908518048,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.8222222222222222),
        -43.387242651120935,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.8222222222222222),
        -42.194660354933504,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.8222222222222222),
        -41.004682659516305,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.8222222222222222),
        -39.81736944890537,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.8222222222222222),
        -38.632780005235006,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.8222222222222222),
        -37.4509729852897,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.8222222222222222),
        -36.27200639666442,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.8222222222222222),
        -35.095937573551204,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.8222222222222222),
        -33.92282315217254,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.8222222222222222),
        -32.75271904588343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.8222222222222222),
        -31.585680419965975,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.8222222222222222),
        -30.421761666142224,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.8222222222222222),
        -29.261016376832828,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.8222222222222222),
        -28.103497319190616,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.8222222222222222),
        -26.94925640894032,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.8222222222222222),
        -25.79834468405699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.8222222222222222),
        -24.650812278317677,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.8222222222222222),
        -23.50670839476214,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.8222222222222222),
        -22.36608127910019,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.8222222222222222),
        -21.228978193104645,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.8222222222222222),
        -20.095445388030093,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.8222222222222222),
        -18.965528078099165,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.8222222222222222),
        -17.839270414098927,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.8222222222222222),
        -16.716715457131436,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.8222222222222222),
        -15.59790515256294,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.8222222222222222),
        -14.482880304217657,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.8222222222222222),
        -13.371680548862116,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.8222222222222222),
        -12.264344331027123,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.8222222222222222),
        -11.16090887821457,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.8222222222222222),
        -10.061410176536569,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.8222222222222222),
        -8.965882946834691,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.8222222222222222),
        -7.874360621326796,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.8222222222222222),
        -6.786875320828909,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.8222222222222222),
        -5.703457832599198,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.8222222222222222),
        -4.624137588850451,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.8222222222222222),
        -3.5489426459769735,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.8222222222222222),
        -2.4778996645405895,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.8222222222222222),
        -1.411033890059804,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.8222222222222222),
        -0.3483691346445744,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.8222222222222222),
        0.7100722404820092,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.8222222222222222),
        1.7642693415354431,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.8222222222222222),
        2.81420275775701,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.8222222222222222),
        3.859854576000347,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.8222222222222222),
        4.901208394283289,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.8222222222222222),
        5.938249334273088,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.8222222222222222),
        6.97096405267535,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.8222222222222222),
        7.999340751499527,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.8222222222222222),
        9.023369187176232,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.8222222222222222),
        10.0430406785044,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.8222222222222222),
        11.058348113408961,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.8222222222222222),
        12.069285954492562,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.8222222222222222),
        13.075850243367798,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.8222222222222222),
        14.078038603759248,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.8222222222222222),
        15.0758502433678,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.8222222222222222),
        16.069285954492567,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.8222222222222222),
        17.05834811340896,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.8222222222222222),
        18.043040678504397,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.8222222222222222),
        19.023369187176236,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.8222222222222222),
        19.99934075149953,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.8222222222222222),
        20.970964052675352,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.8222222222222222),
        21.938249334273088,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.8222222222222222),
        22.901208394283284,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.8222222222222222),
        23.859854576000345,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.8222222222222222),
        24.814202757757005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.8222222222222222),
        25.76426934153544,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.8222222222222222),
        26.71007224048201,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.8222222222222222),
        27.651630865355425,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.8222222222222222),
        28.588966109940195,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.8222222222222222),
        29.52210033545941,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.8222222222222222),
        30.45105735402303,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.8222222222222222),
        31.37586241114954,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.8222222222222222),
        32.29654216740081,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.8222222222222222),
        33.21312467917109,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.8222222222222222),
        34.1256393786732,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.8222222222222222),
        35.03411705316531,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.8222222222222222),
        35.93858982346343,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.8222222222222222),
        36.83909112178544,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.8222222222222222),
        37.73565566897288,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.8222222222222222),
        38.628319451137884,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.8222222222222222),
        39.51711969578235,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.8222222222222222),
        40.40209484743706,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.8222222222222222),
        41.28328454286857,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.8222222222222222),
        42.16072958590107,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.8222222222222222),
        43.03447192190083,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.8222222222222222),
        43.90455461196991,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.8222222222222222),
        44.77102180689536,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.8222222222222222),
        45.63391872089982,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.8222222222222222),
        46.493291605237864,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.8222222222222222),
        47.34918772168233,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.8222222222222222),
        48.20165531594301,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.8222222222222222),
        49.05074359105968,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.8222222222222222),
        49.89650268080938,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.8222222222222222),
        50.73898362316717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.8222222222222222),
        51.57823833385778,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.8222222222222222),
        52.41431958003403,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.8222222222222222),
        53.24728095411657,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.8222222222222222),
        54.07717684782745,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.8222222222222222),
        54.904062426448796,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.8222222222222222),
        55.727993603335584,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.8222222222222222),
        56.54902701471031,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.8222222222222222),
        57.367219994765,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.8222222222222222),
        58.18263055109463,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.8222222222222222),
        58.99531734048371,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.8222222222222222),
        59.8053396450665,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.8222222222222222),
        60.61275734887907,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.8222222222222222),
        61.41763091481951,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.8222222222222222),
        62.2200213620313,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.8222222222222222),
        63.01999024372226,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.8222222222222222),
        63.8175996254305,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.8222222222222222),
        64.61291206374635,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.8222222222222222),
        65.40599058549795,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.8222222222222222),
        66.1968986674063,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.8222222222222222),
        66.98570021621398,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.8222222222222222),
        67.7724595492899,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.8222222222222222),
        68.55724137571121,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.8222222222222222),
        69.34011077782172,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.8222222222222222),
        70.12113319326471,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.8222222222222222),
        70.9003743974867,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.8222222222222222),
        71.67790048670714,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.8222222222222222),
        72.45377786134797,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.8222222222222222),
        73.22807320991508,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.8222222222222222),
        74.00085349332365,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.8222222222222222),
        74.77218592965657,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.8222222222222222),
        75.54213797934547,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.8222222222222222),
        76.31077733076197,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.8222222222222222),
        77.07817188620608,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.8222222222222222),
        77.84438974827751,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.8222222222222222),
        78.60949920661501,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.8222222222222222),
        79.37356872498779,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.8222222222222222),
        80.13666692872229,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.8222222222222222),
        80.89886259244699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.8222222222222222),
        81.66022462813703,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.8222222222222222),
        82.4208220734399,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.8222222222222222),
        83.18072408026273,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.8222222222222222),
        83.93999990360166,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.8222222222222222),
        84.69871889059212,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.8222222222222222),
        85.45695046976,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.8222222222222222),
        86.21476414045183,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.8222222222222222),
        86.97222946242256,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.8222222222222222),
        87.72941604555903,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.8222222222222222),
        88.48639353971699,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.8222222222222222),
        89.2432316246488,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.8222222222222222),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 0.9111111111111112),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 0.9111111111111112),
        -88.78049841231991,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 0.9111111111111112),
        -87.56106046709472,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 0.9111111111111112),
        -86.34174979579412,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 0.9111111111111112),
        -85.12263000790071,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 0.9111111111111112),
        -83.90376467987511,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 0.9111111111111112),
        -82.68521734407126,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 0.9111111111111112),
        -81.46705147758571,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 0.9111111111111112),
        -80.24933049102474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 0.9111111111111112),
        -79.03211771717311,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 0.9111111111111112),
        -77.81547639954877,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 0.9111111111111112),
        -76.59946968082807,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 0.9111111111111112),
        -75.38416059112595,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 0.9111111111111112),
        -74.16961203611676,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 0.9111111111111112),
        -72.95588678498062,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 0.9111111111111112),
        -71.7430474581619,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 0.9111111111111112),
        -70.5311565149257,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 0.9111111111111112),
        -69.3202762406997,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 0.9111111111111112),
        -68.11046873418853,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 0.9111111111111112),
        -66.9017958942489,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 0.9111111111111112),
        -65.69431940651434,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 0.9111111111111112),
        -64.48810072975841,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 0.9111111111111112),
        -63.28320108198717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 0.9111111111111112),
        -62.07968142625101,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 0.9111111111111112),
        -60.87760245616814,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 0.9111111111111112),
        -59.677024581151706,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 0.9111111111111112),
        -58.47800791133395,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 0.9111111111111112),
        -57.28061224218179,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 0.9111111111111112),
        -56.08489703879889,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 0.9111111111111112),
        -54.89092141991028,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 0.9111111111111112),
        -53.69874414152681,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 0.9111111111111112),
        -52.50842358028761,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 0.9111111111111112),
        -51.32001771647975,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 0.9111111111111112),
        -50.133584116735754,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 0.9111111111111112),
        -48.94917991641025,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 0.9111111111111112),
        -47.76686180163856,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 0.9111111111111112),
        -46.58668599108138,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 0.9111111111111112),
        -45.4087082173603,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 0.9111111111111112),
        -44.232983708191014,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 0.9111111111111112),
        -43.05956716722143,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 0.9111111111111112),
        -41.88851275458412,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 0.9111111111111112),
        -40.71987406717289,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 0.9111111111111112),
        -39.55370411865531,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 0.9111111111111112),
        -38.39005531923396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 0.9111111111111112),
        -37.22897945517062,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 0.9111111111111112),
        -36.07052766808872,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 0.9111111111111112),
        -34.9147504340712,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 0.9111111111111112),
        -33.7616975425713,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 0.9111111111111112),
        -32.611418075156216,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 0.9111111111111112),
        -31.46396038410396,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 0.9111111111111112),
        -30.319372070875175,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 0.9111111111111112),
        -29.177699964483317,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 0.9111111111111112),
        -28.03899009978714,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 0.9111111111111112),
        -26.903287695730963,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 0.9111111111111112),
        -25.770637133559287,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 0.9111111111111112),
        -24.641081935033156,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 0.9111111111111112),
        -23.514664740676803,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 0.9111111111111112),
        -22.3914272880839,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 0.9111111111111112),
        -21.27141039031404,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 0.9111111111111112),
        -20.15465391441001,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 0.9111111111111112),
        -19.04119676006811,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 0.9111111111111112),
        -17.931076838493716,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 0.9111111111111112),
        -16.824331051475045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 0.9111111111111112),
        -15.720995270708773,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 0.9111111111111112),
        -14.62110431741102,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 0.9111111111111112),
        -13.524691942248074,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 0.9111111111111112),
        -12.431790805620928,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 0.9111111111111112),
        -11.34243245833797,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 0.9111111111111112),
        -10.25664732271026,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 0.9111111111111112),
        -9.174464674103303,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 0.9111111111111112),
        -8.095912622979437,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 0.9111111111111112),
        -7.021018097464155,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 0.9111111111111112),
        -5.949806826469525,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 0.9111111111111112),
        -4.8823033234071245,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 0.9111111111111112),
        -3.8185308705221277,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 0.9111111111111112),
        -2.7585115038796193,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 0.9111111111111112),
        -1.7022659990329552,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 0.9111111111111112),
        -0.6498138574032203,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 0.9111111111111112),
        0.39882670660245895,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 0.9111111111111112),
        1.4436387777072224,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 0.9111111111111112),
        2.484606751090163,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 0.9111111111111112),
        3.5217163428977014,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 0.9111111111111112),
        4.554954599918377,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 0.9111111111111112),
        5.584309908400535,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 0.9111111111111112),
        6.609772001993968,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 0.9111111111111112),
        7.631331968798446,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 0.9111111111111112),
        8.648982257503873,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 0.9111111111111112),
        9.662716682608727,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 0.9111111111111112),
        10.672530428705347,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 0.9111111111111112),
        11.678420053822792,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 0.9111111111111112),
        12.68038349181982,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 0.9111111111111112),
        13.678420053822794,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 0.9111111111111112),
        14.672530428705347,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 0.9111111111111112),
        15.662716682608725,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 0.9111111111111112),
        16.64898225750388,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 0.9111111111111112),
        17.631331968798445,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 0.9111111111111112),
        18.609772001993967,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 0.9111111111111112),
        19.584309908400535,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 0.9111111111111112),
        20.554954599918375,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 0.9111111111111112),
        21.5217163428977,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 0.9111111111111112),
        22.484606751090165,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 0.9111111111111112),
        23.44363877770722,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 0.9111111111111112),
        24.398826706602456,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 0.9111111111111112),
        25.35018614259678,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 0.9111111111111112),
        26.297734000967047,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 0.9111111111111112),
        27.24148849612038,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 0.9111111111111112),
        28.181469129477875,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 0.9111111111111112),
        29.117696676592878,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 0.9111111111111112),
        30.050193173530474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 0.9111111111111112),
        30.978981902535846,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 0.9111111111111112),
        31.904087377020563,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 0.9111111111111112),
        32.825535325896695,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 0.9111111111111112),
        33.74335267728975,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 0.9111111111111112),
        34.657567541662026,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 0.9111111111111112),
        35.56820919437909,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 0.9111111111111112),
        36.475308057751924,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 0.9111111111111112),
        37.37889568258898,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 0.9111111111111112),
        38.27900472929124,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 0.9111111111111112),
        39.17566894852496,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 0.9111111111111112),
        40.06892316150629,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 0.9111111111111112),
        40.95880323993188,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 0.9111111111111112),
        41.845346085589995,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 0.9111111111111112),
        42.72858960968596,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 0.9111111111111112),
        43.6085727119161,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 0.9111111111111112),
        44.485335259323215,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 0.9111111111111112),
        45.35891806496684,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 0.9111111111111112),
        46.22936286644072,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 0.9111111111111112),
        47.09671230426904,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 0.9111111111111112),
        47.961009900212865,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 0.9111111111111112),
        48.822300035516676,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 0.9111111111111112),
        49.680627929124825,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 0.9111111111111112),
        50.536039615896044,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 0.9111111111111112),
        51.38858192484378,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 0.9111111111111112),
        52.23830245742871,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 0.9111111111111112),
        53.0852495659288,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 0.9111111111111112),
        53.92947233191127,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 0.9111111111111112),
        54.7710205448294,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 0.9111111111111112),
        55.60994468076605,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 0.9111111111111112),
        56.446295881344696,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 0.9111111111111112),
        57.280125932827104,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 0.9111111111111112),
        58.11148724541589,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 0.9111111111111112),
        58.940432832778576,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 0.9111111111111112),
        59.767016291808986,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 0.9111111111111112),
        60.591291782639686,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 0.9111111111111112),
        61.41331400891862,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 0.9111111111111112),
        62.23313819836144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 0.9111111111111112),
        63.050820083589755,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 0.9111111111111112),
        63.86641588326425,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 0.9111111111111112),
        64.67998228352027,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 0.9111111111111112),
        65.4915764197124,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 0.9111111111111112),
        66.30125585847318,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 0.9111111111111112),
        67.10907858008972,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 0.9111111111111112),
        67.91510296120111,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 0.9111111111111112),
        68.71938775781821,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 0.9111111111111112),
        69.52199208866605,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 0.9111111111111112),
        70.32297541884829,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 0.9111111111111112),
        71.12239754383187,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 0.9111111111111112),
        71.920318573749,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 0.9111111111111112),
        72.71679891801284,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 0.9111111111111112),
        73.51189927024156,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 0.9111111111111112),
        74.30568059348565,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 0.9111111111111112),
        75.09820410575108,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 0.9111111111111112),
        75.88953126581147,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 0.9111111111111112),
        76.6797237593003,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 0.9111111111111112),
        77.4688434850743,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 0.9111111111111112),
        78.25695254183812,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 0.9111111111111112),
        79.0441132150194,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 0.9111111111111112),
        79.83038796388325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 0.9111111111111112),
        80.61583940887404,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 0.9111111111111112),
        81.40053031917193,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 0.9111111111111112),
        82.18452360045121,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 0.9111111111111112),
        82.96788228282689,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 0.9111111111111112),
        83.75066950897525,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 0.9111111111111112),
        84.53294852241429,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 0.9111111111111112),
        85.31478265592874,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 0.9111111111111112),
        86.0962353201249,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 0.9111111111111112),
        86.87736999209929,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 0.9111111111111112),
        87.65825020420588,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 0.9111111111111112),
        88.43893953290527,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 0.9111111111111112),
        89.21950158768009,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 90.0, 0.9111111111111112),
        90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -90.0, 1.0),
        -90.0,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -89.0, 1.0),
        -88.80000974766241,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -88.0, 1.0),
        -87.60007797901855,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -87.0, 1.0),
        -86.40026316635586,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -86.0, 1.0),
        -85.200623759137,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -85.0, 1.0),
        -84.00121817255722,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -84.0, 1.0),
        -82.80210477606528,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -83.0, 1.0),
        -81.60334188183627,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -82.0, 1.0),
        -80.40498773318431,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -81.0, 1.0),
        -79.20710049290352,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -80.0, 1.0),
        -78.00973823152586,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -79.0, 1.0),
        -76.81295891548454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -78.0, 1.0),
        -75.6168203951722,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -77.0, 1.0),
        -74.42138039288292,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -76.0, 1.0),
        -73.22669649062797,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -75.0, 1.0),
        -72.03282611781526,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -74.0, 1.0),
        -70.83982653878276,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -73.0, 1.0),
        -69.64775484017696,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -72.0, 1.0),
        -68.45666791816745,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -71.0, 1.0),
        -67.26662246548963,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -70.0, 1.0),
        -66.07767495830758,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -69.0, 1.0),
        -64.8898816428902,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -68.0, 1.0),
        -63.70329852209391,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -67.0, 1.0),
        -62.51798134164596,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -66.0, 1.0),
        -61.33398557622325,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -65.0, 1.0),
        -60.15136641532166,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -64.0, 1.0),
        -58.97017874891222,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -63.0, 1.0),
        -57.79047715288095,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -62.0, 1.0),
        -56.61231587424962,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -61.0, 1.0),
        -55.4357488161761,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -60.0, 1.0),
        -54.260829522733204,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -59.0, 1.0),
        -53.08761116346616,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -58.0, 1.0),
        -51.91614651772915,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -57.0, 1.0),
        -50.74648795880326,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -56.0, 1.0),
        -49.578687437797704,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -55.0, 1.0),
        -48.41279646733823,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -54.0, 1.0),
        -47.24886610504707,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -53.0, 1.0),
        -46.086946936819544,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -52.0, 1.0),
        -44.927089059903984,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -51.0, 1.0),
        -43.76934206579182,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -50.0, 1.0),
        -42.61375502292634,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -49.0, 1.0),
        -41.4603764592393,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -48.0, 1.0),
        -40.309254344525264,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -47.0, 1.0),
        -39.16043607266512,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -46.0, 1.0),
        -38.013968443710795,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -45.0, 1.0),
        -36.86989764584401,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -44.0, 1.0),
        -35.728269237223515,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -43.0, 1.0),
        -34.58912812773519,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -42.0, 1.0),
        -33.45251856066144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -41.0, 1.0),
        -32.318484094286326,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -40.0, 1.0),
        -31.187067583454116,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -39.0, 1.0),
        -30.058311161099866,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -38.0, 1.0),
        -28.932256219771336,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -37.0, 1.0),
        -27.808943393162203,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -36.0, 1.0),
        -26.688412537677717,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -35.0, 1.0),
        -25.570702714054157,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -34.0, 1.0),
        -24.45585216905443,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -33.0, 1.0),
        -23.34389831726257,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -32.0, 1.0),
        -22.23487772300082,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -31.0, 1.0),
        -21.12882608239301,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -30.0, 1.0),
        -20.025778205598655,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -29.0, 1.0),
        -18.925767999242748,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -28.0, 1.0),
        -17.828828449066208,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -27.0, 1.0),
        -16.734991602822532,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -26.0, 1.0),
        -15.644288553446046,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -25.0, 1.0),
        -14.556749422517688,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -24.0, 1.0),
        -13.472403344054063,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -23.0, 1.0),
        -12.3912784486454,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -22.0, 1.0),
        -11.313401847968223,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -21.0, 1.0),
        -10.23879961969815,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -20.0, 1.0),
        -9.16749679284807,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -19.0, 1.0),
        -8.099517333556602,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -18.0, 1.0),
        -7.034884131351297,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -17.0, 1.0),
        -5.973618985910688,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -16.0, 1.0),
        -4.915742594348384,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -15.0, 1.0),
        -3.861274539042306,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -14.0, 1.0),
        -2.8102332760308264,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -13.0, 1.0),
        -1.7626361239972315,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -12.0, 1.0),
        -0.7184992538628489,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -11.0, 1.0),
        0.32216232099175274,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -10.0, 1.0),
        1.359334753859042,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -9.0, 1.0),
        2.3930053731729664,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -8.0, 1.0),
        3.4231626902973225,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -7.0, 1.0),
        4.449796406619122,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -6.0, 1.0),
        5.472897419933128,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -5.0, 1.0),
        6.4924578301052005,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -4.0, 1.0),
        7.508470944003321,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -3.0, 1.0),
        8.520931279686618,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -2.0, 1.0),
        9.529834569844144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, -1.0, 1.0),
        10.535177764476568,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 0.0, 1.0),
        11.53695903281549,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 1.0, 1.0),
        12.53517776447657,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 2.0, 1.0),
        13.529834569844143,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 3.0, 1.0),
        14.520931279686616,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 4.0, 1.0),
        15.508470944003323,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 5.0, 1.0),
        16.4924578301052,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 6.0, 1.0),
        17.47289741993313,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 7.0, 1.0),
        18.449796406619125,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 8.0, 1.0),
        19.42316269029733,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 9.0, 1.0),
        20.393005373172965,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 10.0, 1.0),
        21.359334753859045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 11.0, 1.0),
        22.32216232099175,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 12.0, 1.0),
        23.281500746137155,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 13.0, 1.0),
        24.23736387600277,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 14.0, 1.0),
        25.18976672396918,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 15.0, 1.0),
        26.138725460957694,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 16.0, 1.0),
        27.084257405651616,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 17.0, 1.0),
        28.026381014089317,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 18.0, 1.0),
        28.965115868648695,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 19.0, 1.0),
        29.9004826664434,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 20.0, 1.0),
        30.832503207151927,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 21.0, 1.0),
        31.761200380301844,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 22.0, 1.0),
        32.68659815203178,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 23.0, 1.0),
        33.6087215513546,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 24.0, 1.0),
        34.52759665594594,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 25.0, 1.0),
        35.44325057748231,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 26.0, 1.0),
        36.355711446553954,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 27.0, 1.0),
        37.26500839717746,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 28.0, 1.0),
        38.17117155093379,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 29.0, 1.0),
        39.07423200075727,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 30.0, 1.0),
        39.97422179440135,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 31.0, 1.0),
        40.871173917606995,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 32.0, 1.0),
        41.76512227699917,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 33.0, 1.0),
        42.656101682737436,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 34.0, 1.0),
        43.544147830945576,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 35.0, 1.0),
        44.429297285945836,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 36.0, 1.0),
        45.31158746232229,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 37.0, 1.0),
        46.191056606837805,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 38.0, 1.0),
        47.06774378022867,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 39.0, 1.0),
        47.94168883890011,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 40.0, 1.0),
        48.812932416545884,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 41.0, 1.0),
        49.68151590571367,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 42.0, 1.0),
        50.54748143933857,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 43.0, 1.0),
        51.41087187226481,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 44.0, 1.0),
        52.271730762776485,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 45.0, 1.0),
        53.13010235415599,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 46.0, 1.0),
        53.98603155628922,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 47.0, 1.0),
        54.839563927334886,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 48.0, 1.0),
        55.69074565547475,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 49.0, 1.0),
        56.5396235407607,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 50.0, 1.0),
        57.386244977073666,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 51.0, 1.0),
        58.2306579342082,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 52.0, 1.0),
        59.07291094009602,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 53.0, 1.0),
        59.91305306318045,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 54.0, 1.0),
        60.75113389495292,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 55.0, 1.0),
        61.58720353266176,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 56.0, 1.0),
        62.42131256220231,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 57.0, 1.0),
        63.25351204119675,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 58.0, 1.0),
        64.08385348227087,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 59.0, 1.0),
        64.91238883653385,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 60.0, 1.0),
        65.73917047726678,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 61.0, 1.0),
        66.56425118382391,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 62.0, 1.0),
        67.38768412575038,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 63.0, 1.0),
        68.20952284711905,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 64.0, 1.0),
        69.02982125108778,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 65.0, 1.0),
        69.84863358467835,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 66.0, 1.0),
        70.66601442377676,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 67.0, 1.0),
        71.48201865835405,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 68.0, 1.0),
        72.2967014779061,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 69.0, 1.0),
        73.11011835710978,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 70.0, 1.0),
        73.9223250416924,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 71.0, 1.0),
        74.73337753451035,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 72.0, 1.0),
        75.54333208183255,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 73.0, 1.0),
        76.35224515982304,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 74.0, 1.0),
        77.16017346121724,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 75.0, 1.0),
        77.96717388218474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 76.0, 1.0),
        78.77330350937203,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 77.0, 1.0),
        79.57861960711709,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 78.0, 1.0),
        80.38317960482779,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 79.0, 1.0),
        81.18704108451544,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 80.0, 1.0),
        81.99026176847414,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 81.0, 1.0),
        82.79289950709646,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 82.0, 1.0),
        83.59501226681569,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 83.0, 1.0),
        84.39665811816373,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 84.0, 1.0),
        85.19789522393474,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 85.0, 1.0),
        85.9987818274428,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 86.0, 1.0),
        86.79937624086301,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 87.0, 1.0),
        87.59973683364414,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 88.0, 1.0),
        88.39992202098144,
        epsilon = 3.0
    );
    assert_relative_eq!(
        lookup_gravity_heading(0.2, 89.0, 1.0),
        89.19999025233759,
        epsilon = 3.0
    );
    assert_relative_eq!(lookup_gravity_heading(0.2, 90.0, 1.0), 90.0, epsilon = 3.0);
}
