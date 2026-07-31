use chell::*;

#[chell_definition(id = 0)]
mod command_msgs {
    #[chv(crate::types::Telecommand)]
    struct Telecommand;

    #[chv(())]
    struct LaunchDetected;
}

#[chell_definition(id = 5)]
mod timesync_msgs {
    #[chv((), id_range = 8)]
    struct TimesyncRequest;

    #[chv(crate::types::Timesync, id_range = 8)]
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
                |v: &u8| crate::parsing::names_from_bitflags(
                    crate::types::eps::SourceEnabled::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct SourceEnabled;

        #[chv(u8,
            flags(
                alloc::string::String,
                |v: &u8| crate::parsing::names_from_bitflags(
                    crate::types::eps::SinkEnabled::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct SinkEnabled;

        #[chv(i16, c(f32, |v| crate::parsing::fixed_dec(10., v)))]
        struct InternalTemperature;

        #[chv(i16, v(f32, crate::parsing::eps::bus_voltage_convert))]
        struct AuxPowerVoltage;

        #[chv(i16, a(f32, crate::parsing::eps::shunt_current_convert))]
        struct AuxPowerCurrent;

        #[chv(i16, v(f32, crate::parsing::eps::bus_voltage_convert))]
        struct Bat1Voltage;

        #[chv(i16, c(f32, |v| crate::parsing::fixed_dec(10., v)))]
        struct Bat1Temperature;

        #[chv(i16, a(f32, crate::parsing::eps::shunt_current_convert))]
        struct Bat1Current;

        #[chv(i16, v(f32, crate::parsing::eps::bus_voltage_convert))]
        struct Bat2Voltage;

        #[chv(i16, c(f32, |v| crate::parsing::fixed_dec(10., v)))]
        struct Bat2Temperature;

        #[chv(i16, a(f32, crate::parsing::eps::shunt_current_convert))]
        struct Bat2Current;
    }

    mod upper_sensor {
        mod imu1 {
            #[chv(
                crate::types::upper_sensor::AccelRaw,
                mps(crate::types::Vector3f32, crate::parsing::upper_sensor::accel_f32)
            )]
            struct Accel;

            #[chv(
                crate::types::Vector3i16,
                rps(crate::types::Vector3f32, crate::parsing::upper_sensor::gyro_f32)
            )]
            struct Gyro;
        }

        mod imu2 {
            #[chv(
                crate::types::upper_sensor::AccelRaw,
                mps(crate::types::Vector3f32, crate::parsing::upper_sensor::accel_f32)
            )]
            struct Accel;

            #[chv(
                crate::types::Vector3i16,
                rps(crate::types::Vector3f32, crate::parsing::upper_sensor::gyro_f32)
            )]
            struct Gyro;
        }

        mod gps {
            #[chv(
                crate::types::Vector3i32,
                m(crate::types::Vector3f64, crate::parsing::upper_sensor::ecef_cm_to_m),
                llh(
                    crate::types::upper_sensor::LLH,
                    crate::parsing::upper_sensor::ecef_cm_to_llh
                )
            )]
            struct Pos;

            #[chv(crate::types::Vector3i32)]
            struct Vel;

            #[chv(u8, sv(u8, |v| crate::parsing::mask(2, 6, v)), nav(u8, |v| crate::parsing::mask(0, 2, v)))]
            struct Status;
        }

        #[chv(u16, pa(f32, crate::parsing::upper_sensor::baro_pressure_convert_pa))]
        struct Baro;

        #[chv(
            crate::types::Vector3i32,
            mt(crate::types::Vector3f32, crate::parsing::upper_sensor::mag_f32)
        )]
        struct Magneto;

        #[chv(i16, c(f32, |v| crate::parsing::fixed_dec(10., v)))]
        struct InternalTemperature;
    }

    mod pyro {
        #[chv(u8,
            flags(
                alloc::string::String,
                |v: &u8| crate::parsing::names_from_bitflags(
                    crate::types::pyro::StateFlags::from_bits_truncate(*v)
                ),
                ground
            )
        )]
        struct Status;

        #[chv(i16, c(f32, |v| crate::parsing::fixed_dec(10., v)))]
        struct InternalTemperature;

        #[chv(i16, v(f32, |v| crate::parsing::fixed_dec(1000., v)))]
        struct Bat1Voltage;

        #[chv(i16, v(f32, |v| crate::parsing::fixed_dec(1000., v)))]
        struct Bat2Voltage;

        #[chv(i16, v(f32, |v| crate::parsing::fixed_dec(1000., v)))]
        struct Out1Voltage;

        #[chv(i16, v(f32, |v| crate::parsing::fixed_dec(1000., v)))]
        struct Out2Voltage;
    }

    mod lower_sensor {
        #[chv(
            crate::types::lower_sensor::LowerSensorAdcValues,
            pres_ch1_pa(f32, crate::parsing::lower_sensor::pascal_ch1),
            pres_ch2_pa(f32, crate::parsing::lower_sensor::pascal_ch2),
            temp_c(f32, crate::parsing::lower_sensor::celcius)
        )]
        struct Adc;
    }
}
