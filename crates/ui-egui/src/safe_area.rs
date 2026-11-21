//! Central Safe Area and Notch Inset Management System.
//!
//! Provides unified handling for:
//! - Android status bar & camera cutouts (punch-holes, notches)
//! - System navigation bars (gesture bar, 3-button navigation)
//! - Dynamic soft keyboard (IME) insets
//! - Edge-to-edge content protection for dialogs, popups, drawers, and floating HUDs.

use egui::{Align2, Area, Context, Id, Margin, Modal, Pos2, Rect, Vec2, pos2, vec2};

/// Key for storing runtime safe area insets in egui temporary context memory.
fn safe_area_insets_id() -> Id {
    Id::unique("suda_safe_area_insets_store")
}

/// Safe area and soft-keyboard insets (in logical points).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SafeAreaInsets {
    /// Left inset (e.g. landscape camera notch or bezel).
    pub left: f32,
    /// Top inset (system status bar, camera notch / punch hole).
    pub top: f32,
    /// Right inset (landscape camera notch or navigation bar).
    pub right: f32,
    /// Bottom inset (system navigation bar / gesture home indicator).
    pub bottom: f32,
    /// Soft keyboard (IME) inset height when virtual keyboard is active.
    pub ime: f32,
}

impl SafeAreaInsets {
    /// Effective bottom clearance taking the maximum of system navigation bar and virtual keyboard.
    #[inline]
    pub fn effective_bottom(&self) -> f32 {
        self.bottom.max(self.ime)
    }

    /// Whether all insets are zero (e.g. on traditional desktop displays).
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.left <= 0.0 && self.top <= 0.0 && self.right <= 0.0 && self.bottom <= 0.0 && self.ime <= 0.0
    }
}

/// Sets the current safe area insets on the egui Context.
pub fn set_safe_area_insets(ctx: &Context, insets: SafeAreaInsets) {
    ctx.data_mut(|d| {
        d.insert_temp(
            safe_area_insets_id(),
            SafeAreaInsets {
                left: insets.left.max(0.0),
                top: insets.top.max(0.0),
                right: insets.right.max(0.0),
                bottom: insets.bottom.max(0.0),
                ime: insets.ime.max(0.0),
            },
        );
    });
}

/// Retrieves the active safe area insets for this frame.
pub fn get_safe_area_insets(ctx: &Context) -> SafeAreaInsets {
    ctx.data(|d| d.get_temp(safe_area_insets_id())).unwrap_or_default()
}

/// Stores the current IME (soft keyboard) height in logical points.
pub fn set_ime_inset(ctx: &Context, ime: f32) {
    let mut current = get_safe_area_insets(ctx);
    current.ime = ime.max(0.0);
    set_safe_area_insets(ctx, current);
}

/// Retrieves the current IME (soft keyboard) height in logical points.
pub fn get_ime_inset(ctx: &Context) -> f32 {
    get_safe_area_insets(ctx).ime
}

/// Returns the safe visible screen rectangle, excluding status bar, cutouts, navigation bar, and IME.
pub fn safe_rect(ctx: &Context) -> Rect {
    let content = ctx.content_rect();
    let insets = get_safe_area_insets(ctx);
    let bottom_inset = insets.effective_bottom();

    let min_x = content.min.x + insets.left;
    let min_y = content.min.y + insets.top;
    let max_x = (content.max.x - insets.right).max(min_x);
    let max_y = (content.max.y - bottom_inset).max(min_y);

    Rect::from_min_max(pos2(min_x, min_y), pos2(max_x, max_y))
}

/// Returns safe insets formatted as an `egui::Margin` (in logical points).
pub fn safe_margin(ctx: &Context) -> Margin {
    let insets = get_safe_area_insets(ctx);
    Margin {
        left: insets.left.round() as i8,
        right: insets.right.round() as i8,
        top: insets.top.round() as i8,
        bottom: insets.effective_bottom().round() as i8,
    }
}

/// Prepares an `egui::Area` for a modal dialog, anchored within the screen's safe bounds.
///
/// Ensures the modal center aligns with `safe_rect(ctx).center()`, preventing its title bar
/// or action buttons from colliding with camera notches or system navigation bars.
pub fn modal_area(ctx: &Context, id: Id) -> Area {
    let safe = safe_rect(ctx);
    let full = ctx.input(|i| i.viewport_rect());
    let center_offset = safe.center() - full.center();
    Modal::default_area(id).anchor(Align2::CENTER_CENTER, center_offset)
}

/// Clamps a rectangle completely inside `safe_rect(ctx)`.
pub fn clamp_rect_to_safe(ctx: &Context, rect: Rect) -> Rect {
    let safe = safe_rect(ctx);
    let w = rect.width().min(safe.width());
    let h = rect.height().min(safe.height());

    let x = rect.min.x.clamp(safe.left(), (safe.right() - w).max(safe.left()));
    let y = rect.min.y.clamp(safe.top(), (safe.bottom() - h).max(safe.top()));

    Rect::from_min_size(pos2(x, y), vec2(w, h))
}

/// Clamps a top-left position for an element of `size` within `safe_rect(ctx)` with an optional margin.
pub fn clamp_pos_to_safe(ctx: &Context, pos: Pos2, size: Vec2, margin: f32) -> Pos2 {
    let safe = safe_rect(ctx);
    let min_x = safe.left() + margin;
    let min_y = safe.top() + margin;
    let max_x = (safe.right() - size.x - margin).max(min_x);
    let max_y = (safe.bottom() - size.y - margin).max(min_y);

    pos2(pos.x.clamp(min_x, max_x), pos.y.clamp(min_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_area_insets_storage_roundtrip() {
        let ctx = Context::default();
        let insets = SafeAreaInsets {
            left: 12.0,
            top: 48.0,
            right: 16.0,
            bottom: 34.0,
            ime: 280.0,
        };
        set_safe_area_insets(&ctx, insets);
        assert_eq!(get_safe_area_insets(&ctx), insets);
        assert_eq!(get_ime_inset(&ctx), 280.0);
        assert_eq!(insets.effective_bottom(), 280.0);
    }
}

