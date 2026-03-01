use tmtc_system::*;

#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct ImuRaw {
    accel_full_range: [i16; 3],
    accel_low_range: [i16; 3],
    gyro: [i16; 3],
}


#[derive(TMValue, Clone, Copy, Debug)]
#[cfg_attr(feature = "ground", derive(serde::Serialize))]
pub struct BaroRaw {
    pub status: u8,            // 2 bit
    pub pressure_data: u16,    //14 bit
    pub temperature_data: u16, //14 bit
}
