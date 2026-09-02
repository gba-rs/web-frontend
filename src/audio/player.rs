use wasm_bindgen::JsValue;
use web_sys::AudioContext;

use super::resampler::Resampler;

const SCHEDULING_LOOKAHEAD_SECONDS: f64 = 0.1;
const FLUSH_THRESHOLD_SAMPLES: usize = 4096;

pub struct AudioPlayer {
    context: AudioContext,
    resampler: Resampler,
    next_start_time: f64,
    pending: Vec<i16>,
}

impl AudioPlayer {
    pub fn new(input_rate: u32) -> Result<AudioPlayer, JsValue> {
        let context = AudioContext::new()?;
        let output_rate = context.sample_rate() as u32;
        let next_start_time = context.current_time();
        Ok(AudioPlayer {
            context,
            resampler: Resampler::new(input_rate, output_rate),
            next_start_time,
            pending: Vec::with_capacity(FLUSH_THRESHOLD_SAMPLES * 2),
        })
    }

    pub fn resume(&self) -> Result<(), JsValue> {
        self.context.resume().map(|_| ())
    }

    pub fn push_samples(&mut self, samples: &[i16]) -> Result<(), JsValue> {
        if samples.is_empty() {
            return Ok(());
        }

        self.pending.extend_from_slice(samples);
        if self.pending.len() < FLUSH_THRESHOLD_SAMPLES {
            return Ok(());
        }

        self.flush()
    }

    pub fn flush(&mut self) -> Result<(), JsValue> {
        if self.pending.is_empty() {
            return Ok(());
        }
        let pending = std::mem::take(&mut self.pending);

        let resampled = self.resampler.process(&pending);
        let frame_count = resampled.len() / 2;
        if frame_count == 0 {
            return Ok(());
        }

        let sample_rate = self.context.sample_rate();
        let buffer = self.context.create_buffer(2, frame_count as u32, sample_rate)?;

        let mut left = vec![0f32; frame_count];
        let mut right = vec![0f32; frame_count];
        for i in 0..frame_count {
            left[i] = resampled[i * 2] as f32 / 32768.0;
            right[i] = resampled[i * 2 + 1] as f32 / 32768.0;
        }
        buffer.copy_to_channel(&mut left, 0)?;
        buffer.copy_to_channel(&mut right, 1)?;

        let source = self.context.create_buffer_source()?;
        source.set_buffer(Some(&buffer));
        source.connect_with_audio_node(&self.context.destination())?;

        let now = self.context.current_time();
        if self.next_start_time < now {
            self.next_start_time = now + SCHEDULING_LOOKAHEAD_SECONDS;
        }
        source.start_with_when(self.next_start_time)?;
        self.next_start_time += frame_count as f64 / sample_rate as f64;

        Ok(())
    }
}
