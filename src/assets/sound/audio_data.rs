use crate::assets::sound::utils::{i16_sample_to_f32, linear_stereo_interpolation, mono_to_stereo};
use crate::basics::safe::float::safe_mul;
use std::f32::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ChannelCount {
    Mono,
    Stereo,
}

#[derive(Debug)]
pub enum GetSamplesError {
    NoValidPosition,
    EmptyData,
    FirstFrameOOB,
}

#[derive(Debug, Clone)]
pub struct AudioData {
    pub sample_data: Arc<[i16]>,
    pub count_channels: ChannelCount,
    pub sample_rate: u32,
    pub step_width: f64,
    pub volume: f32,
}

impl AudioData {
    /* Uses linear interpolation ...
    HINT: If the audio data is mono (and next_sample is something), only the left channel's value is used for interpolation
    HINT: gain is ignored when having stereo audio.
    */
    pub fn get_samples(
        &self,
        pos: f32,
        next_sample: Option<[f32; 2]>,
        gain: Option<[f32; 2]>,
    ) -> Result<[f32; 2], GetSamplesError> {
        if pos < 0.0 {
            return Err(GetSamplesError::NoValidPosition);
        }

        let count_channels: usize = match self.count_channels {
            ChannelCount::Mono => 1,
            ChannelCount::Stereo => 2,
        };

        let total_samples = self.sample_data.len() / count_channels;

        if total_samples == 0 {
            return Err(GetSamplesError::EmptyData);
        }

        let mut samples: [f32; 2];

        let first_frame_index: usize = pos as usize;

        if first_frame_index >= total_samples {
            return Err(GetSamplesError::FirstFrameOOB);
        }

        let next_frame_index: usize;

        if first_frame_index + 1 < total_samples {
            next_frame_index = first_frame_index + 1;
        } else {
            next_frame_index = first_frame_index;
        }

        /* Used for linear interpolation */
        let sample_fraction: f32 = pos.fract();

        match self.count_channels {
            ChannelCount::Mono => {
                let loc_gain: [f32; 2] = gain.unwrap_or([FRAC_1_SQRT_2; 2]);

                let samp_first: f32 = i16_sample_to_f32(self.sample_data[first_frame_index]);
                let samp_next: f32;

                if next_sample.is_none() {
                    samp_next = i16_sample_to_f32(self.sample_data[next_frame_index]);
                } else {
                    samp_next = next_sample.unwrap()[0];
                }

                let f_with_gain: [f32; 2] = mono_to_stereo(samp_first, loc_gain);
                let n_with_gain: [f32; 2] = mono_to_stereo(samp_next, loc_gain);

                samples = linear_stereo_interpolation(sample_fraction, f_with_gain, n_with_gain)
            }
            ChannelCount::Stereo => {
                let samples_first: [f32; 2] = [
                    i16_sample_to_f32(self.sample_data[first_frame_index * 2]),
                    i16_sample_to_f32(self.sample_data[first_frame_index * 2 + 1]),
                ];

                let mut samples_next: [f32; 2] = [0.0; 2];

                if next_sample.is_none() {
                    samples_next[0] = i16_sample_to_f32(self.sample_data[next_frame_index * 2]);
                    samples_next[1] = i16_sample_to_f32(self.sample_data[next_frame_index * 2 + 1]);
                } else {
                    samples_next = next_sample.unwrap();
                }

                samples = linear_stereo_interpolation(sample_fraction, samples_first, samples_next);
            }
        };

        samples[0] = safe_mul(0.0, samples[0], self.volume);
        samples[1] = safe_mul(0.0, samples[1], self.volume);

        Ok(samples)
    }
}
