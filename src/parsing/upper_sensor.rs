use lsm6dsv32::config::{FifoDisabled, ImuConfig, Int1Disabled, Int2Disabled};
use spin::Lazy;

use crate::{configs::{imu_config, mag_config}, types::{Vector3f32, Vector3i16, Vector3i32, upper_sensor::{AccelRaw, LLH}}};

static IMU_CONFIG: Lazy<ImuConfig<FifoDisabled, Int1Disabled, Int2Disabled>> = Lazy::new(imu_config::get_imu_config);
static GYRO_SCALING: Lazy<f32> = Lazy::new(|| IMU_CONFIG.gyro.calc_scaling_factor(true));
static ACCEL_LOW_RANGE_SCALING: Lazy<f32> = Lazy::new(|| IMU_CONFIG.accel.calc_scaling_factor(true));
static ACCEL_FULL_RANGE_SCALING: Lazy<f32> = Lazy::new(|| IMU_CONFIG.accel.calc_scaling_factor_ch2(true));
static IMU_ACCEL_SCALING_THRESHOLD: Lazy<f32> = Lazy::new(|| libm::powf(2., 2. + (IMU_CONFIG.accel.full_scale as u8 as f32)));

static MAG_CONFIG: Lazy<rm3100::config::Config> = Lazy::new(mag_config::get_mag_config);
static MAG_SCALING: Lazy<f32> = Lazy::new(|| MAG_CONFIG.calc_scaling_factor());

// Imu
pub fn gyro_f32(raw: &Vector3i16) -> Vector3f32 {
    Vector3f32 {
        x: raw.x as f32 * *GYRO_SCALING,
        y: raw.y as f32 * *GYRO_SCALING,
        z: raw.z as f32 * *GYRO_SCALING,
    }
}

pub fn accel_f32(raw: &AccelRaw) -> Vector3f32 {
    let raw_low = raw.accel_low_range;
    let raw_full = raw.accel_full_range;
    
    let calib_low = Vector3f32 {
        x: raw_low.x as f32 * *ACCEL_LOW_RANGE_SCALING,
        y: raw_low.y as f32 * *ACCEL_LOW_RANGE_SCALING,
        z: raw_low.z as f32 * *ACCEL_LOW_RANGE_SCALING,
    };

    let calib_full = Vector3f32 {
        x: raw_full.x as f32 * *ACCEL_FULL_RANGE_SCALING,
        y: raw_full.y as f32 * *ACCEL_FULL_RANGE_SCALING,
        z: raw_full.z as f32 * *ACCEL_FULL_RANGE_SCALING,
    };

    Vector3f32 {
        x: if calib_full.x < *IMU_ACCEL_SCALING_THRESHOLD { calib_low.x } else { calib_full.x },
        y: if calib_full.y < *IMU_ACCEL_SCALING_THRESHOLD { calib_low.y } else { calib_full.y },
        z: if calib_full.z < *IMU_ACCEL_SCALING_THRESHOLD { calib_low.z } else { calib_full.z },
    }
}

// Magneto
pub fn mag_f32(raw: &Vector3i32) -> Vector3f32 {
    Vector3f32 {
        x: raw.x as f32 * *MAG_SCALING,
        y: raw.y as f32 * *MAG_SCALING,
        z: raw.z as f32 * *MAG_SCALING,
    }
}

// Baro
// pub fn baro_temp_convert(v: &u16) -> f32 {
//     (*v as f32 / 2047.0) * 200.0 - 50.0 // Temprature = 11 Bit -> 2^11 = 2048
//     // Temp Range -50...150 C -> 150-(-50) = 200
//     // Offset -50, so raw_temp = 0 -> -50 C
// }

pub fn baro_pressure_convert_pa(v: &u16) -> f32 {
    const OUT_MIN: f32 = 1638.0; //10% of 2^14
    const OUT_MAX: f32 = 14745.0; //90% of 2^14
    const PRESSURE_RANGE_PA: f32 = 206_842.0; //30 psi in PA

    let c = (*v as f32).clamp(OUT_MIN, OUT_MAX);

    (c - OUT_MIN) * PRESSURE_RANGE_PA / (OUT_MAX - OUT_MIN)
}


// GPS
pub fn ecef_cm_to_llh(ecef_cm: &Vector3i32) -> LLH {
    // Returns: (latitude_deg, longitude_deg, height_m) on WGS84.
    const A: f32 = 6_378_137.0;
    const F: f32 = 1.0 / 298.257_223_563;
    const E2: f32 = F * (2.0 - F);
    const B: f32 = A * (1.0 - F);
    const RAD2DEG: f32 = 180.0 / core::f32::consts::PI;

    let x = ecef_cm.x as f32 * 0.01;
    let y = ecef_cm.y as f32 * 0.01;
    let z = ecef_cm.z as f32 * 0.01;

    let lon = libm::atan2f(y, x);
    let p = libm::sqrtf(x * x + y * y);

    if p < 1e-9 {
        let lat = if z >= 0.0 {
            core::f32::consts::FRAC_PI_2
        } else {
            -core::f32::consts::FRAC_PI_2
        };
        let h = libm::fabsf(z) - B;
        return LLH { lat: lat * RAD2DEG, lon: lon * RAD2DEG, h};
    }

    let mut lat = libm::atan2f(z, p * (1.0 - E2));
    let mut h = 0.0;

    for _ in 0..10 {
        let sin_lat = libm::sinf(lat);
        let n = A / libm::sqrtf(1.0 - E2 * sin_lat * sin_lat);
        let cos_lat = libm::cosf(lat);
        h = p / cos_lat - n;
        let lat_next = libm::atan2f(z, p * (1.0 - E2 * n / (n + h)));
        if libm::fabsf(lat_next - lat) < 1e-13 {
            lat = lat_next;
            break;
        }
        lat = lat_next;
    }

    LLH { lat: lat * RAD2DEG, lon: lon * RAD2DEG, h}
}
