#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod beacons;
mod definitions;

#[cfg(feature = "embedded")]
pub mod can_config;
pub mod types;

// public reexports
pub use beacons::eps_beacon::EPSBeacon;
pub use beacons::lst_beacon::LSTBeacon;
pub use beacons::low_rate_upper_sensor_beacon::LowRateUpperSensorBeacon;
pub use beacons::high_rate_upper_sensor_beacon::HighRateUpperSensorBeacon;
pub use beacons::lower_sensor_beacon::LowerSensorBeacon;
pub use definitions::{telecommands, telemetry};
pub use tmtc_system::*;
