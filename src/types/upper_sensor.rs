use crate::types::Vector3i16;
use chell::*;

#[derive(ChellValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct AccelRaw {
    pub accel_full_range: Vector3i16,
    pub accel_low_range: Vector3i16,
}

impl From<([i16; 3], [i16; 3])> for AccelRaw {
    fn from(value: ([i16; 3], [i16; 3])) -> Self {
        Self {
            accel_low_range: value.0.into(),
            accel_full_range: value.1.into(),
        }
    }
}

#[derive(ChellValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct LLH {
    pub lat: f64,
    pub lon: f64,
    pub h: f64,
}
