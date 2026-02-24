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
        struct PacketsSent;

        #[tmv(u32)]
        struct PacketsGood;

        #[tmv(u32)]
        struct PacketsRejectedChecksum;

        #[tmv(u32)]
        struct PacketsRejectedOther;
    }

    mod eps {
        #[tmv(u8, enabled = |v: &u8| crate::types::EPSEnabled::from_bits_truncate(*v))]
        struct EnableBitmap;

        #[tmv(i16, v = |v| crate::parsers::fixed_dec(100., v))]
        struct AuxPowerVoltage;

        #[tmv(i16, c = |v| crate::parsers::fixed_dec(10., v))]
        struct InternalTemperature;

        #[tmv(i16, v = |v| crate::parsers::fixed_dec(100., v))]
        struct Bat1Voltage;

        #[tmv(i16, c = |v| crate::parsers::fixed_dec(10., v))]
        struct Bat1Temperature;

        #[tmv(i16, v = |v| crate::parsers::fixed_dec(100., v))]
        struct Bat2Voltage;

        #[tmv(i16, c = |v| crate::parsers::fixed_dec(10., v))]
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
            #[tmv([i32; 3])]
            struct ECEF;

            #[tmv([i32; 3])]
            struct Vel;

            #[tmv(u8, d = |v| crate::parsers::split_byte([2, 2, 4], v))]
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
        #[tmv(i16, pa = crate::parsers::pres_raw_to_pascal)]
        struct Pressure1;

        #[tmv(i16, pa = crate::parsers::pres_raw_to_pascal)]
        struct Pressure2;

        #[tmv(i16, c = crate::parsers::temp_raw_to_celcius)]
        struct Temp;

        #[tmv(i16)]
        struct AdcTemp;
    }
}
