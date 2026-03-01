use tmtc_system::*;

#[derive(TMValue, Clone, Copy)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct LowerSensorAdcValues {
    pub pres_1_ch: i16,
    pub pres_2_ch: i16,
    pub temp_ch: i16,
    pub internal_temp: i16,
}
