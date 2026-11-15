use crate::basics::safe::float::{safe_add, safe_div, safe_mul, safe_sub};
use std::f32::consts::FRAC_PI_2;

/* Following functions are AI generated and modified by me, except the last 2 ones that I wrote on my own. */

#[inline]
pub fn get_gain_by_pan(pan: f32) -> [f32; 2] {
    let pan2 = pan.clamp(-1.0, 1.0);

    let pan_plus_one: f32 = safe_add(0.0, pan2, 1.0);
    let pan_half: f32 = safe_mul(0.0, pan_plus_one, 0.5);
    let angle: f32 = safe_mul(0.0, pan_half, FRAC_PI_2);

    [angle.cos(), angle.sin()]
}

#[inline]
pub fn mono_to_stereo(sample: f32, gain: [f32; 2]) -> [f32; 2] {
    [sample * gain[0], sample * gain[1]]
}

#[inline]
pub fn i16_sample_to_f32(sample: i16) -> f32 {
    safe_div(0.0, sample as f32, i16::MAX as f32 + 1.0)
}

#[inline]
pub fn f32_sample_to_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);

    safe_mul(0.0, clamped, i16::MAX as f32).round() as i16
}

#[inline]
pub fn linear_interpolation(frac: f32, sample_1: f32, sample_2: f32) -> f32 {
    safe_add(
        0.0,
        sample_1,
        safe_mul(0.0, frac, safe_sub(0.0, sample_2, sample_1)),
    )
}

#[inline]
pub fn linear_stereo_interpolation(
    frac: f32,
    first_sample: [f32; 2],
    next_sample: [f32; 2],
) -> [f32; 2] {
    [
        linear_interpolation(frac, first_sample[0], next_sample[0]),
        linear_interpolation(frac, first_sample[1], next_sample[1]),
    ]
}
