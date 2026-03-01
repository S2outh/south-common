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

pub fn mask(shift: u8, length: u8, v: &u8) -> u8 {
    (*v >> shift) & ((1u8 << length) - 1)
}

pub fn ecef_cm_to_llh(ecef_cm: &[i32; 3]) -> [f64; 3] {
    // Returns: (latitude_deg, longitude_deg, height_m) on WGS84.
    const A: f64 = 6_378_137.0;
    const F: f64 = 1.0 / 298.257_223_563;
    const E2: f64 = F * (2.0 - F);
    const B: f64 = A * (1.0 - F);
    const RAD2DEG: f64 = 180.0 / core::f64::consts::PI;

    let x = ecef_cm[0] as f64 * 0.01;
    let y = ecef_cm[1] as f64 * 0.01;
    let z = ecef_cm[2] as f64 * 0.01;

    let lon = libm::atan2(y, x);
    let p = libm::sqrt(x * x + y * y);

    if p < 1e-9 {
        let lat = if z >= 0.0 {
            core::f64::consts::FRAC_PI_2
        } else {
            -core::f64::consts::FRAC_PI_2
        };
        let h = libm::fabs(z) - B;
        return [lat * RAD2DEG, lon * RAD2DEG, h];
    }

    let mut lat = libm::atan2(z, p * (1.0 - E2));
    let mut h = 0.0;

    for _ in 0..10 {
        let sin_lat = libm::sin(lat);
        let n = A / libm::sqrt(1.0 - E2 * sin_lat * sin_lat);
        let cos_lat = libm::cos(lat);
        h = p / cos_lat - n;
        let lat_next = libm::atan2(z, p * (1.0 - E2 * n / (n + h)));
        if libm::fabs(lat_next - lat) < 1e-13 {
            lat = lat_next;
            break;
        }
        lat = lat_next;
    }

    [lat * RAD2DEG, lon * RAD2DEG, h]
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
