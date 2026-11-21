//! Budgeting rules for viewport tile pooling.
//!
//! Decides whether a document should use full-document texture allocation or
//! fall back to dynamic viewport tile pooling based on system memory, available
//! GPU canvas budget, and platform constraints.

use super::budget::canvas_texture_bytes;
use super::viewport_pool::{DEFAULT_POOL_TILES_X, DEFAULT_POOL_TILES_Y};

// ── constants ────────────────────────────────────────────────────────────────

/// Maximum GPU memory the viewport tile pool may occupy (320 MB).
/// On mobile, this leaves ample VRAM for egui, UI textures, and the compositor.
pub const MAX_POOL_BYTES: u64 = 320 << 20;

/// Threshold above which Android automatically activates viewport pooling (256 MB).
pub const ANDROID_POOL_THRESHOLD: u64 = 256 << 20;

// ── functions ────────────────────────────────────────────────────────────────

/// Check whether the viewport tile pool should be active for a given document.
pub fn should_use_viewport_pool(full_canvas_bytes: u64, budget: u64) -> bool {
    if full_canvas_bytes > budget {
        return true;
    }
    if cfg!(target_os = "android") && full_canvas_bytes > ANDROID_POOL_THRESHOLD {
        return true;
    }
    false
}

/// Estimate default pool bytes when no viewport hint is yet available (e.g. initial doc load).
pub fn default_pool_bytes(tile: u32, bpp: u64) -> u64 {
    let pool_size = [tile * DEFAULT_POOL_TILES_X, tile * DEFAULT_POOL_TILES_Y];
    canvas_texture_bytes(pool_size, tile, bpp).min(MAX_POOL_BYTES)
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exceeds_budget_triggers_pool() {
        assert!(should_use_viewport_pool(500 << 20, 400 << 20));
    }

    #[test]
    fn fits_budget_does_not_trigger_on_desktop() {
        if !cfg!(target_os = "android") {
            assert!(!should_use_viewport_pool(100 << 20, 400 << 20));
        }
    }

    #[test]
    fn default_pool_bytes_is_bounded() {
        let bytes = default_pool_bytes(2048, 4);
        assert!(bytes > 0);
        assert!(bytes <= MAX_POOL_BYTES);
    }
}

