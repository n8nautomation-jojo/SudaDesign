//! Multi-resolution tile pyramid cache for layer and document surfaces.
//!
//! When navigating large documents at different zoom levels, repeatedly downsampling
//! surfaces via [`crate::proxy::downsample`] wastes CPU cycles. [`TilePyramid`]
//! caches downsampled copies of a [`photocraft_raster::Surface`] keyed by downscale
//! factor (2×, 4×, 8×, 16×, 32×).

use std::collections::HashMap;
use photocraft_raster::Surface;

use crate::lod::LodLevel;

// ── TilePyramid ──────────────────────────────────────────────────────────────

/// A multi-resolution surface cache holding power-of-2 downscaled representations.
#[derive(Clone, Debug, Default)]
pub struct TilePyramid {
    /// Cached downsampled surfaces keyed by integer downscale factor.
    levels: HashMap<u32, Surface>,
}

impl TilePyramid {
    /// Create a new, empty pyramid cache.
    pub fn new() -> Self {
        Self { levels: HashMap::new() }
    }

    /// Retrieve or compute a downscaled representation of `source` at `level`.
    pub fn get_or_compute(&mut self, source: &Surface, level: LodLevel) -> Surface {
        if !level.is_downscaled() {
            return source.clone();
        }
        if let Some(cached) = self.levels.get(&level.factor) {
            return cached.clone();
        }
        let downsampled = crate::proxy::downsample(source, level.factor);
        self.levels.insert(level.factor, downsampled.clone());
        downsampled
    }

    /// Clear all cached downsampled levels (e.g. when source is mutated or memory is low).
    pub fn clear(&mut self) {
        self.levels.clear();
    }

    /// Number of downscaled levels currently cached.
    pub fn len(&self) -> usize {
        self.levels.len()
    }

    /// True if no downscaled levels are currently cached.
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use photocraft_color::PixelFormat;

    #[test]
    fn new_pyramid_is_empty() {
        let p = TilePyramid::new();
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn get_base_level_returns_clone_without_caching() {
        let mut p = TilePyramid::new();
        let s = Surface::new(PixelFormat::RGBA8);
        let res = p.get_or_compute(&s, LodLevel::BASE);
        assert_eq!(res.format(), s.format());
        assert!(p.is_empty());
    }

    #[test]
    fn get_downscaled_caches_level() {
        let mut p = TilePyramid::new();
        let s = Surface::new(PixelFormat::RGBA8);
        let level = LodLevel { factor: 2, shift: 1 };
        let _ = p.get_or_compute(&s, level);
        assert_eq!(p.len(), 1);
        assert!(!p.is_empty());

        // Second fetch hits cache
        let _ = p.get_or_compute(&s, level);
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn clear_empties_pyramid() {
        let mut p = TilePyramid::new();
        let s = Surface::new(PixelFormat::RGBA8);
        let _ = p.get_or_compute(&s, LodLevel { factor: 4, shift: 2 });
        assert_eq!(p.len(), 1);
        p.clear();
        assert!(p.is_empty());
    }
}

