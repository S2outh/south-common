use chell::*;
use nalgebra as na;

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Command {
    State(StateCommand),
    RotateAz(f32),
    RotateEl(f32),
    Target(na::Vector3<f64>)
}


#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateCommand {
    ManualControl,
    Tracking,
    Stop,
}
