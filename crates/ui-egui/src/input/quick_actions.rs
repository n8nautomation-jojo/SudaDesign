//! Touch-native Floating Smart Quick Actions HUD (`QuickActions`).
//!
//! Provides one-tap access to frequent repeated editing operations on touch
//! devices (Android phones, tablets, touchscreens, stylus):
//! - **Undo** (`edit.undo`)
//! - **Redo** (`edit.redo`)
//! - **Fit Screen** (`view.fitOnScreen`)
//! - **Deselect** (`select.deselect`)
//! - **Invert Selection** (`select.inverse`)
//! - **Duplicate Layer** (`layer.duplicate`)
//! - **Color Eyedropper** (`Tool::Eyedropper`)
//!
//! Features:
//! - **Draggable**: Drag handle allows freely moving the HUD anywhere on screen.
//! - **Minimizable**: Collapses into a small ⚡ floating badge to avoid obscuring canvas art.
//! - **State-Aware**: Disables actions when not applicable (e.g. no document, nothing to undo).
//! - **Touch-Optimized**: 36pt minimum hit targets meeting mobile ergonomic standards.

use egui::{Color32, CornerRadius, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::PhotocraftApp;
use crate::Tool;
use crate::theme::Tokens;

/// Area ID for egui persistence and positioning.
const QUICK_ACTIONS_AREA_ID: &str = "suda_floating_quick_actions";

/// Displays the Smart Quick Actions HUD.
pub fn show(app: &mut PhotocraftApp, ctx: &egui::Context) {
    let screen = ctx.content_rect();
    if screen.width() <= 0.0 || screen.height() <= 0.0 {
        return;
    }

    let is_mobile = crate::dialogs::is_mobile_screen(ctx);
    // On mobile viewports, show by default unless explicitly hidden.
    let visible = if app.ui.shell.quick_actions_visible {
        true
    } else if is_mobile && !app.ui.shell.quick_actions_explicitly_hidden {
        true
    } else {
        false
    };

    if !visible {
        return;
    }

    let t = Tokens::get(ctx);
    let safe = crate::safe_area::safe_rect(ctx);
    let is_minimized = app.ui.shell.quick_actions_minimized;

    // Default position: floating safely below top cutout/notch on mobile, or upper right of canvas
    let default_pos = if is_mobile {
        pos2((safe.width() - 320.0).max(16.0) * 0.5 + safe.left(), safe.top() + 8.0)
    } else {
        pos2((safe.width() - 420.0).max(80.0) + safe.left(), safe.top() + 14.0)
    };

    let hud_pos = app
        .ui.shell
        .quick_actions_pos
        .map(|[x, y]| pos2(x.clamp(safe.left() + 8.0, (safe.right() - 60.0).max(safe.left() + 8.0)), y.clamp(safe.top() + 8.0, (safe.bottom() - 50.0).max(safe.top() + 8.0))))
        .unwrap_or(default_pos);

    let mut new_pos = hud_pos;
    let mut update_pos = false;
    let mut toggle_minimize = false;

    let bg_fill = if t.dark() {
        Color32::from_rgba_unmultiplied(22, 22, 26, 230)
    } else {
        Color32::from_rgba_unmultiplied(238, 238, 242, 230)
    };

    let border_stroke = Stroke::new(
        1.0,
        if t.dark() {
            Color32::from_white_alpha(45)
        } else {
            Color32::from_black_alpha(45)
        },
    );

    egui::Area::new(egui::Id::unique(QUICK_ACTIONS_AREA_ID))
        .order(egui::Order::Foreground)
        .fixed_pos(hud_pos)
        .show(ctx, |ui| {
            if is_minimized {
                // Minimized pill mode: small unobtrusive floating trigger badge
                egui::Frame::NONE
                    .fill(bg_fill)
                    .stroke(border_stroke)
                    .corner_radius(CornerRadius::same(18))
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                            // Drag Handle
                            let (handle_rect, handle_resp) = ui.allocate_exact_size(vec2(14.0, 28.0), Sense::drag());
                            ui.painter().text(
                                handle_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "⋮",
                                egui::FontId::proportional(12.0),
                                t.text_faint,
                            );
                            if handle_resp.dragged() {
                                new_pos += handle_resp.drag_delta();
                                update_pos = true;
                            }

                            // Expand Button
                            let (btn_rect, btn_resp) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::click());
                            let icon_color = if btn_resp.hovered() { t.text } else { t.accent };
                            ui.painter().text(
                                btn_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "⚡",
                                egui::FontId::proportional(15.0),
                                icon_color,
                            );
                            if btn_resp.clicked() {
                                toggle_minimize = true;
                            }
                        });
                    });
            } else {
                // Expanded toolbar mode
                egui::Frame::NONE
                    .fill(bg_fill)
                    .stroke(border_stroke)
                    .corner_radius(CornerRadius::same(18))
                    .inner_margin(egui::Margin::symmetric(6, 4))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(3.0, 0.0);

                            // 1. Drag Handle
                            let (handle_rect, handle_resp) = ui.allocate_exact_size(vec2(14.0, 32.0), Sense::drag());
                            let handle_color = if handle_resp.hovered() || handle_resp.dragged() {
                                t.text
                            } else {
                                t.text_faint
                            };
                            ui.painter().text(
                                handle_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "⋮⋮",
                                egui::FontId::proportional(12.0),
                                handle_color,
                            );
                            if handle_resp.dragged() {
                                new_pos += handle_resp.drag_delta();
                                update_pos = true;
                            }

                            let has_doc = app.session.active_index().is_some();

                            // 2. Undo
                            let can_undo = app.session.is_enabled("edit.undo");
                            if render_action_button(ui, &t, "↺", "Undo", can_undo).clicked() && can_undo {
                                let _ = app.run("edit.undo", json!({}));
                            }

                            // 3. Redo
                            let can_redo = app.session.is_enabled("edit.redo");
                            if render_action_button(ui, &t, "↻", "Redo", can_redo).clicked() && can_redo {
                                let _ = app.run("edit.redo", json!({}));
                            }

                            ui.separator();

                            // 4. Fit Screen
                            if render_action_button(ui, &t, "⛶", "Fit Screen", has_doc).clicked() && has_doc {
                                let _ = app.run("view.fitOnScreen", json!({}));
                            }

                            // 5. Deselect
                            let can_deselect = app.session.is_enabled("select.deselect");
                            if render_action_button(ui, &t, "▢", "Deselect", can_deselect).clicked() && can_deselect {
                                let _ = app.run("select.deselect", json!({}));
                            }

                            // 6. Invert Selection
                            let can_invert = app.session.is_enabled("select.inverse");
                            if render_action_button(ui, &t, "◐", "Invert", can_invert).clicked() && can_invert {
                                let _ = app.run("select.inverse", json!({}));
                            }

                            // 7. Duplicate Layer
                            let can_dup = app.session.is_enabled("layer.duplicate");
                            if render_action_button(ui, &t, "⧉", "Duplicate", can_dup).clicked() && can_dup {
                                let _ = app.run("layer.duplicate", json!({}));
                            }

                            // 8. Eyedropper / Color Sampler
                            let is_eyedropper = app.ui.tool == Tool::Eyedropper;
                            if render_tool_button(ui, &t, "💧", "Sampler", has_doc, is_eyedropper).clicked() && has_doc {
                                crate::tool_memory::select(app, Tool::Eyedropper);
                            }

                            // 9. Soft Keyboard toggle (Shortcuts / Virtual Keyboard)
                            let kb_active = app.ui.shell.soft_keyboard_requested;
                            if render_tool_button(ui, &t, "⌨", "Virtual Keyboard (Shortcuts)", true, kb_active).clicked() {
                                app.ui.shell.soft_keyboard_requested = !app.ui.shell.soft_keyboard_requested;
                            }

                            // 10. Minimize toggle button
                            let (min_rect, min_resp) = ui.allocate_exact_size(vec2(22.0, 32.0), Sense::click());
                            let min_color = if min_resp.hovered() { t.text } else { t.text_faint };
                            ui.painter().text(
                                min_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "◀",
                                egui::FontId::proportional(11.0),
                                min_color,
                            );
                            if min_resp.clicked() {
                                toggle_minimize = true;
                            }
                        });
                    });
            }
        });

    if update_pos {
        let clamped_x = new_pos.x.clamp(safe.left() + 8.0, (safe.right() - 60.0).max(safe.left() + 8.0));
        let clamped_y = new_pos.y.clamp(safe.top() + 8.0, (safe.bottom() - 50.0).max(safe.top() + 8.0));
        app.ui.shell.quick_actions_pos = Some([clamped_x, clamped_y]);
    }
    if toggle_minimize {
        app.ui.shell.quick_actions_minimized = !is_minimized;
    }
}

/// Helper to render a touch-friendly action button with tooltip.
fn render_action_button(
    ui: &mut egui::Ui,
    t: &Tokens,
    symbol: &str,
    tooltip: &str,
    enabled: bool,
) -> egui::Response {
    let size = vec2(32.0, 32.0);
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });

    let bg = if !enabled {
        Color32::TRANSPARENT
    } else if resp.is_pointer_button_down_on() {
        t.accent.linear_multiply(0.4)
    } else if resp.hovered() {
        if t.dark() { Color32::from_white_alpha(35) } else { Color32::from_black_alpha(25) }
    } else {
        Color32::TRANSPARENT
    };

    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CornerRadius::same(12), bg);
    }

    let text_color = if !enabled {
        t.text_faint.linear_multiply(0.4)
    } else if resp.hovered() {
        t.text
    } else {
        t.text_faint
    };

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        symbol,
        egui::FontId::proportional(14.0),
        text_color,
    );

    resp.on_hover_text(tooltip)
}

/// Helper to render a tool toggle button with active indicator.
fn render_tool_button(
    ui: &mut egui::Ui,
    t: &Tokens,
    symbol: &str,
    tooltip: &str,
    enabled: bool,
    active: bool,
) -> egui::Response {
    let size = vec2(32.0, 32.0);
    let (rect, resp) = ui.allocate_exact_size(size, if enabled { Sense::click() } else { Sense::hover() });

    let bg = if active {
        t.accent.linear_multiply(0.45)
    } else if !enabled {
        Color32::TRANSPARENT
    } else if resp.hovered() {
        if t.dark() { Color32::from_white_alpha(35) } else { Color32::from_black_alpha(25) }
    } else {
        Color32::TRANSPARENT
    };

    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CornerRadius::same(12), bg);
    }

    let text_color = if active {
        t.accent
    } else if !enabled {
        t.text_faint.linear_multiply(0.4)
    } else {
        t.text_faint
    };

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        symbol,
        egui::FontId::proportional(14.0),
        text_color,
    );

    resp.on_hover_text(tooltip)
}
