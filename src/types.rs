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
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum Telecommand {
    RocketLST(LSTCommand),
    EPS(EPSCommand),
    Pyro(PyroCommand),
}

pub struct NoCommand;

pub trait SubsystemCommand {
    fn from(cmd: Telecommand) -> Option<Self>
    where
        Self: Sized;
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
#[cfg_attr(feature = "ground", derive(serde::Serialize, serde::Deserialize))]
pub enum LSTCommand {
    Reboot,
}
