use tmtc_system::*;

beacon!(
    LSTBeacon,
    crate::definitions::telemetry::lst,
    crate::definitions::telemetry::Timestamp,
    id = 0,
    telemetry(
        Uptime,
        Rssi,
        Lqi,
        PacketsSend,
        PacketsGood,
        PacketsBadChecksum,
        PacketsBadOther
    )
);

beacon!(
    EPSBeacon,
    crate::definitions::telemetry::eps,
    crate::definitions::telemetry::Timestamp,
    id = 1,
    telemetry(
        EnableBitmap,
        AuxPowerVoltage,
        InternalTemperature,
        Bat1Voltage,
        Bat1Temperature,
        Bat2Voltage,
        Bat2Temperature
    )
);

beacon!(
    SensorboardBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 2,
    telemetry(
        imu1::AccelLowRange,
        imu1::AccelFullRange,
        imu1::Gyro,
        imu1::Temp,
        imu2::AccelLowRange,
        imu2::AccelFullRange,
        imu2::Gyro,
        imu2::Temp,
        baro::Pressure,
        baro::Temp,
        InternalTemperature
    )
);
