use crate::assets::sound::AudioData;

#[derive(Debug, Clone)]
pub enum LoopState {
    NoLoop,
    Loop(usize, usize), /* Loop begin, loop end */
}

#[derive(Debug, Clone)]
pub enum StreamState {
    NoStreaming,
    Stream(usize, usize), /* Streamed data begin, data end */
}

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub data: AudioData,
    pub st_state: StreamState,
    pub lp_state: LoopState,
    pub current_pos: f32,
    /* pub source */
}
