#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_default)]
#![feature(iter_intersperse)]
#[allow(async_fn_in_trait)]
// TM definitions
pub mod definitions;

// TM types
pub mod types;

// Parsing functions
mod parsing;

// Beacon definitions
pub mod beacons;

// Common device configs
pub mod configs;

// util macros
pub mod macros;
pub use paste;

// Common onboard configs
#[cfg(feature = "embedded")]
pub mod obdh;
#[cfg(feature = "embedded")]
pub mod utils;

#[cfg(feature = "ground")]
extern crate alloc;

// timesync
#[cfg(feature = "timesync")]
pub mod timesync;

// public chell reexport
pub use chell;
