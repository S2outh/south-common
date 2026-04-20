use bitflags::bitflags;
use chell::*;

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum PyroChannel {
    Channel1,
    Channel2,
}

#[derive(ChellValue)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub enum PyroCommand {
    Arm(PyroChannel),
    Disarm(PyroChannel),
    Fire(PyroChannel),
}

// # Telemetry types
bitflags! {
    #[cfg_attr(feature = "ground", derive(serde::Serialize))]
    #[cfg_attr(feature = "ground", serde(transparent))]
    pub struct StateFlags: u8 {
        const SAFE_A = 1 << 0;
        const FIRE_A = 1 << 1;
        const SAFE_B = 1 << 2;
        const FIRE_B = 1 << 3;
    }
}
