#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

#[cfg(feature = "ground")]
extern crate alloc;

pub mod beacons;
pub mod definitions;

pub mod configs;
pub mod types;
pub mod utils;

#[cfg(feature = "embedded")]
pub mod comms;

// Ground parsing
#[cfg(feature = "ground")]
mod parsing;

// public reexports
pub use chell;
