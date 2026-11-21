//! LRU cache for recently-decompressed [`Tile`]s.
//!
//! When a tile is in [`Surface::compressed`], decompressing it via
//! `DeflateDecoder` costs 15–30 ms on ARM (a 256×256 CMYK tile ≈ 262 KB).
//! Repeated access during pan/zoom re-decompresses the same tile from scratch.
//! [`LruTileCache`] retains recently decompressed tiles in a bounded queue,
//! so the cost is paid at most once per tile per eviction cycle.

use std::collections::VecDeque;
use std::sync::Arc;

use photocraft_geom::TileCoord;

use crate::Tile;

// ── constants ────────────────────────────────────────────────────────────────

/// Sentinel byte count used when a tile has no data (should never be zero in
/// practice, but guards against zero-division in capacity maths).
const MIN_ENTRY_BYTES: usize = 1;

// ── LruEntry ─────────────────────────────────────────────────────────────────

/// One entry in the LRU queue.
struct LruEntry {
    /// Identity of the [`CompressedTile`] (Arc pointer address).
    /// Same pointer ⇒ same content, because CompressedTiles are Arc-shared and
    /// immutable once created.
    key: usize,
    /// Grid coordinate of this tile.
    coord: TileCoord,
    /// The decompressed tile data.
    tile: Arc<Tile>,
    /// Byte count charged against [`LruTileCache::capacity`].
    bytes: usize,
}

// ── LruTileCache ─────────────────────────────────────────────────────────────

/// A bounded LRU cache of recently-decompressed [`Tile`]s.
///
/// Tiles are keyed by `(TileCoord, ct_ptr)` where `ct_ptr` is the Arc pointer
/// address of the originating [`CompressedTile`].  Because `CompressedTile`s
/// are Arc-shared and immutable, equal pointers guarantee equal content, making
/// this a valid identity key without hashing the compressed bytes.
///
/// The queue is kept in MRU order (front = most recently used). `get` moves a
/// hit to the front; `insert` appends to the front and evicts from the back.
pub(crate) struct LruTileCache {
    /// Decompressed tiles in LRU order: front = most recently used.
    entries: VecDeque<LruEntry>,
    /// Total bytes currently held across all entries.
    bytes_used: usize,
    /// Maximum bytes this cache may hold before evicting.
    capacity: usize,
}

impl LruTileCache {
    /// Create a new cache with the given byte `capacity`.
    pub(crate) fn new(capacity: usize) -> Self {
        Self { entries: VecDeque::new(), bytes_used: 0, capacity }
    }

    /// Return the total bytes currently held by cached tiles.
    pub(crate) fn bytes_used(&self) -> usize {
        self.bytes_used
    }

    /// Number of entries currently in the cache.
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    /// Look up a cached tile by coordinate and compressed-tile pointer.
    ///
    /// On a hit the entry is moved to the front (MRU position) and a clone of
    /// the `Arc<Tile>` is returned.  Returns `None` on a miss.
    pub(crate) fn get(&mut self, coord: TileCoord, ct_ptr: usize) -> Option<Arc<Tile>> {
        let pos = self.entries.iter().position(|e| e.coord == coord && e.key == ct_ptr)?;
        // Rotate the hit to the front without reallocating.
        self.entries.rotate_left(pos);
        let entry = self.entries.front()?;
        Some(entry.tile.clone())
    }

    /// Insert a freshly-decompressed tile into the cache.
    ///
    /// If `coord`/`ct_ptr` is already present it is replaced in-place (the old
    /// entry is removed and the new one is pushed to the front).
    /// Excess entries are evicted from the back (LRU end) until
    /// `bytes_used <= capacity`.
    pub(crate) fn insert(&mut self, coord: TileCoord, ct_ptr: usize, tile: Arc<Tile>) {
        // Remove any stale entry for the same key first.
        if let Some(pos) = self.entries.iter().position(|e| e.coord == coord && e.key == ct_ptr) {
            let old = self.entries.remove(pos);
            self.bytes_used = self.bytes_used.saturating_sub(old.map_or(0, |o| o.bytes));
        }
        let bytes = tile.bytes().len().max(MIN_ENTRY_BYTES);
        let entry = LruEntry { key: ct_ptr, coord, tile, bytes };
        self.entries.push_front(entry);
        self.bytes_used = self.bytes_used.saturating_add(bytes);
        self.evict_to(self.capacity);
    }

    /// Evict all entries for `coord` (called when a tile is promoted from
    /// compressed → live resident, making the cache copy stale).
    pub(crate) fn remove_coord(&mut self, coord: TileCoord) {
        self.entries.retain(|e| {
            if e.coord == coord {
                self.bytes_used = self.bytes_used.saturating_sub(e.bytes);
                false
            } else {
                true
            }
        });
    }

    /// Purge all entries, releasing all cached tile memory.
    ///
    /// Call this under system memory pressure.
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
        self.bytes_used = 0;
    }

    /// Evict LRU entries until `bytes_used <= new_cap`.
    ///
    /// Used by [`crate::cache_policy::shrink_under_pressure`].
    pub(crate) fn trim_to(&mut self, new_cap: usize) {
        self.evict_to(new_cap);
    }

    // ── private helpers ──────────────────────────────────────────────────────

    /// Pop entries from the back until `bytes_used <= limit`.
    fn evict_to(&mut self, limit: usize) {
        while self.bytes_used > limit {
            match self.entries.pop_back() {
                Some(e) => self.bytes_used = self.bytes_used.saturating_sub(e.bytes),
                None => {
                    self.bytes_used = 0;
                    break;
                }
            }
        }
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use photocraft_color::PixelFormat;
    use photocraft_geom::TileCoord;

    fn make_tile(byte_val: u8, len: usize) -> Arc<Tile> {
        use crate::Surface;
        use photocraft_geom::Rect;
        let fmt = PixelFormat::GRAY8;
        let mut s = Surface::new(fmt);
        let fill: Vec<f32> = vec![byte_val as f32 / 255.0];
        s.fill_rect(Rect::new(0, 0, 1, 1), &fill);
        let _ = len;
        s.get_tile(TileCoord { tx: 0, ty: 0 }).unwrap_or_else(|| {
            Arc::new(crate::Tile { data: vec![byte_val; 64].into_boxed_slice() })
        })
    }

    #[test]
    fn get_on_empty_returns_none() {
        let mut cache = LruTileCache::new(1024 * 1024);
        let c = TileCoord { tx: 0, ty: 0 };
        assert!(cache.get(c, 0x1234).is_none());
    }

    #[test]
    fn insert_and_get_roundtrip() {
        let mut cache = LruTileCache::new(128 * 1024 * 1024);
        let c = TileCoord { tx: 1, ty: 2 };
        let tile = make_tile(42, 256);
        let ptr = Arc::as_ptr(&tile) as usize;
        cache.insert(c, ptr, tile.clone());
        let got = cache.get(c, ptr);
        assert!(got.is_some());
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn wrong_key_misses() {
        let mut cache = LruTileCache::new(128 * 1024 * 1024);
        let c = TileCoord { tx: 0, ty: 0 };
        let tile = make_tile(1, 64);
        let ptr = Arc::as_ptr(&tile) as usize;
        cache.insert(c, ptr, tile);
        assert!(cache.get(c, ptr + 1).is_none());
    }

    #[test]
    fn eviction_respects_capacity() {
        // capacity = 0 → every insert is evicted immediately
        let mut cache = LruTileCache::new(0);
        let c = TileCoord { tx: 0, ty: 0 };
        let tile = make_tile(7, 64);
        let ptr = Arc::as_ptr(&tile) as usize;
        cache.insert(c, ptr, tile);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn remove_coord_purges_entries() {
        let mut cache = LruTileCache::new(128 * 1024 * 1024);
        let c = TileCoord { tx: 3, ty: 4 };
        let tile = make_tile(9, 64);
        let ptr = Arc::as_ptr(&tile) as usize;
        cache.insert(c, ptr, tile);
        cache.remove_coord(c);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.bytes_used(), 0);
    }

    #[test]
    fn clear_empties_cache() {
        let mut cache = LruTileCache::new(128 * 1024 * 1024);
        for i in 0i32..5 {
            let c = TileCoord { tx: i, ty: 0 };
            let tile = make_tile(i as u8, 64);
            let ptr = Arc::as_ptr(&tile) as usize + i as usize;
            cache.insert(c, ptr, tile);
        }
        assert!(cache.len() > 0);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.bytes_used(), 0);
    }

    #[test]
    fn trim_to_evicts_lru() {
        let mut cache = LruTileCache::new(128 * 1024 * 1024);
        // Insert a tile then trim to 0
        let c = TileCoord { tx: 0, ty: 0 };
        let tile = make_tile(1, 64);
        let ptr = Arc::as_ptr(&tile) as usize;
        cache.insert(c, ptr, tile);
        cache.trim_to(0);
        assert_eq!(cache.len(), 0);
    }
}
