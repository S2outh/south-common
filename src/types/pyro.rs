use bitflags::bitflags;
use chell::*;

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum PyroChannel {
    Channel1,
    Channel2,
}

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum PyroCommand {
    Arm(PyroChannel),
    Disarm(PyroChannel),
    Fire(PyroChannel),
}

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum PyroState {
    Disarmed,
    Armed,
    Fired,
}

// # Telemetry types
bitflags! {
    pub struct StateFlags: u8 {
        const SAFE_A = 1 << 0;
        const FIRE_A = 1 << 1;
        const SAFE_B = 1 << 2;
        const FIRE_B = 1 << 3;
    }
}
