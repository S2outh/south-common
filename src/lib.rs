#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

mod definitions;
mod beacons;

pub mod command_types;
#[cfg(feature = "embedded")]
pub mod can_config;

// public reexports
pub use definitions::{telemetry, telecommands};
pub use tmtc_system::{
    TMValue,
    DynTMValue,
    DynBeacon,
    DynTelemetryDefinition,
    TelemetryContainer,
    telemetry_container,
};
pub use beacons::low_rate_telemetry::LowRateTelemetry;
pub use beacons::mid_rate_telemetry::MidRateTelemetry;
