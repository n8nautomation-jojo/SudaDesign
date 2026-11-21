//! Progressive multi-phase canvas refiner for extreme-scale documents.
//!
//! Orchestrates coarse interactive previewing vs fine-detail background refinement.
//! During pan/zoom gestures on massive canvases (e.g. 279 MP banners), rendering
//! starts at a coarse LOD (instant 60 FPS) and asynchronously refines to full
//! resolution only once the view settles.

use super::gesture_state::GestureState;

// ── RenderStage ──────────────────────────────────────────────────────────────

/// Current progressive refinement stage of a document canvas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum RenderStage {
    /// Initial or settled: fully refined at target resolution.
    #[default]
    Refined,
    /// Active gesture: fast coarse proxy displayed for 60 FPS fluidity.
    CoarseGesture,
    /// Gesture ended: settling debounce in progress before kicking off refine.
    Settling,
}

// ── ProgressiveRender ────────────────────────────────────────────────────────

/// Coordinates progressive refinement between active gestures and settled detail.
#[derive(Clone, Debug, Default)]
pub struct ProgressiveRender {
    /// Current render stage.
    pub stage: RenderStage,
    /// Navigation gesture tracking state.
    pub gesture: GestureState,
}

impl ProgressiveRender {
    /// Update the progressive render state based on current time and gesture activity.
    pub fn update(&mut self, now_ms: f64, interacting: bool, velocity: [f32; 2]) {
        self.gesture.set_interacting(interacting, now_ms);
        if interacting || velocity[0].abs() > 0.0 || velocity[1].abs() > 0.0 {
            self.gesture.record_motion(now_ms, velocity);
        }

        if self.gesture.is_active() {
            self.stage = RenderStage::CoarseGesture;
        } else if self.stage == RenderStage::CoarseGesture {
            self.stage = RenderStage::Settling;
        } else if self.stage == RenderStage::Settling && self.gesture.is_settled(now_ms) {
            self.stage = RenderStage::Refined;
        }
    }

    /// Whether background refinement should be deferred to preserve interactive frame rate.
    pub fn should_defer_refine(&self) -> bool {
        self.stage != RenderStage::Refined
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_refined_no_defer() {
        let p = ProgressiveRender::default();
        assert_eq!(p.stage, RenderStage::Refined);
        assert!(!p.should_defer_refine());
    }

    #[test]
    fn gesture_transitions_to_coarse_and_settling() {
        let mut p = ProgressiveRender::default();
        // Start gesture
        p.update(100.0, true, [0.0, 0.0]);
        assert_eq!(p.stage, RenderStage::CoarseGesture);
        assert!(p.should_defer_refine());

        // Release gesture
        p.update(120.0, false, [0.0, 0.0]);
        assert_eq!(p.stage, RenderStage::Settling);
        assert!(p.should_defer_refine());

        // After settle duration (80ms + 120ms = 200ms)
        p.update(210.0, false, [0.0, 0.0]);
        assert_eq!(p.stage, RenderStage::Refined);
        assert!(!p.should_defer_refine());
    }
}

