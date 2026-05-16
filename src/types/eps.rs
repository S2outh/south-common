use bitflags::bitflags;
use chell::*;

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum EPSCommand {
    EnableSource(FlipFlopInput),
    DisableSource(FlipFlopInput),
    RebootSource(FlipFlopInput),
    EnableSink(Sink),
    DisableSink(Sink),
    RebootSink(Sink),
}

#[derive(ChellValue, Clone, Copy)]
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum FlipFlopInput {
    Bat1,
    Bat2,
    AuxPwr,
}

#[derive(ChellValue, Clone, Copy)]
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum Sink {
    Carrier,     // pa5
    Umbilical,   // pa3
    RocketLst1,  // pc6
    RocketLst2,  // pa9
    SensorLower, // pa8
    RocketHD,    // pa15
    BackupSink,  // pa4
}

// # Telemetry types
bitflags! {
    pub struct SinkEnabled: u8 {
        const CARRIER       = 1 << 0;
        const UMBILICAL     = 1 << 1;
        const ROCKET_LST_1  = 1 << 2;
        const ROCKET_LST_2  = 1 << 3;
        const SENSOR_LOWER  = 1 << 4;
        const ROCKET_H_D    = 1 << 5;
        const BACKUP_SINK   = 1 << 6;
    }
}

bitflags! {
    pub struct SourceEnabled: u8 {
        const BAT_1    = 1 << 0;
        const BAT_2    = 1 << 1;
        const AUX_PWR  = 1 << 2;
    }
}
