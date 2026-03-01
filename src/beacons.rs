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
        Imu1
    )
);

beacon!(
    LowRateUpperSensorBeacon,
    crate::definitions::telemetry::upper_sensor,
    crate::definitions::telemetry::Timestamp,
    id = 3,
    telemetry(
        gps::ECEF,
        gps::Vel,
        gps::Status,
        Baro,
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
