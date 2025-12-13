use tmtc_system::*;

beacon!(LowRateTelemetry, crate::definitions::telemetry,
    header(0x4E, 0x4A, 1, 0, 37, 0, 0),
    telemetry(
        Timestamp, 
        lst::Uptime,
        lst::Rssi,
        lst::Lqi,
        lst::PacketsSend,
        lst::PacketsGood,
        lst::PacketsBadChecksum,
        lst::PacketsBadOther
    )
);

beacon!(MidRateTelemetry, crate::definitions::telemetry,
    header(0x4E, 0x4A, 2, 0, 28, 0, 0),
    telemetry(
        Timestamp, 
        eps::EnableBitmap,
        eps::AuxPowerVoltage,
        eps::InternalTemperature,
        eps::Bat1Voltage,
        eps::Bat1Temperature,
        eps::Bat2Voltage,
        eps::Bat2Temperature
    )
);
