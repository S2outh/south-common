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
    HighRateUpperSensorBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 2,
    telemetry(
        imu1::AccelLowRange,
        imu1::AccelFullRange,
        imu1::Gyro,
        imu1::Temp
    )
);

beacon!(
    LowRateUpperSensorBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 3,
    telemetry(
        gps::Pos,
        gps::Status,
        baro::Pressure,
        baro::Temp,
        InternalTemperature
    )
);

beacon!(
    LowerSensorBeacon,
    crate::definitions::telemetry::lower_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 4,
    telemetry(Pressure1, Pressure2, Temp, AdcTemp)
);
