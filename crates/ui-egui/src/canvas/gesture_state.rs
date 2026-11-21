//! Touch and pan/zoom navigation gesture tracker.
//!
//! Prevents heavy background CPU/GPU tile re-rendering during active continuous
//! touch gestures (pan, pinch-zoom, inertial flings). Implements an 80 ms settling
//! debounce window before scheduling expensive fine-detail refinement.

// ── constants ────────────────────────────────────────────────────────────────

/// Settling debounce duration in milliseconds (80 ms).
/// After touch release or motion stop, heavy refinement waits this long.
pub const SETTLE_DEBOUNCE_MS: f64 = 80.0;

/// Velocity threshold (pixels/sec) above which motion is considered active fling.
pub const FLING_VELOCITY_THRESHOLD: f32 = 10.0;

// ── GestureState ─────────────────────────────────────────────────────────────

/// Tracks the active state and timing of canvas pan/zoom navigation gestures.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureState {
    /// True while finger/stylus/wheel is actively down or dragging.
    pub is_interacting: bool,
    /// Timestamp (ms) of the last detected camera motion.
    pub last_motion_ms: f64,
    /// Estimated pointer/scroll velocity in screen points per second.
    pub velocity: [f32; 2],
}

impl Default for GestureState {
    fn default() -> Self {
        Self {
            is_interacting: false,
            last_motion_ms: 0.0,
            velocity: [0.0, 0.0],
        }
    }
}

impl GestureState {
    /// Record active motion at timestamp `now_ms`.
    pub fn record_motion(&mut self, now_ms: f64, velocity: [f32; 2]) {
        self.last_motion_ms = now_ms;
        self.velocity = velocity;
    }

    /// Update interaction state (e.g. pointer down/up).
    pub fn set_interacting(&mut self, interacting: bool, now_ms: f64) {
        self.is_interacting = interacting;
        if interacting {
            self.last_motion_ms = now_ms;
        }
    }

    /// Check if the canvas is currently undergoing active gesture motion.
    /// Returns true while touching, or during inertial motion above threshold.
    pub fn is_active(&self) -> bool {
        if self.is_interacting {
            return true;
        }
        let speed_sq = self.velocity[0] * self.velocity[0] + self.velocity[1] * self.velocity[1];
        speed_sq > FLING_VELOCITY_THRESHOLD * FLING_VELOCITY_THRESHOLD
    }

    /// Check if motion has ceased and the settling debounce interval has elapsed.
    pub fn is_settled(&self, now_ms: f64) -> bool {
        if self.is_active() {
            return false;
        }
        (now_ms - self.last_motion_ms) >= SETTLE_DEBOUNCE_MS
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_idle() {
        let g = GestureState::default();
        assert!(!g.is_active());
        assert!(g.is_settled(100.0));
    }

    #[test]
    fn interacting_is_active_not_settled() {
        let mut g = GestureState::default();
        g.set_interacting(true, 10.0);
        assert!(g.is_active());
        assert!(!g.is_settled(100.0));
    }

    #[test]
    fn release_settles_after_debounce() {
        let mut g = GestureState::default();
        g.set_interacting(true, 10.0);
        g.set_interacting(false, 20.0);
        assert!(!g.is_active());
        assert!(!g.is_settled(50.0)); // 30ms < 80ms
        assert!(g.is_settled(110.0)); // 90ms >= 80ms
    }

    #[test]
    fn fling_velocity_keeps_active() {
        let mut g = GestureState::default();
        g.record_motion(10.0, [20.0, 0.0]);
        assert!(g.is_active());
    }
}

