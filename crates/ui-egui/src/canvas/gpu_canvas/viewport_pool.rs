//! Viewport tile pool calculation for large documents on mobile GPUs.
//!
//! When a document exceeds the GPU texture budget (e.g. 2 m × 1 m at 300 DPI = ~1.5 GB),
//! [`ViewportHint`] determines the sub-rectangle of document tiles that are visible
//! in the current screen viewport plus a 1-tile safety margin, enabling GPU rendering
//! within a lean 100–300 MB footprint.

// ── constants ────────────────────────────────────────────────────────────────

/// Extra tile margin around visible viewport to prevent black edges during panning.
pub const MARGIN_TILES: u32 = 1;

/// Default tile count along X when no screen size is known.
pub const DEFAULT_POOL_TILES_X: u32 = 3;

/// Default tile count along Y when no screen size is known.
pub const DEFAULT_POOL_TILES_Y: u32 = 3;

// ── ViewportHint ─────────────────────────────────────────────────────────────

/// Visible area hint used to scope GPU tile allocations to the user's viewport.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportHint {
    /// Center of the view in document coordinates.
    pub center: [f32; 2],
    /// Zoom factor (1.0 = 100 %).
    pub zoom: f32,
    /// Physical screen/viewport size in pixels.
    pub screen_px: [u32; 2],
}

impl ViewportHint {
    /// Construct a hint from view parameters, returning `None` if zoom or screen is degenerate.
    pub fn from_view(center: [f32; 2], zoom: f32, screen_px: [u32; 2]) -> Option<Self> {
        if !zoom.is_finite() || zoom <= 1e-6 || screen_px[0] == 0 || screen_px[1] == 0 {
            return None;
        }
        if !center[0].is_finite() || !center[1].is_finite() {
            return None;
        }
        Some(Self { center, zoom, screen_px })
    }

    /// Calculate the visible document rectangle [x0, y0, x1, y1] clamped to `doc_size`.
    pub fn visible_rect(&self, doc_size: [u32; 2]) -> [f32; 4] {
        let half_w = (self.screen_px[0] as f32 / (2.0 * self.zoom)).max(1.0);
        let half_h = (self.screen_px[1] as f32 / (2.0 * self.zoom)).max(1.0);
        let x0 = (self.center[0] - half_w).clamp(0.0, doc_size[0] as f32);
        let y0 = (self.center[1] - half_h).clamp(0.0, doc_size[1] as f32);
        let x1 = (self.center[0] + half_w).clamp(0.0, doc_size[0] as f32);
        let y1 = (self.center[1] + half_h).clamp(0.0, doc_size[1] as f32);
        [x0, y0, x1, y1]
    }

    /// Compute the snapped tile clip rect `[cx0, cy0, cx1, cy1]` in document coordinates,
    /// including the 1-tile safety margin around the visible area.
    pub fn doc_clip(&self, tile: u32, doc_size: [u32; 2]) -> [u32; 4] {
        let tile = tile.max(1);
        let [vx0, vy0, vx1, vy1] = self.visible_rect(doc_size);
        let margin = (MARGIN_TILES * tile) as f32;

        let raw_x0 = (vx0 - margin).max(0.0) as u32;
        let raw_y0 = (vy0 - margin).max(0.0) as u32;
        let raw_x1 = ((vx1 + margin).min(doc_size[0] as f32) as u32).min(doc_size[0]);
        let raw_y1 = ((vy1 + margin).min(doc_size[1] as f32) as u32).min(doc_size[1]);

        // Snap outward to tile grid boundaries
        let cx0 = (raw_x0 / tile) * tile;
        let cy0 = (raw_y0 / tile) * tile;
        let cx1 = ((raw_x1 + tile - 1) / tile * tile).min(doc_size[0]);
        let cy1 = ((raw_y1 + tile - 1) / tile * tile).min(doc_size[1]);

        [cx0, cy0, cx1, cy1]
    }

    /// Return true when the user has panned or zoomed far enough from `prev`
    /// to warrant re-allocating or shifting the viewport tile pool.
    pub fn needs_reclip(&self, prev: &Self, tile: u32) -> bool {
        let threshold = (tile / 2).max(1) as f32;
        let dx = (self.center[0] - prev.center[0]).abs();
        let dy = (self.center[1] - prev.center[1]).abs();
        if dx >= threshold || dy >= threshold {
            return true;
        }
        let zoom_ratio = if prev.zoom > 0.0 { self.zoom / prev.zoom } else { 2.0 };
        zoom_ratio < 0.7 || zoom_ratio > 1.4
    }

    /// Estimate total GPU bytes required for the clipped viewport tile pool.
    pub fn pool_bytes(&self, tile: u32, bpp: u64, doc_size: [u32; 2]) -> u64 {
        let clip = self.doc_clip(tile, doc_size);
        crate::canvas::gpu_canvas::budget::clipped_canvas_bytes(clip, doc_size, tile, bpp)
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_view_returns_none() {
        assert!(ViewportHint::from_view([0.0, 0.0], 0.0, [100, 100]).is_none());
        assert!(ViewportHint::from_view([0.0, 0.0], -1.0, [100, 100]).is_none());
        assert!(ViewportHint::from_view([0.0, 0.0], 1.0, [0, 100]).is_none());
        assert!(ViewportHint::from_view([f32::NAN, 0.0], 1.0, [100, 100]).is_none());
    }

    #[test]
    fn valid_view_creates_hint() {
        let hint = ViewportHint::from_view([500.0, 500.0], 1.0, [1000, 1000]);
        assert!(hint.is_some());
        let hint = hint.unwrap();
        let rect = hint.visible_rect([2000, 2000]);
        assert_eq!(rect, [0.0, 0.0, 1000.0, 1000.0]);
    }

    #[test]
    fn doc_clip_snaps_to_tile_boundaries() {
        let hint = ViewportHint::from_view([2048.0, 2048.0], 1.0, [1024, 1024]).unwrap();
        let clip = hint.doc_clip(1024, [8192, 8192]);
        // Visible: [1536, 1536, 2560, 2560]
        // Margin: 1 tile (1024) -> [512, 512, 3584, 3584]
        // Snapped: [0, 0, 4096, 4096]
        assert_eq!(clip[0] % 1024, 0);
        assert_eq!(clip[1] % 1024, 0);
    }

    #[test]
    fn needs_reclip_detects_pan_and_zoom() {
        let v1 = ViewportHint::from_view([1000.0, 1000.0], 1.0, [1000, 1000]).unwrap();
        let v2 = ViewportHint::from_view([1010.0, 1000.0], 1.0, [1000, 1000]).unwrap();
        assert!(!v2.needs_reclip(&v1, 1024)); // Moved only 10 px (< 512 px)

        let v3 = ViewportHint::from_view([1600.0, 1000.0], 1.0, [1000, 1000]).unwrap();
        assert!(v3.needs_reclip(&v1, 1024)); // Moved 600 px (> 512 px)

        let v4 = ViewportHint::from_view([1000.0, 1000.0], 2.0, [1000, 1000]).unwrap();
        assert!(v4.needs_reclip(&v1, 1024)); // Zoomed 2x (> 1.4x)
    }
}

