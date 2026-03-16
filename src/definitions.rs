use tmtc_system::*;

#[telemetry_definition(id = 0)]
mod internal_msgs {
    #[tmv(crate::types::Telecommand)]
    struct Telecommand;

    #[tmv(u8)]
    struct TimesyncRequest;

    #[tmv(crate::types::Timesync)]
    struct TimesyncAnswer;
}

#[telemetry_definition(id = 10)]
pub mod telemetry {

    /// This is a utc timestamp in microseconds
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

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[tmv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct AuxPowerVoltage;

        #[tmv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct AuxPowerCurrent;

        #[tmv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct Bat1Voltage;

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat1Temperature;

        #[tmv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct Bat1Current;

        #[tmv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct Bat2Voltage;

        #[tmv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat2Temperature;

        #[tmv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct Bat2Current;
    }

    mod upper_sensor {
        mod imu1 {
            #[tmv(crate::types::upper_sensor::AccelRaw,
                mps = crate::parsing::upper_sensor::accel_f32
            )]
            struct Accel;

            #[tmv(crate::types::Vector3i16,
                rps = crate::parsing::upper_sensor::gyro_f32
            )]
            struct Gyro;
        }

        mod imu2 {
            #[tmv(crate::types::upper_sensor::AccelRaw,
                mps = crate::parsing::upper_sensor::accel_f32
            )]
            struct Accel;

            #[tmv(crate::types::Vector3i16,
                rps = crate::parsing::upper_sensor::gyro_f32
            )]
            struct Gyro;
        }

        mod gps {
            #[tmv(crate::types::Vector3i32, llh = crate::parsing::upper_sensor::ecef_cm_to_llh)]
            struct Pos;

            #[tmv(crate::types::Vector3i32)]
            struct Vel;

            #[tmv(u8, sv = |v| crate::parsing::mask(2, 6, v), nav = |v| crate::parsing::mask(0, 2, v))]
            struct Status;
        }

        #[tmv(u16, pa = crate::parsing::upper_sensor::baro_pressure_convert_pa)]
        struct Baro;

        #[tmv(i16)]
        struct InternalTemperature;
    }

    mod lower_sensor {
        #[tmv(crate::types::lower_sensor::LowerSensorAdcValues,
            pres_ch1_pa = crate::parsing::lower_sensor::pascal_ch1,
            pres_ch2_pa = crate::parsing::lower_sensor::pascal_ch2,
            temp_c = crate::parsing::lower_sensor::celcius
        )]
        struct Adc;
    }
}
