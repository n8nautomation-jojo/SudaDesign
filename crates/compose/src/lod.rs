//! Level-of-detail (LOD) selection for reduced/downscaled composites.
//!
//! When rendering a massive document (e.g. 23,622 × 11,811) at a small preview
//! or screen resolution (e.g. 1,000 × 500), compositing every single full-res
//! pixel is wasteful. [`LodLevel`] chooses an optimal power-of-2 downscale factor
//! (1×, 2×, 4×, 8×, 16×, 32×) so that the working resolution is at least as large
//! as the target output while discarding up to 99 % of redundant composite work.

// ── constants ────────────────────────────────────────────────────────────────

/// Maximum LOD downscale power (2^5 = 32× downscaling).
pub const MAX_LOD_FACTOR: u32 = 32;

/// Minimum document pixel area (16 MP) before LOD reduction is activated.
pub const MIN_LOD_AREA: u64 = 16 << 20;

// ── LodLevel ─────────────────────────────────────────────────────────────────

/// Represents a chosen downscale level of detail for a composite operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LodLevel {
    /// Integer power-of-2 downscale factor (1, 2, 4, 8, 16, 32).
    pub factor: u32,
    /// Bit-shift equivalent to `factor` (`1 << shift == factor`).
    pub shift: u32,
}

impl LodLevel {
    /// Full resolution (1:1, no downscaling).
    pub const BASE: Self = Self { factor: 1, shift: 0 };

    /// Select the coarsest power-of-2 LOD level where downscaled dimensions
    /// remain at least as large as the target output (preserving the Nyquist limit).
    pub fn select(doc_w: u32, doc_h: u32, target_w: u32, target_h: u32) -> Self {
        if target_w == 0 || target_h == 0 || doc_w == 0 || doc_h == 0 {
            return Self::BASE;
        }
        let ratio_w = doc_w / target_w.max(1);
        let ratio_h = doc_h / target_h.max(1);
        let min_ratio = ratio_w.min(ratio_h);

        // Find largest power of 2 <= min_ratio / 2 (keeping 2x safety margin)
        // capped at MAX_LOD_FACTOR.
        let safe_ratio = min_ratio / 2;
        let mut factor = 1u32;
        let mut shift = 0u32;
        while factor * 2 <= safe_ratio && factor * 2 <= MAX_LOD_FACTOR {
            factor *= 2;
            shift += 1;
        }

        Self { factor, shift }
    }

    /// True if this level represents a downscaled representation (`factor > 1`).
    #[inline]
    pub fn is_downscaled(&self) -> bool {
        self.factor > 1
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_level_is_unity() {
        assert_eq!(LodLevel::BASE.factor, 1);
        assert_eq!(LodLevel::BASE.shift, 0);
        assert!(!LodLevel::BASE.is_downscaled());
    }

    #[test]
    fn select_degenerate_inputs_returns_base() {
        assert_eq!(LodLevel::select(0, 100, 10, 10), LodLevel::BASE);
        assert_eq!(LodLevel::select(100, 0, 10, 10), LodLevel::BASE);
        assert_eq!(LodLevel::select(100, 100, 0, 10), LodLevel::BASE);
        assert_eq!(LodLevel::select(100, 100, 10, 0), LodLevel::BASE);
    }

    #[test]
    fn select_same_size_returns_base() {
        assert_eq!(LodLevel::select(1000, 1000, 1000, 1000), LodLevel::BASE);
    }

    #[test]
    fn select_large_banner_returns_coarse_lod() {
        // 2m x 1m banner (23622 x 11811) at 1000x500 screen
        // ratio = 23, safe_ratio = 11 -> largest power of 2 <= 11 is 8
        let lod = LodLevel::select(23622, 11811, 1000, 500);
        assert_eq!(lod.factor, 8);
        assert_eq!(lod.shift, 3);
        assert!(lod.is_downscaled());
    }

    #[test]
    fn select_caps_at_max_lod_factor() {
        let lod = LodLevel::select(100000, 100000, 100, 100);
        assert_eq!(lod.factor, MAX_LOD_FACTOR);
    }
}

