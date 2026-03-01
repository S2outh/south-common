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
    Carrier,
    RocketLST,
    GPS,
    ExternalCamera,
    SensorLower,
    RocketHD,
    BackupSink
}

// # Telemetry types
bitflags! {
    #[cfg_attr(feature = "ground", derive(serde::Serialize))]
    #[cfg_attr(feature = "ground", serde(transparent))]
    pub struct SinkEnabled: u8 {
        const CARRIER    = 1 << 0;
        const ROCKETLST  = 1 << 1;
        const GPS        = 1 << 2;
        const EXT_CAM    = 1 << 3;
        const LOWER_SENS = 1 << 4;
        const ROCKETHD   = 1 << 5;
        const BACKUP     = 1 << 6;
    }
}

bitflags! {
    #[cfg_attr(feature = "ground", derive(serde::Serialize))]
    #[cfg_attr(feature = "ground", serde(transparent))]
    pub struct SourceEnabled: u8 {
        const BAT_1    = 1 << 0;
        const BAT_2    = 1 << 1;
        const AUX_PWR  = 1 << 2;
    }
}
