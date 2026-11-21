//! Suda Design Adaptive Keyboard Shortcut Accessory Bar (`KeyboardBar`).
//!
//! Provides a touch-first shortcut and modifier accessory strip attached right above
//! the Android virtual keyboard (IME).
//!
//! Features:
//! - **Adaptive Inset Tracking**: Anchors precisely above the native phone keyboard using `get_ime_inset`.
//! - **Modifier Quick Latches**: One-tap toggle for `Ctrl`/`Cmd`, `Alt`, `Shift` to pair with soft keyboard typing.
//! - **High-Frequency Actions**: Direct one-tap execution for Undo, Redo, Save (Ctrl+S), Cut, Copy, Paste, Deselect, Fit Screen.
//! - **Tool Quick Switchers**: Direct one-tap switching to Move, Brush, Eraser, Type.
//! - **One-Tap Dismiss**: Dedicated dismiss button that cleanly surrenders focus and hides the virtual keyboard.

use egui::{Color32, CornerRadius, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::PhotocraftApp;
use crate::Tool;
use crate::dialogs::{get_ime_inset, is_mobile_screen};
use crate::theme::Tokens;

/// Height of the accessory strip in logical points.
pub const KEYBOARD_BAR_HEIGHT: f32 = 42.0;

/// Identifier for the floating area.
const KEYBOARD_BAR_AREA_ID: &str = "suda_keyboard_accessory_strip";

/// Renders the adaptive keyboard shortcut accessory bar if the soft keyboard is open or requested.
pub fn show(app: &mut PhotocraftApp, ctx: &egui::Context) {
    if !is_mobile_screen(ctx) {
        return;
    }

    let ime_inset = get_ime_inset(ctx);
    let requested = app.ui.shell.soft_keyboard_requested;

    // Show only when the keyboard is physically visible on-screen (ime > 0)
    // or when the user has explicitly requested the keyboard.
    if ime_inset <= 0.0 && !requested {
        return;
    }

    let screen = ctx.content_rect();
    if screen.width() <= 0.0 || screen.height() <= 0.0 {
        return;
    }

    let insets = crate::safe_area::get_safe_area_insets(ctx);
    let safe = crate::safe_area::safe_rect(ctx);
    let t = Tokens::get(ctx);
    let bar_height = KEYBOARD_BAR_HEIGHT;

    // Position directly on top of the soft keyboard or safely above the system navigation bar
    let y = (safe.bottom() - bar_height).max(safe.top());
    let bar_rect = Rect::from_min_size(pos2(0.0, y), vec2(screen.width(), bar_height));

    let bg_fill = if t.dark() {
        Color32::from_rgba_unmultiplied(20, 20, 24, 245)
    } else {
        Color32::from_rgba_unmultiplied(240, 240, 244, 245)
    };

    let border_stroke = Stroke::new(
        1.0,
        if t.dark() {
            Color32::from_white_alpha(45)
        } else {
            Color32::from_black_alpha(40)
        },
    );

    let cmd_label = if cfg!(target_os = "macos") { "Cmd" } else { "Ctrl" };

    let pad_left = (8.0 + insets.left).round() as i8;
    let pad_right = (8.0 + insets.right).round() as i8;

    egui::Area::new(egui::Id::unique(KEYBOARD_BAR_AREA_ID))
        .order(egui::Order::Foreground)
        .fixed_pos(bar_rect.min)
        .show(ctx, |ui| {
            ui.set_width(bar_rect.width());
            ui.set_height(bar_height);

            egui::Frame::NONE
                .fill(bg_fill)
                .stroke(border_stroke)
                .inner_margin(egui::Margin {
                    left: pad_left,
                    right: pad_right,
                    top: 4,
                    bottom: 4,
                })
                .show(ui, |ui| {
                    egui::ScrollArea::horizontal()
                        .id_salt("keyboard_bar_scroll")
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                                // -------------------------------------------------------------
                                // 1. Modifier Chips (Tap-to-Lock / Latch)
                                // -------------------------------------------------------------
                                let ctrl_active = app.ui.shell.sticky_command || app.ui.shell.held_command;
                                if render_modifier_chip(ui, &t, cmd_label, ctrl_active).clicked() {
                                    app.ui.shell.sticky_command = !app.ui.shell.sticky_command;
                                }

                                let alt_active = app.ui.shell.sticky_alt || app.ui.shell.held_alt;
                                if render_modifier_chip(ui, &t, "Alt", alt_active).clicked() {
                                    app.ui.shell.sticky_alt = !app.ui.shell.sticky_alt;
                                }

                                let shift_active = app.ui.shell.sticky_shift || app.ui.shell.held_shift;
                                if render_modifier_chip(ui, &t, "Shift", shift_active).clicked() {
                                    app.ui.shell.sticky_shift = !app.ui.shell.sticky_shift;
                                }

                                render_strip_separator(ui, &t);

                                // -------------------------------------------------------------
                                // 2. High-Frequency Actions (Undo, Redo, Save, Deselect, Fit)
                                // -------------------------------------------------------------
                                let can_undo = app.session.is_enabled("edit.undo");
                                if render_action_chip(ui, &t, "↺ Undo", can_undo).clicked() && can_undo {
                                    let _ = app.run("edit.undo", json!({}));
                                }

                                let can_redo = app.session.is_enabled("edit.redo");
                                if render_action_chip(ui, &t, "↻ Redo", can_redo).clicked() && can_redo {
                                    let _ = app.run("edit.redo", json!({}));
                                }

                                let has_doc = app.session.active_index().is_some();
                                if render_action_chip(ui, &t, "💾 Save", has_doc).clicked() && has_doc {
                                    let _ = app.run("file.save", json!({}));
                                }

                                let can_deselect = app.session.is_enabled("select.deselect");
                                if render_action_chip(ui, &t, "▢ Deselect", can_deselect).clicked() && can_deselect {
                                    let _ = app.run("select.deselect", json!({}));
                                }

                                if render_action_chip(ui, &t, "⛶ Fit", has_doc).clicked() && has_doc {
                                    let _ = app.run("view.fitOnScreen", json!({}));
                                }

                                let can_copy = app.session.is_enabled("edit.copy");
                                if render_action_chip(ui, &t, "📋 Copy", can_copy).clicked() && can_copy {
                                    let _ = app.run("edit.copy", json!({}));
                                }

                                let can_paste = app.session.is_enabled("edit.paste");
                                if render_action_chip(ui, &t, "📋 Paste", can_paste).clicked() && can_paste {
                                    let _ = app.run("edit.paste", json!({}));
                                }

                                render_strip_separator(ui, &t);

                                // -------------------------------------------------------------
                                // 3. Quick Tool Selectors (Move, Brush, Eraser, Type)
                                // -------------------------------------------------------------
                                if render_tool_chip(ui, &t, "V Move", app.ui.tool == Tool::Move, has_doc).clicked() && has_doc {
                                    crate::tool_memory::select(app, Tool::Move);
                                }

                                if render_tool_chip(ui, &t, "B Brush", app.ui.tool == Tool::Brush, has_doc).clicked() && has_doc {
                                    crate::tool_memory::select(app, Tool::Brush);
                                }

                                if render_tool_chip(ui, &t, "E Eraser", app.ui.tool == Tool::Eraser, has_doc).clicked() && has_doc {
                                    crate::tool_memory::select(app, Tool::Eraser);
                                }

                                if render_tool_chip(ui, &t, "T Type", app.ui.tool == Tool::Type, has_doc).clicked() && has_doc {
                                    crate::tool_memory::select(app, Tool::Type);
                                }

                                render_strip_separator(ui, &t);

                                // -------------------------------------------------------------
                                // 4. Dismiss Keyboard Button
                                // -------------------------------------------------------------
                                if render_dismiss_chip(ui, &t).clicked() {
                                    app.ui.shell.soft_keyboard_requested = false;
                                    ui.ctx().memory_mut(|m| {
                                        if let Some(focused) = m.focused() {
                                            m.surrender_focus(focused);
                                        }
                                    });
                                    ui.ctx().request_repaint();
                                }
                            });
                        });
                });
        });
}

/// Helper to render a modifier toggle chip.
fn render_modifier_chip(
    ui: &mut egui::Ui,
    t: &Tokens,
    label: &str,
    active: bool,
) -> egui::Response {
    let height = 32.0;
    let text_len = label.chars().count();
    let width = (f32::from(text_len as u16) * 8.5 + 20.0).max(44.0);
    let (rect, resp) = ui.allocate_exact_size(vec2(width, height), Sense::click());

    let (bg, text_color) = if active {
        (t.accent, Color32::WHITE)
    } else if resp.hovered() {
        (t.hover, t.text)
    } else {
        (if t.dark() { Color32::from_white_alpha(20) } else { Color32::from_black_alpha(15) }, t.text)
    };

    ui.painter().rect_filled(rect, CornerRadius::same(10), bg);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.5),
        text_color,
    );

    resp
}

/// Helper to render an action chip.
fn render_action_chip(
    ui: &mut egui::Ui,
    t: &Tokens,
    label: &str,
    enabled: bool,
) -> egui::Response {
    let height = 32.0;
    let text_len = label.chars().count();
    let width = (f32::from(text_len as u16) * 8.0 + 16.0).max(40.0);
    let (rect, resp) = ui.allocate_exact_size(vec2(width, height), if enabled { Sense::click() } else { Sense::hover() });

    let bg = if !enabled {
        Color32::TRANSPARENT
    } else if resp.is_pointer_button_down_on() {
        t.accent.linear_multiply(0.4)
    } else if resp.hovered() {
        t.hover
    } else {
        if t.dark() { Color32::from_white_alpha(15) } else { Color32::from_black_alpha(10) }
    };

    let text_color = if !enabled {
        t.text_faint.linear_multiply(0.45)
    } else if resp.hovered() {
        t.text
    } else {
        t.text_dim
    };

    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CornerRadius::same(8), bg);
    }

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.0),
        text_color,
    );

    resp
}

/// Helper to render a tool selection chip.
fn render_tool_chip(
    ui: &mut egui::Ui,
    t: &Tokens,
    label: &str,
    active: bool,
    enabled: bool,
) -> egui::Response {
    let height = 32.0;
    let text_len = label.chars().count();
    let width = (f32::from(text_len as u16) * 8.0 + 18.0).max(44.0);
    let (rect, resp) = ui.allocate_exact_size(vec2(width, height), if enabled { Sense::click() } else { Sense::hover() });

    let (bg, text_color) = if active {
        (t.accent.linear_multiply(0.35), t.accent)
    } else if !enabled {
        (Color32::TRANSPARENT, t.text_faint.linear_multiply(0.4))
    } else if resp.hovered() {
        (t.hover, t.text)
    } else {
        (Color32::TRANSPARENT, t.text_dim)
    };

    if bg != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CornerRadius::same(8), bg);
    }

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(12.0),
        text_color,
    );

    resp
}

/// Helper to render the keyboard dismiss chip.
fn render_dismiss_chip(ui: &mut egui::Ui, t: &Tokens) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(60.0, 32.0), Sense::click());

    let (bg, text_color) = if resp.is_pointer_button_down_on() {
        (t.accent.linear_multiply(0.5), Color32::WHITE)
    } else if resp.hovered() {
        (t.hover, t.text)
    } else {
        (if t.dark() { Color32::from_white_alpha(25) } else { Color32::from_black_alpha(20) }, t.text)
    };

    ui.painter().rect_filled(rect, CornerRadius::same(10), bg);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "⌨ ✕",
        egui::FontId::proportional(12.5),
        text_color,
    );

    resp.on_hover_text("Dismiss Virtual Keyboard")
}

/// Small vertical separator between functional groups.
fn render_strip_separator(ui: &mut egui::Ui, t: &Tokens) {
    let (rect, _) = ui.allocate_exact_size(vec2(6.0, 24.0), Sense::hover());
    let x = rect.center().x;
    let stroke = Stroke::new(1.0, if t.dark() { Color32::from_white_alpha(35) } else { Color32::from_black_alpha(30) });
    ui.painter().line_segment([pos2(x, rect.top() + 4.0), pos2(x, rect.bottom() - 4.0)], stroke);
}

