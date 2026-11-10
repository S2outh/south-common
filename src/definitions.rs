use crate::parsing;
use crate::types;
use chell::chell_definition;
use nalgebra as na;

#[chell_definition(id = 0)]
mod command_msgs {

    /// The parent telecommand type accepted by the rocket avionics
    #[def(types::Telecommand)]
    #[command]
    struct Telecommand;

    #[def(())]
    /// A signal executed when the umbilical ground connection breaks
    struct LaunchDetected;
}

#[chell_definition(id = 5)]
mod timesync_msgs {

    /// A request for a sync message from a timesource
    #[def(())]
    #[id(range = 8)]
    struct TimesyncRequest;

    /// A sync message from a timesource
    #[def(types::Timesync)]
    #[id(range = 8)]
    struct TimesyncAnswer;
}

#[chell_definition(id = 30)]
pub mod telemetry {

    /// A utc timestamp in microseconds
    #[def(u64)]
    struct Timestamp;

    mod lst {
        /// The uptime of the CC1110 in seconds
        #[def(u32)]
        struct Uptime;

        /// How many packets were send since the last reboot
        #[def(u32)]
        struct PacketsSent;
    }

    mod eps {
        #[def(u8)]
        #[parser(flags -> types::eps::SourceEnabledKeyValue
            |v: &u8| types::eps::SourceEnabled::from_bits_truncate(*v).into()
        )]
        struct SourceEnabled;

        #[def(u8)]
        #[parser(flags -> types::eps::SinkEnabledKeyValue
            |v: &u8| types::eps::SinkEnabled::from_bits_truncate(*v).into()
        )]
        struct SinkEnabled;

        #[def(i16)]
        #[parser(c -> f32 |v| parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[def(i16)]
        #[parser(v -> f32 parsing::eps::bus_voltage_convert)]
        struct AuxPowerVoltage;

        #[def(i16)]
        #[parser(a -> f32 parsing::eps::shunt_current_convert)]
        struct AuxPowerCurrent;

        #[def(i16)]
        #[parser(v -> f32 parsing::eps::bus_voltage_convert)]
        struct Bat1Voltage;

        #[def(i16)]
        #[parser(c -> f32 |v| parsing::fixed_dec(10., v))]
        struct Bat1Temperature;

        #[def(i16)]
        #[parser(a -> f32 parsing::eps::shunt_current_convert)]
        struct Bat1Current;

        #[def(i16)]
        #[parser(v -> f32 parsing::eps::bus_voltage_convert)]
        struct Bat2Voltage;

        #[def(i16)]
        #[parser(c -> f32 |v| parsing::fixed_dec(10., v))]
        struct Bat2Temperature;

        #[def(i16)]
        #[parser(a -> f32 parsing::eps::shunt_current_convert)]
        struct Bat2Current;
    }

    mod upper_sensor {

        #[def(types::upper_sensor::Kinematics)]
        #[parser(llh -> types::upper_sensor::LLH
            |v: &types::upper_sensor::Kinematics| parsing::upper_sensor::ecef_to_llh(&v.position)
        )]
        struct KinematicState;

        #[def(na::Quaternion<f32>)]
        struct Orientation;

        mod imu1 {
            #[def(types::upper_sensor::AccelRaw)]
            #[parser(mps -> na::Vector3<f32> parsing::upper_sensor::accel_f32)]
            struct Accel;

            #[def(na::Vector3<i16>)]
            #[parser(rps -> na::Vector3<f32> parsing::upper_sensor::gyro_f32)]
            struct Gyro;
        }

        mod imu2 {
            #[def(types::upper_sensor::AccelRaw)]
            #[parser(mps -> na::Vector3<f32> parsing::upper_sensor::accel_f32)]
            struct Accel;

            #[def(na::Vector3<i16>)]
            #[parser(rps -> na::Vector3<f32> parsing::upper_sensor::gyro_f32)]
            struct Gyro;
        }

        mod gps {
            #[def(na::Vector3<i32>)]
            #[parser(m -> na::Vector3<f64> parsing::upper_sensor::ecef_cm_to_m)]
            #[parser(llh -> types::upper_sensor::LLH
                |v| parsing::upper_sensor::ecef_to_llh(&parsing::upper_sensor::ecef_cm_to_m(v))
            )]
            struct Pos;

            #[def(na::Vector3<i32>)]
            struct Vel;

            #[def(u8)]
            #[parser(sv -> u8 |v| parsing::mask(2, 6, v))]
            #[parser(nav -> u8 |v| parsing::mask(0, 2, v))]
            struct Status;
        }

        #[def(u16)]
        #[parser(pa -> f32 parsing::upper_sensor::baro_pressure_convert_pa)]
        struct Baro;

        #[def(na::Vector3<i32>)]
        #[parser(mt -> na::Vector3<f32> parsing::upper_sensor::mag_f32)]
        struct Magneto;

        #[def(i16)]
        #[parser(c -> f32 |v| parsing::fixed_dec(10., v))]
        struct InternalTemperature;
    }

    mod pyro {
        #[def(u8)]
        #[parser(flags -> types::pyro::StateFlagsKeyValue
            |v: &u8| types::pyro::StateFlags::from_bits_truncate(*v).into()
        )]
        struct Status;

        #[def(i16)]
        #[parser(c -> f32 |v| parsing::fixed_dec(10., v))]
        struct InternalTemperature;

        #[def(i16)]
        #[parser(v -> f32 |v| parsing::fixed_dec(1000., v))]
        struct Bat1Voltage;

        #[def(i16)]
        #[parser(v -> f32 |v| parsing::fixed_dec(1000., v))]
        struct Bat2Voltage;

        #[def(i16)]
        #[parser(v -> f32 |v| parsing::fixed_dec(1000., v))]
        struct Out1Voltage;

        #[def(i16)]
        #[parser(v -> f32 |v| parsing::fixed_dec(1000., v))]
        struct Out2Voltage;
    }

    mod lower_sensor {
        #[def(types::lower_sensor::LowerSensorAdcValues)]
        #[parser(pres_ch1_pa -> f32 parsing::lower_sensor::pascal_ch1)]
        #[parser(pres_ch2_pa -> f32 parsing::lower_sensor::pascal_ch2)]
        #[parser(temp_c -> f32 parsing::lower_sensor::celcius)]
        struct Adc;
    }
}

// ground only definitiona
#[chell_definition(id = 0)]
mod groundstation {
    mod primary_lst {
        #[def(u32)]
        struct Uptime;

        #[def(i8)]
        struct Rssi;

        #[def(u8)]
        struct Lqi;

        #[def(u32)]
        struct PacketsSent;

        #[def(u32)]
        struct PacketsGood;

        #[def(u32)]
        struct PacketsRejectedChecksum;

        #[def(u32)]
        struct PacketsRejectedOther;
    }

    mod secondary_lst {
        #[def(u32)]
        struct Uptime;

        #[def(i8)]
        struct Rssi;

        #[def(u8)]
        struct Lqi;

        #[def(u32)]
        struct PacketsSent;

        #[def(u32)]
        struct PacketsGood;

        #[def(u32)]
        struct PacketsRejectedChecksum;

        #[def(u32)]
        struct PacketsRejectedOther;
    }

    mod umbilical {
        #[def(u32)]
        struct TelecommandCounter;

        #[def(f32)]
        struct InternalTemperature;
    }

    mod trex {
        #[def(types::trex::State)]
        #[command]
        struct ChangeState;

        #[def(types::trex::Command)]
        #[command]
        struct Command;

        #[def(na::Vector2<f64>)]
        struct Angles;

        #[def(na::Vector2<f64>)]
        struct AngularVelocities;

        #[def(types::trex::State)]
        struct State;
    }
}
