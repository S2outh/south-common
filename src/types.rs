pub mod eps;
pub mod lower_sensor;
pub mod pyro;
pub mod upper_sensor;

use chell::*;

use crate::types::{eps::EPSCommand, pyro::PyroCommand};

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Timesync {
    pub request_id: u8,
    pub priority: u8,
    pub unix_time_recv: u64,
    pub unix_time_snd: u64,
}

// # Telecommands
#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum Telecommand {
    RocketLST(LSTCommand),
    EPS(EPSCommand),
    Pyro(PyroCommand),
}

pub struct NoCommand;

pub trait SubsystemCommand {
    fn from(cmd: Telecommand) -> Option<Self> where Self: Sized;
}
impl SubsystemCommand for NoCommand {
    fn from(_cmd: Telecommand) -> Option<Self> {
        None
    }
}

macro_rules! derive_subsys_cmd {
    ($var: path, $cmd: ty) => {
        impl SubsystemCommand for $cmd {
            fn from(cmd: Telecommand) -> Option<Self> {
                if let $var(subsys_cmd) = cmd {
                    Some(subsys_cmd)
                } else {
                    None
                }
            }
        }
    };
}

derive_subsys_cmd!(Telecommand::RocketLST, LSTCommand);
derive_subsys_cmd!(Telecommand::EPS, EPSCommand);
derive_subsys_cmd!(Telecommand::Pyro, PyroCommand);

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum LSTCommand {
    Reboot,
}

// # vectors
#[derive(ChellValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3i16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl From<[i16; 3]> for Vector3i16 {
    fn from(value: [i16; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

#[derive(ChellValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3i32 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<[i32; 3]> for Vector3i32 {
    fn from(value: [i32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

#[derive(ChellValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct Vector3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vector3f32 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
