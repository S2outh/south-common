pub mod eps;
pub mod lower_sensor;
pub mod upper_sensor;

use tmtc_system::*;

use crate::types::eps::EPSCommand;

#[derive(TMValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Timesync {
    request_id: u8,
    timestamp: u64,
}

// # Telecommands
#[derive(TMValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum Telecommand {
    RocketLST(LSTCommand),
    EPS(EPSCommand),
}

#[derive(TMValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum LSTCommand {
    Reboot,
}

// # vectors
#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3i16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3i32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
