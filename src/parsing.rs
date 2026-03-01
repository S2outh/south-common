pub mod upper_sensor;
pub mod lower_sensor;


pub fn fixed_dec<T: Into<f64> + Clone>(shift: f64, v: &T) -> f64 {
    (*v).clone().into() / shift
}

pub fn mask(shift: u8, length: u8, v: &u8) -> u8 {
    (*v >> shift) & ((1u8 << length) - 1)
}


