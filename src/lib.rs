#![no_std]
#![feature(const_trait_impl)]

mod definitions;
mod beacons;

pub mod command_types;
pub mod can_config;

// public reexports
pub use tmtc_system::TMValue;
pub use tmtc_system::DynTMValue;
pub use tmtc_system::DynBeacon;
pub use tmtc_system::DynTelemetryDefinition;
pub use beacons::low_rate_telemetry::LowRateTelemetry;
pub use beacons::mid_rate_telemetry::MidRateTelemetry;

