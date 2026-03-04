use tmtc_system::*;
use crate::types::Vector3i16;

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct AccelRaw {
    pub accel_full_range: Vector3i16,
    pub accel_low_range: Vector3i16,
}

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct LLH {
    pub lat: f32,
    pub lon: f32,
    pub h: f32,
}

