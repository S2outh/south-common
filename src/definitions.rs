use chell::*;

#[chell_definition(id = 0)]
mod internal_msgs {
    #[chv(crate::types::Telecommand)]
    struct Telecommand;

    #[chv(u8)]
    struct TimesyncRequest;

    #[chv(crate::types::Timesync)]
    struct TimesyncAnswer;
}

#[chell_definition(id = 10)]
pub mod telemetry {

    /// This is a utc timestamp in microseconds
    #[chv(u64)]
    struct Timestamp;

    mod lst {
        #[chv(u32)]
        struct Uptime;

        #[chv(i8)]
        struct Rssi;

        #[chv(u8)]
        struct Lqi;

        #[chv(u32)]
        struct PacketsSent;

        #[chv(u32)]
        struct PacketsGood;

        #[chv(u32)]
        struct PacketsRejectedChecksum;

        #[chv(u32)]
        struct PacketsRejectedOther;
    }

    mod eps {
        #[chv(u8,
            flags = |v: &u8| crate::parsing::names_from_bitflags(
                crate::types::eps::SourceEnabled::from_bits_truncate(*v))
            )
        ]
        struct SourceEnabled;

        #[chv(u8,
            flags = |v: &u8| crate::parsing::names_from_bitflags(
                crate::types::eps::SinkEnabled::from_bits_truncate(*v))
            )
        ]
        struct SinkEnabled;

        #[chv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[chv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct AuxPowerVoltage;

        #[chv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct AuxPowerCurrent;

        #[chv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct Bat1Voltage;

        #[chv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat1Temperature;

        #[chv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct Bat1Current;

        #[chv(i16, v = crate::parsing::eps::bus_voltage_convert)]
        struct Bat2Voltage;

        #[chv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct Bat2Temperature;

        #[chv(i16, a = crate::parsing::eps::shunt_current_convert)]
        struct Bat2Current;
    }

    mod upper_sensor {
        mod imu1 {
            #[chv(crate::types::upper_sensor::AccelRaw,
                mps = crate::parsing::upper_sensor::accel_f32
            )]
            struct Accel;

            #[chv(crate::types::Vector3i16,
                rps = crate::parsing::upper_sensor::gyro_f32
            )]
            struct Gyro;
        }

        mod imu2 {
            #[chv(crate::types::upper_sensor::AccelRaw,
                mps = crate::parsing::upper_sensor::accel_f32
            )]
            struct Accel;

            #[chv(crate::types::Vector3i16,
                rps = crate::parsing::upper_sensor::gyro_f32
            )]
            struct Gyro;
        }

        mod gps {
            #[chv(crate::types::Vector3i32, llh = crate::parsing::upper_sensor::ecef_cm_to_llh)]
            struct Pos;

            #[chv(crate::types::Vector3i32)]
            struct Vel;

            #[chv(u8, sv = |v| crate::parsing::mask(2, 6, v), nav = |v| crate::parsing::mask(0, 2, v))]
            struct Status;
        }

        #[chv(u16, pa = crate::parsing::upper_sensor::baro_pressure_convert_pa)]
        struct Baro;

        #[chv(crate::types::Vector3i32, mt = crate::parsing::upper_sensor::mag_f32)]
        struct Magneto;

        #[chv(i16)]
        struct InternalTemperature;
    }

    mod pyro {
        #[chv(u8,
            flags = |v: &u8| crate::parsing::names_from_bitflags(
                crate::types::pyro::StateFlags::from_bits_truncate(*v))
            )
        ]
        struct Status;

        #[chv(i16, c = |v| crate::parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[chv(i16, v = |v| crate::parsing::fixed_dec(1000., v))]
        struct Bat1Voltage;

        #[chv(i16, v = |v| crate::parsing::fixed_dec(1000., v))]
        struct Bat2Voltage;

        #[chv(i16, v = |v| crate::parsing::fixed_dec(1000., v))]
        struct Out1Voltage;

        #[chv(i16, v = |v| crate::parsing::fixed_dec(1000., v))]
        struct Out2Voltage;
    }

    mod lower_sensor {
        #[chv(crate::types::lower_sensor::LowerSensorAdcValues,
            pres_ch1_pa = crate::parsing::lower_sensor::pascal_ch1,
            pres_ch2_pa = crate::parsing::lower_sensor::pascal_ch2,
            temp_c = crate::parsing::lower_sensor::celcius
        )]
        struct Adc;
    }
}
