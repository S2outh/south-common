#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(iter_intersperse)]

// TM definitions
pub mod definitions;

// TM types
pub mod types;

// Beacon definitions
pub mod beacons;

// Common device configs
pub mod configs;

// util macros
pub mod macros;
pub use paste;

// Common onboard configs
#[cfg(feature = "embedded")]
pub mod utils;
#[cfg(feature = "embedded")]
pub mod obdh;

// Common ground configs
#[cfg(feature = "ground")]
mod parsing;

#[cfg(feature = "ground")]
extern crate alloc;

// public chell reexport
pub use chell;
