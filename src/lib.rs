#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

#[cfg(feature = "ground")]
extern crate alloc;

pub mod beacons;
pub mod definitions;

#[cfg(feature = "ground")]
mod parsers;

#[cfg(feature = "embedded")]
pub mod can_config;
pub mod types;

// public reexports
pub use tmtc_system;
