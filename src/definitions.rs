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
        #[tmv(u8, flags = |v: &u8| crate::types::eps::SourceEnabled::from_bits_truncate(*v))]
        struct SourceEnabled;

        #[tmv(u8, flags = |v: &u8| crate::types::eps::SinkEnabled::from_bits_truncate(*v))]
        struct SinkEnabled;

        #[tmv(i16, v = |v| crate::parsing::fixed_dec(100., v))]
        struct AuxPowerVoltage;

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[tmv(i16, v = |v| crate::parsing::fixed_dec(100., v))]
        struct Bat1Voltage;

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat1Temperature;

        #[tmv(i16, v = |v| crate::parsing::fixed_dec(100., v))]
        struct Bat2Voltage;

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat2Temperature;
    }

    mod upper_sensor {
        #[tmv(crate::types::upper_sensor::ImuRaw)]
        struct Imu1;

        #[tmv(crate::types::upper_sensor::ImuRaw)]
        struct Imu2;

        mod gps {
            #[tmv([i32; 3], llh = crate::parsing::upper_sensor::ecef_cm_to_llh)]
            struct ECEF;

            #[tmv([i32; 3])]
            struct Vel;

            #[tmv(u8, sv = |v| crate::parsing::mask(2, 6, v), nav = |v| crate::parsing::mask(0, 2, v))]
            struct Status;
        }

        #[tmv(crate::types::upper_sensor::BaroRaw,
            c = crate::parsing::upper_sensor::baro_temp_convert,
            p = crate::parsing::upper_sensor::baro_pressure_convert_pa
        )]
        struct Baro;

        #[tmv(i16)]
        struct InternalTemperature;
    }

    mod lower_sensor {
        #[tmv(crate::types::lower_sensor::LowerSensorAdcValues,
            p_ch1 = crate::parsing::lower_sensor::pascal_ch1,
            p_ch2 = crate::parsing::lower_sensor::pascal_ch1,
            c = crate::parsing::lower_sensor::celcius
        )]
        struct Adc;
    }
}
