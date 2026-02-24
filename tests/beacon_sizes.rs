use south_common::beacons::*;

#[test]
pub fn get_beacon_sizes() {
    println!("lst beacon: {}", LSTBeacon::BYTE_SIZE);
    println!("eps beacon: {}", EPSBeacon::BYTE_SIZE);
    println!("low rate upper beacon: {}", LowRateUpperSensorBeacon::BYTE_SIZE);
    println!("high rate upper beacon: {}", HighRateUpperSensorBeacon::BYTE_SIZE);
    println!("lower sens beacon: {}", LowerSensorBeacon::BYTE_SIZE);

    assert_eq!(LSTBeacon::BYTE_SIZE, 34);
    assert_eq!(EPSBeacon::BYTE_SIZE, 25);
    assert_eq!(LowRateUpperSensorBeacon::BYTE_SIZE, 45);
    assert_eq!(HighRateUpperSensorBeacon::BYTE_SIZE, 32);
    assert_eq!(LowerSensorBeacon::BYTE_SIZE, 20);
}
