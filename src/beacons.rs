use chell::*;

// # Primary Lst
beacon!(
    LSTBeacon,
    crate::definitions::telemetry::lst,
    crate::definitions::telemetry::Timestamp,
    id = 0,
    telemetry(
        Uptime,
        Rssi,
        Lqi,
        PacketsSent,
        PacketsGood,
        PacketsRejectedChecksum,
        PacketsRejectedOther
    )
);

beacon!(
    EPSBeacon,
    crate::definitions::telemetry::eps,
    crate::definitions::telemetry::Timestamp,
    id = 1,
    telemetry(
        SourceEnabled,
        SinkEnabled,
        AuxPowerVoltage,
        AuxPowerCurrent,
        Bat1Voltage,
        Bat1Temperature,
        Bat1Current,
        Bat2Voltage,
        Bat2Temperature,
        Bat2Current
    )
);

beacon!(
    HighRateUpperSensorBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 2,
    telemetry(
        imu1::Accel,
        imu1::Gyro
    )
);

beacon!(
    LowRateUpperSensorBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 3,
    telemetry(
        gps::Pos,
        gps::Vel,
        gps::Status,
        Baro,
        Magneto,
        InternalTemperature
    )
);

beacon!(
    LowerSensorBeacon,
    crate::definitions::telemetry::lower_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 4,
    telemetry(Adc)
);

beacon!(
    PyroBeacon,
    crate::definitions::telemetry::pyro,
    crate::definitions::telemetry::Timestamp,
    id = 5,
    telemetry(
        Status,
        Bat1Voltage,
        Bat2Voltage,
        Out1Voltage,
        Out2Voltage
    )
);

// # Secondary Lst
beacon!(
    SecondaryLstBeacon,
    crate::definitions::telemetry,
    crate::definitions::telemetry::Timestamp,
    id = 0,
    telemetry(
        eps::SourceEnabled,
        eps::SinkEnabled,
        upper_sensor::gps::Pos
    )
);
