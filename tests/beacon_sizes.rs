use south_common::*;

#[test]
pub fn get_beacon_sizes() {
    println!("lst beacon: {}", LSTBeacon::BYTE_SIZE);
    println!("eps beacon: {}", EPSBeacon::BYTE_SIZE);
    println!("upper sens beacon: {}", SensorboardBeacon::BYTE_SIZE);
    println!("lower sens beacon: {}", LowerSensorBeacon::BYTE_SIZE);
    panic!()
}
