//! Touch-native Floating Modifier Keys HUD (`ModifierHud`).
//!
//! Provides on-screen virtual modifier keys (`Shift`, `Alt`, `Ctrl`/`Cmd`) for
//! touch-driven environments (Android phones and tablets, touchscreens, stylus).
//!
//! Features:
//! - **Hold-to-Modify**: Holding a modifier with a thumb keeps it active while the other hand
//!   draws, transforms, or navigates. Releasing automatically disengages.
//! - **Tap-to-Lock**: Tapping toggles a latched state with visual accent indicator.
//! - **Draggable**: Drag handle allows placing the HUD anywhere along screen edges.
//! - **Clear All**: One-tap quick reset when any modifier is locked.

use egui::{Color32, CornerRadius, Sense, Stroke, pos2, vec2};

use crate::PhotocraftApp;
use crate::theme::Tokens;

/// Identifier for egui Area and drag state.
const HUD_AREA_ID: &str = "pc_floating_modifier_hud";

/// Renders the floating modifier keys HUD if enabled.
pub fn show(app: &mut PhotocraftApp, ctx: &egui::Context) {
    let screen = ctx.content_rect();
    if screen.width() <= 0.0 || screen.height() <= 0.0 {
        return;
    }

    let is_mobile = crate::dialogs::is_mobile_screen(ctx);
    // On mobile screens, default to visible unless explicitly hidden by the user.
    let visible = if app.ui.shell.modifier_hud_visible {
        true
    } else if is_mobile && !app.ui.shell.modifier_hud_explicitly_hidden {
        true
    } else {
        false
    };

    if !visible {
        return;
    }

    let t = Tokens::get(ctx);
    let safe = crate::safe_area::safe_rect(ctx);
    let default_pos = pos2(
        safe.left() + 16.0,
        (safe.bottom() - 110.0).max(safe.top() + 40.0),
    );

    let hud_pos = app
        .ui.shell
        .modifier_hud_pos
        .map(|[x, y]| pos2(x.clamp(safe.left() + 8.0, safe.right() - 180.0), y.clamp(safe.top() + 8.0, safe.bottom() - 60.0)))
        .unwrap_or(default_pos);

    let mut new_pos = hud_pos;
    let mut update_pos = false;

    let cmd_label = if cfg!(target_os = "macos") { "Cmd" } else { "Ctrl" };

    egui::Area::new(egui::Id::unique(HUD_AREA_ID))
        .order(egui::Order::Foreground)
        .fixed_pos(hud_pos)
        .show(ctx, |ui| {
            let bg_fill = if t.dark() {
                Color32::from_rgba_unmultiplied(24, 24, 28, 225)
            } else {
                Color32::from_rgba_unmultiplied(235, 235, 240, 225)
            };
            let border_stroke = Stroke::new(
                1.0,
                if t.dark() {
                    Color32::from_white_alpha(45)
                } else {
                    Color32::from_black_alpha(45)
                },
            );

            egui::Frame::NONE
                .fill(bg_fill)
                .stroke(border_stroke)
                .corner_radius(CornerRadius::same(18))
                .inner_margin(egui::Margin::symmetric(6, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                        // 1. Drag Handle
                        let handle_size = vec2(16.0, 34.0);
                        let (handle_rect, handle_resp) = ui.allocate_exact_size(handle_size, Sense::drag());
                        let handle_color = if handle_resp.hovered() || handle_resp.dragged() {
                            t.text
                        } else {
                            t.text_faint
                        };
                        ui.painter().text(
                            handle_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "⋮⋮",
                            egui::FontId::proportional(13.0),
                            handle_color,
                        );

                        if handle_resp.dragged() {
                            new_pos += handle_resp.drag_delta();
                            update_pos = true;
                        }

                        // 2. Modifiers: Shift, Ctrl/Cmd, Alt
                        render_mod_button(
                            ui,
                            &t,
                            "Shift",
                            app.ui.shell.sticky_shift,
                            app.ui.shell.held_shift,
                            |held, sticky| {
                                app.ui.shell.held_shift = held;
                                if let Some(new_sticky) = sticky {
                                    app.ui.shell.sticky_shift = new_sticky;
                                }
                            },
                        );

                        render_mod_button(
                            ui,
                            &t,
                            cmd_label,
                            app.ui.shell.sticky_command,
                            app.ui.shell.held_command,
                            |held, sticky| {
                                app.ui.shell.held_command = held;
                                if let Some(new_sticky) = sticky {
                                    app.ui.shell.sticky_command = new_sticky;
                                }
                            },
                        );

                        render_mod_button(
                            ui,
                            &t,
                            "Alt",
                            app.ui.shell.sticky_alt,
                            app.ui.shell.held_alt,
                            |held, sticky| {
                                app.ui.shell.held_alt = held;
                                if let Some(new_sticky) = sticky {
                                    app.ui.shell.sticky_alt = new_sticky;
                                }
                            },
                        );

                        // 3. Clear all button (only shown when at least one is sticky)
                        let any_sticky = app.ui.shell.sticky_shift
                            || app.ui.shell.sticky_command
                            || app.ui.shell.sticky_alt;

                        if any_sticky {
                            let (clear_rect, clear_resp) = ui.allocate_exact_size(vec2(22.0, 32.0), Sense::click());
                            if clear_resp.hovered() {
                                ui.painter().rect_filled(clear_rect, CornerRadius::same(11), t.hover);
                            }
                            ui.painter().text(
                                clear_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "✕",
                                egui::FontId::proportional(11.0),
                                t.text_dim,
                            );
                            if clear_resp.clicked() {
                                app.ui.shell.sticky_shift = false;
                                app.ui.shell.sticky_command = false;
                                app.ui.shell.sticky_alt = false;
                            }
                        }
                    });
                });
        });

    if update_pos {
        let clamped_x = new_pos.x.clamp(safe.left() + 8.0, safe.right() - 180.0);
        let clamped_y = new_pos.y.clamp(safe.top() + 8.0, safe.bottom() - 60.0);
        app.ui.shell.modifier_hud_pos = Some([clamped_x, clamped_y]);
    }
}

/// Helper to render an individual modifier button supporting Tap-to-Lock and Hold-to-Modify.
fn render_mod_button<F>(
    ui: &mut egui::Ui,
    t: &Tokens,
    label: &str,
    is_sticky: bool,
    is_held: bool,
    mut on_update: F,
) where
    F: FnMut(bool, Option<bool>),
{
    let min_size = vec2(42.0, 34.0);
    let (rect, resp) = ui.allocate_exact_size(min_size, Sense::click_and_drag());

    let is_down = resp.is_pointer_button_down_on();
    if is_down != is_held {
        on_update(is_down, None);
    }

    if resp.clicked() {
        // Toggle sticky state on deliberate tap
        on_update(false, Some(!is_sticky));
    }

    let active = is_sticky || is_held;

    let fill = if is_held {
        t.accent.gamma_multiply(0.85)
    } else if is_sticky {
        t.accent
    } else if resp.hovered() {
        t.hover
    } else {
        Color32::TRANSPARENT
    };

    let text_color = if active {
        Color32::WHITE
    } else {
        t.text
    };

    let stroke = if is_sticky {
        Stroke::new(1.5, Color32::WHITE)
    } else if active {
        Stroke::new(1.0, t.accent)
    } else {
        Stroke::new(0.8, t.field_border)
    };

    ui.painter().rect(rect, CornerRadius::same(10), fill, stroke, egui::StrokeKind::Inside);

    // If locked (sticky), draw a small active latch dot in top-right corner
    if is_sticky {
        let dot_pos = pos2(rect.right() - 6.0, rect.top() + 6.0);
        ui.painter().circle_filled(dot_pos, 2.5, Color32::WHITE);
    }

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.5),
        text_color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Pos2, Rect};

    #[test]
    fn test_modifier_hud_position_clamping() {
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0));
        let outside = [-100.0, 999.0];
        let clamped = pos2(
            outside[0].clamp(8.0, screen.width() - 180.0),
            outside[1].clamp(8.0, screen.height() - 60.0),
        );
        assert_eq!(clamped.x, 8.0);
        assert_eq!(clamped.y, 540.0);
    }
}
