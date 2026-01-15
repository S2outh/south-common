#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod definitions;
mod beacons;

pub mod types;
#[cfg(feature = "embedded")]
pub mod can_config;

// public reexports
pub use definitions::{telemetry, telecommands};
pub use tmtc_system::*;
pub use beacons::lst_beacon::LSTBeacon;
pub use beacons::eps_beacon::EPSBeacon;
pub use beacons::sensorboard_beacon::SensorboardBeacon;
