pub const GBA_FRAME_SECONDS: f64 = 280896.0 / 16777216.0;
pub const MAX_CATCHUP_FRAMES: u32 = 30;
pub const TURBO_MULTIPLIER: f64 = 4.0;

pub struct FrameAccumulator {
    time_accumulator: f64,
}

impl FrameAccumulator {
    pub fn new() -> Self {
        FrameAccumulator { time_accumulator: 0.0 }
    }

    pub fn frames_to_run(&mut self, dt_seconds: f64) -> u32 {
        self.time_accumulator = (self.time_accumulator + dt_seconds)
            .min(GBA_FRAME_SECONDS * MAX_CATCHUP_FRAMES as f64);

        let mut count = 0;
        while self.time_accumulator >= GBA_FRAME_SECONDS {
            self.time_accumulator -= GBA_FRAME_SECONDS;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_delta_runs_no_frames() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(0.0), 0);
    }

    #[test]
    fn exactly_one_frame_worth_of_time_runs_one_frame() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS), 1);
    }

    #[test]
    fn small_increments_accumulate_into_a_frame() {
        let mut acc = FrameAccumulator::new();
        let step = GBA_FRAME_SECONDS / 4.0;
        assert_eq!(acc.frames_to_run(step), 0);
        assert_eq!(acc.frames_to_run(step), 0);
        assert_eq!(acc.frames_to_run(step), 0);
        assert_eq!(acc.frames_to_run(step), 1);
    }

    #[test]
    fn catchup_is_capped_at_max_catchup_frames() {
        let mut acc = FrameAccumulator::new();
        let huge_stall = GBA_FRAME_SECONDS * 1000.0;
        assert_eq!(acc.frames_to_run(huge_stall), MAX_CATCHUP_FRAMES);
    }

    #[test]
    fn leftover_fractional_time_carries_to_next_call() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 1.5), 1);
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 0.5), 1);
    }

    #[test]
    fn turbo_multiplier_quadruples_effective_frame_rate() {
        let mut acc = FrameAccumulator::new();
        let real_dt = GBA_FRAME_SECONDS;
        assert_eq!(acc.frames_to_run(real_dt * TURBO_MULTIPLIER), 4);
    }
}
