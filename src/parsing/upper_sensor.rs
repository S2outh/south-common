use crate::types::upper_sensor::BaroRaw;

// Imu

// Baro
pub fn baro_temp_convert(v: &BaroRaw) -> f32 {
    (v.temperature_data as f32 / 2047.0) * 200.0 - 50.0 // Temprature = 11 Bit -> 2^11 = 2048
    // Temp Range -50...150 C -> 150-(-50) = 200
    // Offset -50, so raw_temp = 0 -> -50 C
}

pub fn baro_pressure_convert_pa(v: &BaroRaw) -> f32 {
    const OUT_MIN: f32 = 1638.0; //10% of 2^14
    const OUT_MAX: f32 = 14745.0; //90% of 2^14
    const PRESSURE_RANGE_PA: f32 = 206_842.0; //30 psi in PA

    let c = (v.pressure_data as f32).clamp(OUT_MIN, OUT_MAX);

    (c - OUT_MIN) * PRESSURE_RANGE_PA / (OUT_MAX - OUT_MIN)
}


// GPS
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
