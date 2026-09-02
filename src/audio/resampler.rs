pub struct Resampler {
    input_rate: f64,
    output_rate: f64,
    position: f64,
    last_left: i16,
    last_right: i16,
}

impl Resampler {
    pub fn new(input_rate: u32, output_rate: u32) -> Resampler {
        Resampler {
            input_rate: input_rate as f64,
            output_rate: output_rate as f64,
            position: 0.0,
            last_left: 0,
            last_right: 0,
        }
    }

    pub fn process(&mut self, input: &[i16]) -> Vec<i16> {
        let frame_count = input.len() / 2;
        if frame_count == 0 {
            return Vec::new();
        }

        let mut combined: Vec<i16> = Vec::with_capacity(input.len() + 2);
        combined.push(self.last_left);
        combined.push(self.last_right);
        combined.extend_from_slice(input);
        let combined_frames = frame_count + 1;

        let ratio = self.input_rate / self.output_rate;
        let mut output = Vec::new();
        loop {
            let idx = self.position.floor() as usize;
            if idx + 1 >= combined_frames {
                break;
            }
            let frac = (self.position - idx as f64) as f32;
            let l0 = combined[idx * 2] as f32;
            let r0 = combined[idx * 2 + 1] as f32;
            let l1 = combined[(idx + 1) * 2] as f32;
            let r1 = combined[(idx + 1) * 2 + 1] as f32;
            output.push((l0 + (l1 - l0) * frac).round() as i16);
            output.push((r0 + (r1 - r0) * frac).round() as i16);
            self.position += ratio;
        }

        self.position -= frame_count as f64;
        self.last_left = input[input.len() - 2];
        self.last_right = input[input.len() - 1];

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_through_at_equal_rates() {
        let mut r = Resampler::new(32768, 32768);
        let input = vec![100, -100, 200, -200, 300, -300];
        let output = r.process(&input);
        assert_eq!(output.len(), input.len());
    }

    #[test]
    fn upsamples_to_more_frames() {
        let mut r = Resampler::new(32768, 48000);
        let input = vec![0i16; 2000];
        let output = r.process(&input);
        assert!(output.len() > input.len());
    }

    #[test]
    fn downsamples_to_fewer_frames() {
        let mut r = Resampler::new(48000, 32768);
        let input = vec![0i16; 2000];
        let output = r.process(&input);
        assert!(output.len() < input.len());
    }

    #[test]
    fn preserves_phase_across_calls() {
        let mut r = Resampler::new(32768, 48000);
        let mut total_in = 0;
        let mut total_out = 0;
        for _ in 0..10 {
            let input = vec![1i16; 200];
            total_in += input.len() / 2;
            total_out += r.process(&input).len() / 2;
        }
        let expected = (total_in as f64 * 48000.0 / 32768.0) as usize;
        assert!(total_out.abs_diff(expected) <= 1);
    }

    #[test]
    fn empty_input_produces_empty_output_without_panicking() {
        let mut r = Resampler::new(32768, 48000);
        let output = r.process(&[]);
        assert!(output.is_empty());
    }

    #[test]
    fn carries_last_sample_across_calls_for_continuity() {
        let mut r = Resampler::new(32768, 32768);
        r.process(&[1000, -1000]);
        let output = r.process(&[2000, -2000]);
        assert_eq!(output[0], 1000);
        assert_eq!(output[1], -1000);
    }

    #[test]
    fn interpolates_linearly_between_frames() {
        let mut r = Resampler::new(2, 4);
        let output = r.process(&[0, 0, 100, 100]);
        let last_frame = output.len() - 2;
        assert_eq!(output[last_frame], 50);
        assert_eq!(output[last_frame + 1], 50);
    }
}
