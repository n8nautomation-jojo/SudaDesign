//! Lightweight process-wide counters; no document content is retained.
use std::sync::atomic::{AtomicU64, Ordering};

static HITS: AtomicU64 = AtomicU64::new(0);
static MISSES: AtomicU64 = AtomicU64::new(0);
static DECODED: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, Default)]
pub struct TileReadStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub decoded_bytes: u64,
}

pub(crate) fn cache_hit() { HITS.fetch_add(1, Ordering::Relaxed); }
pub(crate) fn cache_miss() { MISSES.fetch_add(1, Ordering::Relaxed); }
pub(crate) fn decoded_bytes(bytes: usize) { DECODED.fetch_add(bytes as u64, Ordering::Relaxed); }

pub fn snapshot_and_reset() -> TileReadStats {
    TileReadStats {
        cache_hits: HITS.swap(0, Ordering::Relaxed),
        cache_misses: MISSES.swap(0, Ordering::Relaxed),
        decoded_bytes: DECODED.swap(0, Ordering::Relaxed),
    }
}
