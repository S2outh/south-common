pub mod eps;
pub mod lower_sensor;
pub mod upper_sensor;

use tmtc_system::*;

use crate::types::eps::EPSCommand;

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


