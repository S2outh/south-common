use lsm6dsv32::config::{FifoDisabled, ImuConfig, Int1Disabled, Int2Disabled};

pub fn get_imu_config() -> ImuConfig<FifoDisabled, Int1Disabled, Int2Disabled> {
    let mut config = ImuConfig::default();

    // high accuracy mode
    config.use_high_accuracy_mode(lsm6dsv32::config::HighAccuracyODR::Standard);

    // config
    config.accel.dual_channel = true;
    config.accel.full_scale = lsm6dsv32::config::AccelFS::G8;

    // active low interrupt
    config.general.interrupt_lvl = true;

    config
        .accel
        .set_odr(lsm6dsv32::config::AccelODR::KHz1_92)
        .map_err(|_| "could not set accel odr")
        .unwrap();
    config
        .gyro
        .set_odr(lsm6dsv32::config::GyroODR::KHz1_92)
        .map_err(|_| "could not set accel odr")
        .unwrap();

    config.int1.data_ready_accel = true;
    config.int1.data_ready_gyro = true;

    config
}
