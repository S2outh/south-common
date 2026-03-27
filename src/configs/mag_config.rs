use rm3100::config::{Config, OperatingMode, DataRate, DrdyMode};

pub fn get_mag_config() -> Config {
    Config {
        mode : OperatingMode::Continous,
        continous_dr: DataRate::Hz18,
        cycle_count: 350,
        use_x: true,
        use_y: true,
        use_z: true,
        drdy: DrdyMode::EndOfMeasurementSequence
    }
}
