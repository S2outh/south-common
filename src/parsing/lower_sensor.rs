// recording to IEC 60751 a resistor of a platinum RTD follows the Callendar-Van Dusen equation
// R(t) = R0 * (1 + At + Bt² + [C(t-100)t³])
// t>0: C = 0

use crate::types::lower_sensor::LowerSensorAdcValues;

const TEMP_A: f32 = 3.9083e-3;
const TEMP_B: f32 = -5.775e-7;
const R_0: f32 = 1000.0;

// max

pub fn celcius(raw: &LowerSensorAdcValues) -> f32 {
    const FSR: f32 = 2.048;

    // convert raw adc value to voltage
    let u = (raw.temp_ch as f32 * FSR) / 32768.0;

    // convert voltage to resistance with constant current source I = 1mA
    let r = u * 1000.0;

    // solve quadratic formula t = (-A + sqrt(D)) / 2B
    let d = TEMP_A * TEMP_A - 4.0 * TEMP_B * (1.0 - r / R_0);

    if d < 0.0 {
        return f32::NAN;
    }

    let sqrt_d = libm::sqrtf(d);
    let temperature = (-TEMP_A + sqrt_d) / (2.0 * TEMP_B);
    temperature as f32
}

pub fn pascal_ch1(raw: &LowerSensorAdcValues) -> f32 {
    const FSR: f32 = 4.096;

    let u_pin = (raw.pres_1_ch as f32 * FSR) / 32768.0;
    let u_sens = u_pin * 3.0;

    let p_bar = u_sens / 10.0 * 100.0;
    p_bar * 100_000.0
}

pub fn pascal_ch2(raw: &LowerSensorAdcValues) -> f32 {
    const FSR: f32 = 4.096;

    let u_pin = (raw.pres_2_ch as f32 * FSR) / 32768.0;
    let u_sens = u_pin * 3.0;

    let p_bar = u_sens / 10.0 * 100.0;
    p_bar * 100_000.0
}

/*
Für Kalibrierung später
- offset error
- gain error

-> (Wert - offset) * (1+ gain*1e-6)

let calibrated_data = match mode {
                    SensorMode::ADC => (average as f32 - offset) * (1.0 + (gain * 1e-6)),
                    SensorMode::Temp => {
                        let code_14bit = average >> 2;
                        // sign = MSB 0 or 1
                        if (code_14bit & 0x2000) == 0 {
                            // positiv
                            (code_14bit as f32) * 0.03125
                        } else {
                            //negative: 14-bit two's complement calculation
                            let mag = ((!code_14bit & 0x3FFF) + 1) as u16;
                            -(mag as f32) * 0.03125
                        }
                    }
                };
*/
