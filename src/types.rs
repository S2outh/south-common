pub mod eps;
pub mod lower_sensor;
pub mod upper_sensor;

use tmtc_system::*;

use crate::types::eps::EPSCommand;

#[derive(TMValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Timesync {
    pub request_id: u8,
    pub priority: u8,
    pub unix_time: u64,
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

impl From<[i16; 3]> for Vector3i16 {
    fn from(value: [i16; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3i32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<[i32; 3]> for Vector3i32 {
    fn from(value: [i32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vector3f32 {
    fn from(value: [f32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}
