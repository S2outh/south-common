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
pub use beacons::sensorboard_beacon::SensorboardBeacon;
pub use definitions::{telecommands, telemetry};
pub use tmtc_system::*;
