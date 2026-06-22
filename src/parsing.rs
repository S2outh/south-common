use bitflags::Flags;

pub mod eps;
pub mod lower_sensor;
pub mod upper_sensor;

pub fn fixed_dec<T: Into<f32> + Clone>(shift: f32, v: &T) -> f32 {
    (*v).clone().into() / shift
}

pub fn mask(shift: u8, length: u8, v: &u8) -> u8 {
    (*v >> shift) & ((1u8 << length) - 1)
}

pub fn names_from_bitflags<F: Flags>(bitflags: F) -> alloc::string::String {
    bitflags
        .iter_names()
        .map(|(name, _flag)| name)
        .intersperse(" | ")
        .collect()
}
