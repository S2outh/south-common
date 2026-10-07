use chell::*;
use nalgebra as na;

#[derive(ChellValue)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Command {
    Set(na::Vector2<f64>),
    Rotate(na::Vector2<f64>),
}


#[derive(ChellValue, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateCmd {
    Manual,
    Tracking,
}
