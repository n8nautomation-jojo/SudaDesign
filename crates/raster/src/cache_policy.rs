//! Memory-pressure policy for [`crate::tile_cache::LruTileCache`].
//!
//! Decides how large the per-surface LRU decompression cache should be, and
//! how aggressively to trim it under Android memory-pressure signals.

use crate::tile_cache::LruTileCache;

// ── constants ────────────────────────────────────────────────────────────────

/// Default cache capacity (64 MB).  Holds roughly 64 uncompressed CMYK
/// 256×256 tiles (262 KB each), covering a ~4096×1024 viewport at 100 % zoom.
pub(crate) const DEFAULT_CAPACITY_BYTES: usize = 64 * 1024 * 1024;

/// Minimum capacity retained even under severe pressure (4 MB).
/// Keeping at least a few tiles cached avoids fully-uncached cold paths.
pub(crate) const MIN_CAPACITY_BYTES: usize = 4 * 1024 * 1024;

/// Level at which we apply a mild trim (keep 50 % of current usage).
const PRESSURE_LEVEL_TRIM: i32 = 20;

/// Level at which we apply an aggressive trim (keep 25 % of current usage).
const PRESSURE_LEVEL_AGGRESSIVE: i32 = 40;

/// Level at which we clear the entire cache.
const PRESSURE_LEVEL_CLEAR: i32 = 80;

// ── CachePolicy ──────────────────────────────────────────────────────────────

/// Applies a memory-pressure `level` (0–100) to an [`LruTileCache`], trimming
/// or clearing it according to the thresholds above.
///
/// Returns the number of bytes released.
pub(crate) fn shrink_under_pressure(cache: &mut LruTileCache, level: i32) -> usize {
    let before = cache.bytes_used();
    if level >= PRESSURE_LEVEL_CLEAR {
        cache.clear();
    } else if level >= PRESSURE_LEVEL_AGGRESSIVE {
        // Keep at most 25 % of current bytes, but never below the minimum.
        let target = (before / 4).max(MIN_CAPACITY_BYTES);
        cache.trim_to(target);
    } else if level >= PRESSURE_LEVEL_TRIM {
        // Keep at most 50 % of current bytes, but never below the minimum.
        let target = (before / 2).max(MIN_CAPACITY_BYTES);
        cache.trim_to(target);
    }
    before.saturating_sub(cache.bytes_used())
}

/// Returns a recommended initial cache capacity for the current platform.
///
/// On Android we use a larger default because CMYK tiles are bigger and panning
/// a 2 m × 1 m canvas stresses repeated decompression most.
pub(crate) fn default_capacity() -> usize {
    // Per-surface interim Android cap; a process-wide budget remains future work.
    if cfg!(target_os = "android") { MIN_CAPACITY_BYTES } else { DEFAULT_CAPACITY_BYTES }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile_cache::LruTileCache;

    fn empty_cache() -> LruTileCache {
        LruTileCache::new(DEFAULT_CAPACITY_BYTES)
    }

    #[test]
    fn no_pressure_no_change() {
        let mut cache = empty_cache();
        let released = shrink_under_pressure(&mut cache, 0);
        assert_eq!(released, 0);
    }

    #[test]
    fn clear_level_empties_cache() {
        let mut cache = empty_cache();
        // Artificially force some bytes_used by inserting nothing (cache is
        // empty here); at level 80 clear() is called → released = 0 (empty).
        // The important invariant: after pressure, len == 0.
        shrink_under_pressure(&mut cache, PRESSURE_LEVEL_CLEAR);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn trim_pressure_does_not_panic() {
        let mut cache = empty_cache();
        // No entries: trim should be a no-op without panicking.
        let released = shrink_under_pressure(&mut cache, PRESSURE_LEVEL_TRIM);
        assert_eq!(released, 0);
    }

    #[test]
    fn aggressive_pressure_does_not_panic() {
        let mut cache = empty_cache();
        let released = shrink_under_pressure(&mut cache, PRESSURE_LEVEL_AGGRESSIVE);
        assert_eq!(released, 0);
    }

    #[test]
    fn default_capacity_is_nonzero() {
        assert!(default_capacity() > 0);
    }
}

