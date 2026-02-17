use bitflags::bitflags;
use tmtc_system::*;

// This is just a general collection of types that are needed across more than one repo. Better
// sorting mechanisms might be found in the future.

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

#[derive(TMValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum EPSCommand {
    SetSource(FlipFlopState, Option<u8>),
    EnableSink(Sink, Option<u8>),
    DisableSink(Sink, Option<u8>),
}

#[derive(TMValue, Clone, Copy)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum FlipFlopState {
    On,
    Bat1,
    Bat2,
    AuxPwr,
}

#[derive(TMValue, Clone, Copy)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum Sink {
    RocketLST,
    SensorUpper,
    GPS,
    RocketHD,
}

// # Telemetry types
bitflags! {
    #[cfg_attr(feature = "ground", derive(serde::Serialize))]
    pub struct EPSEnabled: u8 {
        const BAT1      = 1 << 0;
        const BAT2      = 1 << 1;
        const AUXPWR    = 1 << 2;
        const ROCKETLST = 1 << 3;
        const SENSORUPP = 1 << 4;
        const ROCKETHD  = 1 << 5;
    }
}
