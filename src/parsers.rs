pub fn fixed_dec<T: Into<f64> + Clone>(shift: f64, v: &T) -> f64 {
    (*v).clone().into() / shift
}

pub fn split_byte<const N: usize>(lengths: [u8; N], v: &u8) -> [u8; N] {
    let mut vals = [0u8; N];
    let mut pos = 0;
    for i in 0..N {
        vals[i] = (v >> pos) & ((1u8 << lengths[i]) - 1);
        pos += lengths[i];
    }
    vals
}

// This is redefined from other repos, and a temporary solution
const TEMP_A: f64 = 3.9083e-3;
const TEMP_B: f64 = -5.775e-7;
const R_0: f64 = 1000.0;

pub fn temp_raw_to_celcius(raw: &i16) -> f32 {
    const FSR: f64 = 2.048;

    // convert raw adc value to voltage
    let u = (*raw as f64 * FSR) / 32768.0;

    // convert voltage to resistance with constant current source I = 1mA
    let r = u * 1000.0;

    // solve quadratic formula t = (-A + sqrt(D)) / 2B
    let d = TEMP_A * TEMP_A - 4.0 * TEMP_B * (1.0 - r / R_0);

    if d < 0.0 {
        return f32::NAN;
    }

    let sqrt_d = libm::sqrt(d);
    let temperature = (-TEMP_A + sqrt_d) / (2.0 * TEMP_B);
    temperature as f32
}

pub fn pres_raw_to_pascal(raw: &i16) -> f32 {
    const FSR: f32 = 4.096;

    let u_pin = (*raw as f32 * FSR) / 32768.0;
    let u_sens = u_pin * 3.0;

    let p_bar = u_sens / 10.0 * 100.0;
    p_bar * 100_000.0
}
