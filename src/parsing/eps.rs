pub fn bus_voltage_convert(v: &i16) -> f32 {
    // 8mV per LSB
    (*v as f32 * 8.) / 1000.
}
pub fn shunt_current_convert(v: &i16) -> f32 {
    // 40uV per LSB + 50mOhm shunt resistor
    const SHUNT_RESISTOR: f32 = 0.050;
    let voltage = (*v as f32 * 40.) / 1_000_000.;
    voltage / SHUNT_RESISTOR
}
