use tmtc_system::*;

#[telemetry_definition(id = 0)]
mod telecommands {
    #[tmv(crate::types::Telecommand)]
    struct Telecommand;
}

#[telemetry_definition(id = 1)]
pub mod telemetry {

    #[tmv(u64)]
    struct Timestamp;

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

    mod upper_sensor {
        mod imu1 {
            #[tmv([i16; 3])]
            struct AccelLowRange;

            #[tmv([i16; 3])]
            struct AccelFullRange;

            #[tmv([i16; 3])]
            struct Gyro;

            #[tmv(i16)]
            struct Temp;
        }

        mod imu2 {
            #[tmv([i16; 3])]
            struct AccelLowRange;

            #[tmv([i16; 3])]
            struct AccelFullRange;

            #[tmv([i16; 3])]
            struct Gyro;

            #[tmv(i16)]
            struct Temp;
        }

        mod gps {
            #[tmv([f32; 3])]
            struct Pos;

            #[tmv(u8)]
            struct Status;
        }

        mod baro {
            #[tmv(i16)]
            struct Pressure;

            #[tmv(i16)]
            struct Temp;
        }

        #[tmv(i16)]
        struct InternalTemperature;
    }

    mod lower_sensor {
        #[tmv(i16)]
        struct Pressure1;

        #[tmv(i16)]
        struct Pressure2;

        #[tmv(i16)]
        struct Temp;

        #[tmv(i16)]
        struct AdcTemp;
    }
}
