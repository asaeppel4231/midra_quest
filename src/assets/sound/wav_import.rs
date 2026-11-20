use crate::assets::sound::{AudioData, ChannelCount};
/* use crate::assets::archive::sound_read */
use crate::basics::safe::float::safe_div;
use hound::WavReader;

impl AudioData {
    pub fn import_wav(&mut self, device_sample_rate: u32, reader: impl std::io::Read) -> () {
        let mut handle = WavReader::new(reader).unwrap();

        let specifications = handle.spec();

        self.count_channels = match specifications.channels {
            1 => ChannelCount::Mono,
            2 => ChannelCount::Stereo,
            _ => panic!(
                "Audio file has unsupported number of channels (max. 2): {}",
                specifications.channels
            ),
        };

        self.sample_rate = specifications.sample_rate;
        self.step_width = safe_div(0.0, device_sample_rate as f32, self.sample_rate as f32) as f64;
        self.volume = 0.35; /* 35 % */

        /* Give me all of the samples !!! */
        self.sample_data = handle
            .samples::<i16>()
            .collect::<Result<Vec<i16>, hound::Error>>()
            .unwrap()
            .into();
    }
}
