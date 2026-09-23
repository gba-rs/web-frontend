pub const GBA_FRAME_SECONDS: f64 = 280896.0 / 16777216.0;
// A single frame costs close to the full real-time budget in wasm, so a large
// catch-up allowance (fine on native, where a frame is cheap) turns any small
// deficit into a compounding stall instead of a brief, recoverable one.
pub const MAX_CATCHUP_FRAMES: u32 = 2;
pub const TURBO_MULTIPLIER: f64 = 4.0;
// Turbo deliberately scales dt by TURBO_MULTIPLIER before it reaches the
// accumulator, so its cap must comfortably clear that multiplier or turbo
// would be silently throttled back down to the normal-play cap.
pub const MAX_CATCHUP_FRAMES_TURBO: u32 = 6;

pub struct FrameAccumulator {
    time_accumulator: f64,
}

impl FrameAccumulator {
    pub fn new() -> Self {
        FrameAccumulator { time_accumulator: 0.0 }
    }

    pub fn frames_to_run(&mut self, dt_seconds: f64, max_frames: u32) -> u32 {
        self.time_accumulator += dt_seconds.max(0.0)
            .min(GBA_FRAME_SECONDS * max_frames as f64);

        let mut count = 0;
        while count < max_frames && self.time_accumulator >= GBA_FRAME_SECONDS {
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
        assert_eq!(acc.frames_to_run(0.0, MAX_CATCHUP_FRAMES), 0);
    }

    #[test]
    fn exactly_one_frame_worth_of_time_runs_one_frame() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS, MAX_CATCHUP_FRAMES), 1);
    }

    #[test]
    fn small_increments_accumulate_into_a_frame() {
        let mut acc = FrameAccumulator::new();
        let step = GBA_FRAME_SECONDS / 4.0;
        assert_eq!(acc.frames_to_run(step, MAX_CATCHUP_FRAMES), 0);
        assert_eq!(acc.frames_to_run(step, MAX_CATCHUP_FRAMES), 0);
        assert_eq!(acc.frames_to_run(step, MAX_CATCHUP_FRAMES), 0);
        assert_eq!(acc.frames_to_run(step, MAX_CATCHUP_FRAMES), 1);
    }

    #[test]
    fn catchup_is_capped_at_max_catchup_frames() {
        let mut acc = FrameAccumulator::new();
        let huge_stall = GBA_FRAME_SECONDS * 1000.0;
        assert_eq!(acc.frames_to_run(huge_stall, MAX_CATCHUP_FRAMES), MAX_CATCHUP_FRAMES);
    }

    #[test]
    fn leftover_fractional_time_carries_to_next_call() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 1.5, MAX_CATCHUP_FRAMES), 1);
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 0.5, MAX_CATCHUP_FRAMES), 1);
    }

    #[test]
    fn turbo_multiplier_quadruples_effective_frame_rate() {
        let mut acc = FrameAccumulator::new();
        let real_dt = GBA_FRAME_SECONDS;
        assert_eq!(acc.frames_to_run(real_dt * TURBO_MULTIPLIER, MAX_CATCHUP_FRAMES_TURBO), 4);
    }

    #[test]
    fn turbo_cap_is_not_throttled_by_the_normal_play_cap() {
        assert!(MAX_CATCHUP_FRAMES_TURBO as f64 >= TURBO_MULTIPLIER);
    }
    #[test]
    fn missed_refresh_does_not_discard_existing_fraction() {
        let mut acc = FrameAccumulator::new();
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 0.75, 2), 0);
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 2.0, 2), 2);
        assert_eq!(acc.frames_to_run(GBA_FRAME_SECONDS * 0.25, 2), 1);
    }

}
