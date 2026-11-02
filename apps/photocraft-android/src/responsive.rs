//! Responsive layout and touch interaction adaptation for Android devices.
//!
//! Adapts the desktop-class SudaDesign interface to mobile form factors:
//! - **Safe Area & Notch Inset System**:
//!   - Lock-free atomic storage for `left`, `top`, `right`, `bottom`, `ime` insets reported by Android.
//!   - Protects top status bar, camera cutouts (punch-holes, notches), bottom navigation bar
//!     (gestures or 3-button), and soft keyboard (IME).
//! - **Phone Landscape & Portrait Adaptation**:
//!   - Collapses desktop status bar to maximize canvas height (+24 pt).
//!   - Double-column toolbar (`toolbar_double = true`) fitting all tools cleanly side by side.
//!   - Accordion single-expanded panel management in the dock: prevents panel crowding.
//!   - Compact dock width (230 pt) giving the canvas maximum screen width.
//! - **Touch Input Metrics across all Android form factors**:
//!   - Touch interact size: 36x36 pt (finger friendly, meeting mobile standards).
//!   - Button padding: 8x6 pt.
//!   - Icon sizing: 18 pt.
//!   - Touch-draggable scrollbars (8 pt floating width with solid opacity).

use std::sync::atomic::{AtomicU32, Ordering};

use egui::{Vec2, vec2};
use photocraft_ui_egui::dock::Group;
use photocraft_ui_egui::PhotocraftApp;

// ---------------------------------------------------------------------------
// Safe Area Insets (Lock-Free Atomics)
// ---------------------------------------------------------------------------

static INSET_LEFT: AtomicU32 = AtomicU32::new(0);
static INSET_TOP: AtomicU32 = AtomicU32::new(0);
static INSET_RIGHT: AtomicU32 = AtomicU32::new(0);
static INSET_BOTTOM: AtomicU32 = AtomicU32::new(0);
static INSET_IME: AtomicU32 = AtomicU32::new(0);

/// Safe area and IME insets reported by Android's WindowInsetsCompat (in logical points).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SafeAreaInsets {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub ime: f32,
}

impl SafeAreaInsets {
    /// Retrieve the currently active insets. Lock-free and instantaneous.
    pub fn current() -> Self {
        Self {
            left: f32::from_bits(INSET_LEFT.load(Ordering::Relaxed)),
            top: f32::from_bits(INSET_TOP.load(Ordering::Relaxed)),
            right: f32::from_bits(INSET_RIGHT.load(Ordering::Relaxed)),
            bottom: f32::from_bits(INSET_BOTTOM.load(Ordering::Relaxed)),
            ime: f32::from_bits(INSET_IME.load(Ordering::Relaxed)),
        }
    }

    /// Update the active insets. Called from JNI when Android dispatches WindowInsets.
    pub fn set(left: f32, top: f32, right: f32, bottom: f32, ime: f32) {
        INSET_LEFT.store(left.max(0.0).to_bits(), Ordering::Relaxed);
        INSET_TOP.store(top.max(0.0).to_bits(), Ordering::Relaxed);
        INSET_RIGHT.store(right.max(0.0).to_bits(), Ordering::Relaxed);
        INSET_BOTTOM.store(bottom.max(0.0).to_bits(), Ordering::Relaxed);
        INSET_IME.store(ime.max(0.0).to_bits(), Ordering::Relaxed);
    }
}

// ---------------------------------------------------------------------------
// Screen Profiles
// ---------------------------------------------------------------------------

/// Screen form-factor profile for Android devices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenProfile {
    /// Mobile smartphone in landscape orientation.
    PhoneLandscape,
    /// Mobile smartphone in portrait orientation.
    PhonePortrait,
    /// Android tablet in landscape orientation.
    TabletLandscape,
    /// Android tablet in portrait orientation.
    TabletPortrait,
}

impl ScreenProfile {
    /// Determines the screen profile from logical viewport dimensions.
    pub fn from_size(size: Vec2) -> Self {
        let is_portrait = size.y > size.x;
        if is_portrait {
            if size.x < 600.0 {
                ScreenProfile::PhonePortrait
            } else {
                ScreenProfile::TabletPortrait
            }
        } else if size.y < 500.0 || size.x < 900.0 {
            ScreenProfile::PhoneLandscape
        } else {
            ScreenProfile::TabletLandscape
        }
    }

    /// Whether this profile represents a compact phone layout.
    pub fn is_phone(self) -> bool {
        matches!(self, ScreenProfile::PhoneLandscape | ScreenProfile::PhonePortrait)
    }

    /// Whether the device is currently oriented in portrait mode.
    pub fn is_portrait(self) -> bool {
        matches!(self, ScreenProfile::PhonePortrait | ScreenProfile::TabletPortrait)
    }
}

/// Dynamic responsive layout state tracking.
#[derive(Debug, Default)]
pub struct ResponsiveState {
    pub current_profile: Option<ScreenProfile>,
    pub active_dock_group: Option<Group>,
    pub initialized: bool,
}

impl ResponsiveState {
    /// Adapts the PhotoCraft application layout and egui styling to the active screen profile.
    /// Runs strictly once on startup or when the form factor profile actually flips,
    /// guaranteeing zero feedback loops or visual jitter.
    pub fn adapt(&mut self, app: &mut PhotocraftApp, ctx: &egui::Context) {
        let screen_rect = ctx.input(|i| i.viewport().inner_rect).unwrap_or_else(|| ctx.content_rect());
        let screen_size = screen_rect.size();
        if screen_size.x <= 0.0 || screen_size.y <= 0.0 {
            return;
        }

        let profile = ScreenProfile::from_size(screen_size);
        let profile_changed = self.current_profile != Some(profile);

        if !self.initialized || profile_changed {
            self.apply_touch_metrics(ctx, profile);
            self.apply_profile_layout(app, ctx, profile);
            self.current_profile = Some(profile);
            self.initialized = true;
        }

        // Enforce accordion dock management on phones to prevent crushed panels
        if profile.is_phone() && app.ui.panels.dock {
            self.enforce_accordion_dock(app);
        }
    }

    /// Applies touch-optimized spacing, hit targets, and typography to egui.
    fn apply_touch_metrics(&self, ctx: &egui::Context, profile: ScreenProfile) {
        ctx.global_style_mut(|s| {
            if profile.is_phone() {
                s.spacing.interact_size = vec2(36.0, 36.0);
                s.spacing.button_padding = vec2(8.0, 6.0);
                s.spacing.icon_width = 18.0;
            } else {
                s.spacing.interact_size = vec2(36.0, 36.0);
                s.spacing.button_padding = vec2(8.0, 5.0);
                s.spacing.icon_width = 18.0;
            }

            // Finger-draggable scrollbar handles
            let mut scroll = egui::style::ScrollStyle::thin();
            scroll.floating_width = 8.0;
            scroll.interact_handle_opacity = 1.0;
            scroll.interact_background_opacity = 0.5;
            s.spacing.scroll = scroll;

            // Legible typography tuned for high-density mobile screens
            s.text_styles.insert(egui::TextStyle::Small, egui::FontId::proportional(11.0));
            s.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(12.5));
            s.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(12.5));
            s.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(15.0));
        });
    }

    /// Configures panels, options bar, and dock sizing for the active profile.
    fn apply_profile_layout(&mut self, app: &mut PhotocraftApp, ctx: &egui::Context, profile: ScreenProfile) {
        let screen_rect = ctx.input(|i| i.viewport().inner_rect).unwrap_or_else(|| ctx.content_rect());
        let screen_size = screen_rect.size();

        if profile.is_phone() {
            // Hide status bar on phones to reclaim vertical space
            app.ui.panels.status_bar = false;
            // Double-column toolbar so all tools fit side by side cleanly
            app.ui.panels.toolbar_double = true;
            // Enable iconic rail for fast switching
            app.ui.panels.rail = true;
            // Use narrow options bar
            app.session.prefs.edit(|p| {
                p.workspace.enable_narrow_options_bar = true;
            });
            // Request generous, touch-comfortable dock width for phone screens when summoned
            let touch_dock_w = (screen_size.x * 0.85).clamp(240.0, 340.0);
            photocraft_ui_egui::panels::request_dock_width(ctx, touch_dock_w);

            if profile == ScreenProfile::PhonePortrait {
                // In phone portrait, collapse dock groups by default so the canvas begins with 100% full screen width!
                app.ui.dock.set_collapsed(Group::Color, true);
                app.ui.dock.set_collapsed(Group::Properties, true);
                app.ui.dock.set_collapsed(Group::Layers, true);
                self.active_dock_group = None;
            } else {
                // In phone landscape, keep Layers open in accordion
                app.ui.dock.set_collapsed(Group::Color, true);
                app.ui.dock.set_collapsed(Group::Properties, true);
                app.ui.dock.set_collapsed(Group::Layers, false);
                self.active_dock_group = Some(Group::Layers);
            }
        } else {
            // Tablets and large screens have plenty of room; status bar active, full dock side-by-side
            app.ui.panels.status_bar = true;
            app.ui.panels.rail = true;
            app.ui.panels.toolbar_double = false;
            app.session.prefs.edit(|p| {
                p.workspace.enable_narrow_options_bar = false;
            });
            photocraft_ui_egui::panels::request_dock_width(ctx, 280.0);
            app.ui.dock.set_collapsed(Group::Color, false);
            app.ui.dock.set_collapsed(Group::Properties, false);
            app.ui.dock.set_collapsed(Group::Layers, false);
            self.active_dock_group = Some(Group::Layers);
        }
    }

    /// Single-active accordion dock policy for phones.
    ///
    /// When the user opens a group (e.g. Color), any previously active group (e.g. Layers)
    /// is automatically collapsed so only one group body is displayed at a time.
    /// This guarantees clean vertical space without crushed or overlapping cards.
    fn enforce_accordion_dock(&mut self, app: &mut PhotocraftApp) {
        let open_groups: Vec<Group> = Group::ALL
            .iter()
            .copied()
            .filter(|g| g.shown(&app.ui.panels) && !app.ui.dock.is_collapsed(*g))
            .collect();

        if open_groups.len() > 1 {
            // Find newly opened group that wasn't the active group
            let newly_opened = open_groups
                .iter()
                .copied()
                .find(|g| Some(*g) != self.active_dock_group);

            let chosen = newly_opened.unwrap_or(open_groups[0]);
            for g in open_groups {
                if g != chosen {
                    app.ui.dock.set_collapsed(g, true);
                }
            }
            self.active_dock_group = Some(chosen);
        } else if open_groups.len() == 1 {
            self.active_dock_group = Some(open_groups[0]);
        } else {
            self.active_dock_group = None;
        }
    }
}
