//! Ensure repeated compressed reads share their allocation and preserve pixel bytes.
use super::*;

#[test]
fn compressed_reads_keep_cached_arc_identity() {
    let mut surface = Surface::new(PixelFormat::RGBA8);
    surface.fill_rect(Rect::new(0, 0, 256, 256), &[0.25, 0.5, 0.75, 1.0]);
    let expected = surface.pixel(10, 10);
    assert!(surface.compress_dormant() > 0);
    let coord = TileCoord { tx: 0, ty: 0 };
    let mut first = None;
    assert!(surface.tile_bytes(&coord, &mut first).is_some());
    let mut second = None;
    assert!(surface.tile_bytes(&coord, &mut second).is_some());
    let (Some(a), Some(b)) = (&first, &second) else { panic!("cached tiles required") };
    assert!(Arc::ptr_eq(a, b));
    assert_eq!(surface.pixel(10, 10), expected);
}

#[test]
fn cache_eviction_does_not_change_decoded_pixels() {
    let mut surface = Surface::new(PixelFormat::RGBA8);
    surface.fill_rect(Rect::new(0, 0, 512, 512), &[0.1, 0.2, 0.3, 1.0]);
    let r = Rect::new(10, 10, 400, 400);
    let expected = surface.to_interleaved(r);
    assert!(surface.compress_dormant() > 0);
    assert_eq!(surface.to_interleaved(r), expected);
    surface.clear_tile_cache();
    assert_eq!(surface.to_interleaved(r), expected);
}
