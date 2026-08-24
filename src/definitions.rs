use chell::chell_definition;
use crate::types;
use crate::parsing;

#[chell_definition(id = 0)]
mod command_msgs {
    #[chv(types::Telecommand)]
    struct Telecommand;

    #[chv(())]
    struct LaunchDetected;
}

#[chell_definition(id = 5)]
mod timesync_msgs {
    #[chv((), id_range = 8)]
    struct TimesyncRequest;

    #[chv(types::Timesync, id_range = 8)]
    struct TimesyncAnswer;
}

#[chell_definition(id = 30)]
pub mod telemetry {

    /// This is a utc timestamp in microseconds
    #[chv(u64)]
    struct Timestamp;

    mod lst {
        #[chv(u32)]
        struct Uptime;

        #[chv(u32)]
        struct PacketsSent;
    }

    mod eps {
        #[chv(u8,
            flags(
                alloc::string::String,
                |v: &u8| parsing::names_from_bitflags(
                    types::eps::SourceEnabled::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct SourceEnabled;

        #[chv(u8,
            flags(
                alloc::string::String,
                |v: &u8| parsing::names_from_bitflags(
                    types::eps::SinkEnabled::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct SinkEnabled;

        #[chv(i16, c(f32, |v| parsing::fixed_dec(10., v)))]
        struct InternalTemperature;

        #[chv(i16, v(f32, parsing::eps::bus_voltage_convert))]
        struct AuxPowerVoltage;

        #[chv(i16, a(f32, parsing::eps::shunt_current_convert))]
        struct AuxPowerCurrent;

        #[chv(i16, v(f32, parsing::eps::bus_voltage_convert))]
        struct Bat1Voltage;

        #[chv(i16, c(f32, |v| parsing::fixed_dec(10., v)))]
        struct Bat1Temperature;

        #[chv(i16, a(f32, parsing::eps::shunt_current_convert))]
        struct Bat1Current;

        #[chv(i16, v(f32, parsing::eps::bus_voltage_convert))]
        struct Bat2Voltage;

        #[chv(i16, c(f32, |v| parsing::fixed_dec(10., v)))]
        struct Bat2Temperature;

        #[chv(i16, a(f32, parsing::eps::shunt_current_convert))]
        struct Bat2Current;
    }

    mod upper_sensor {

        #[chv(
            types::Vector3i32,
            m(types::Vector3f64, parsing::upper_sensor::ecef_cm_to_m),
            llh(
                types::upper_sensor::LLH,
                parsing::upper_sensor::ecef_cm_to_llh
            )
        )]
        struct Pos;

        #[chv(types::Vector3i32)]
        struct Vel;

        #[chv(types::Vector3i32)]
        struct Accel;

        mod imu1 {
            #[chv(
                types::upper_sensor::AccelRaw,
                mps(types::Vector3f32, parsing::upper_sensor::accel_f32)
            )]
            struct Accel;

            #[chv(
                types::Vector3i16,
                rps(types::Vector3f32, parsing::upper_sensor::gyro_f32)
            )]
            struct Gyro;
        }

        mod imu2 {
            #[chv(
                types::upper_sensor::AccelRaw,
                mps(types::Vector3f32, parsing::upper_sensor::accel_f32)
            )]
            struct Accel;

            #[chv(
                types::Vector3i16,
                rps(types::Vector3f32, parsing::upper_sensor::gyro_f32)
            )]
            struct Gyro;
        }

        mod gps {
            #[chv(
                types::Vector3i32,
                m(types::Vector3f64, parsing::upper_sensor::ecef_cm_to_m),
                llh(
                    types::upper_sensor::LLH,
                    parsing::upper_sensor::ecef_cm_to_llh
                )
            )]
            struct Pos;

            #[chv(types::Vector3i32)]
            struct Vel;

            #[chv(u8, sv(u8, |v| parsing::mask(2, 6, v)), nav(u8, |v| parsing::mask(0, 2, v)))]
            struct Status;
        }

        #[chv(u16, pa(f32, parsing::upper_sensor::baro_pressure_convert_pa))]
        struct Baro;

        #[chv(
            types::Vector3i32,
            mt(types::Vector3f32, parsing::upper_sensor::mag_f32)
        )]
        struct Magneto;

        #[chv(i16, c(f32, |v| parsing::fixed_dec(10., v)))]
        struct InternalTemperature;
    }

    mod pyro {
        #[chv(u8,
            flags(
                alloc::string::String,
                |v: &u8| parsing::names_from_bitflags(
                    types::pyro::StateFlags::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct Status;

        #[chv(i16, c(f32, |v| parsing::fixed_dec(10., v)))]
        struct InternalTemperature;

        #[chv(i16, v(f32, |v| parsing::fixed_dec(1000., v)))]
        struct Bat1Voltage;

        #[chv(i16, v(f32, |v| parsing::fixed_dec(1000., v)))]
        struct Bat2Voltage;

        #[chv(i16, v(f32, |v| parsing::fixed_dec(1000., v)))]
        struct Out1Voltage;

        #[chv(i16, v(f32, |v| parsing::fixed_dec(1000., v)))]
        struct Out2Voltage;
    }

    mod lower_sensor {
        #[chv(
            types::lower_sensor::LowerSensorAdcValues,
            pres_ch1_pa(f32, parsing::lower_sensor::pascal_ch1),
            pres_ch2_pa(f32, parsing::lower_sensor::pascal_ch2),
            temp_c(f32, parsing::lower_sensor::celcius)
        )]
        struct Adc;
    }
}
