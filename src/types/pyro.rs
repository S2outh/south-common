use bitflags::bitflags;
use chell::*;

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PyroChannel {
    Channel1,
    Channel2,
}

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PyroCommand {
    Arm(PyroChannel),
    Disarm(PyroChannel),
    Fire(PyroChannel),
}

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(u8)]
pub enum PyroState {
    Disarmed,
    Armed,
    Fired,
}

// # Telemetry types
key_value_bitflags! {
    pub struct StateFlags: u8 {
        const ARMD_A = 1 << 0;
        const FIRE_A = 1 << 1;
        const ARMD_B = 1 << 2;
        const FIRE_B = 1 << 3;
    }
}
