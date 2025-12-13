use tmtc_system::*;

#[telemetry_definition(id = 0)]
mod telecommands {
    #[tmv(crate::command_types::Telecommand)]
    struct Telecommand;
}

#[telemetry_definition(id = 1)]
pub mod telemetry {

    #[tmv(i64)]
    struct Timestamp;
    
    #[tmm(id = 100)]
    mod lst {
        #[tmv(u32)]
        struct Uptime;

        #[tmv(i8)]
        struct Rssi;

        #[tmv(u8)]
        struct Lqi;

        #[tmv(u32)]
        struct PacketsSend;
        
        #[tmv(u32)]
        struct PacketsGood;

        #[tmv(u32)]
        struct PacketsBadChecksum;

        #[tmv(u32)]
        struct PacketsBadOther;
    }

    #[tmm(id = 200)]
    mod eps {
        #[tmv(u8)]
        struct EnableBitmap;
        
        #[tmv(i16)]
        struct AuxPowerVoltage;

        #[tmv(i16)]
        struct InternalTemperature;

        #[tmv(i16)]
        struct Bat1Voltage;

        #[tmv(i16)]
        struct Bat1Temperature;
        
        #[tmv(i16)]
        struct Bat2Voltage;

        #[tmv(i16)]
        struct Bat2Temperature;
    }
}

