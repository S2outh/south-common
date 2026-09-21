use chell::*;
use nalgebra as na;

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Command {
    State(State),
    Rotate(na::Vector2<f64>),
}


#[derive(ChellValue, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    Manual,
    Tracking,
}
