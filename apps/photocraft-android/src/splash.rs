//! SudaDesign animated splash screen — drawn over the app on first launch.
//!
//! Mimics the Photoshop-style splash: dark background fades in, the logo appears
//! with a scale + opacity spring animation, a progress stripe sweeps across the
//! bottom, and the whole screen fades out before handing control to the main app.
//!
//! Architecture: pure egui painting, no platform code, no unsafe, no unwrap.

use std::time::{Duration, Instant};

/// Total phases (seconds):
///   0.00 – 0.35  background fade-in + logo scale-spring
///   0.35 – 0.85  logo hold + progress bar begins
///   0.85 – 1.25  tagline fade-in
///   1.25 – 1.85  full brand display (tagline and logo held clearly)
///   1.85 – 2.20  smooth cinematic fade-out
///   2.20+        done (invisible, inner app takes over)
const PHASE_LOGO_IN_END: f32 = 0.35;
const PHASE_HOLD_END: f32 = 0.85;
const PHASE_TAG_IN_END: f32 = 1.25;
const PHASE_HOLD_ALL_END: f32 = 1.85;
const PHASE_FADE_OUT_END: f32 = 2.20;

/// Logo PNG bytes embedded at compile time.
static LOGO_BYTES: &[u8] = include_bytes!("sudadesign_logo.png");

/// Splash screen state. Cheaply `Clone`-free; kept in `AndroidPhotocraftApp`.
pub struct SplashState {
    start: Instant,
    logo_texture: Option<egui::TextureHandle>,
}

impl SplashState {
    /// Create the splash state and pre-load the logo texture into egui's texture manager.
    pub fn new(ctx: &egui::Context) -> Self {
        let logo_texture = load_logo(ctx);
        Self { start: Instant::now(), logo_texture }
    }

    /// Returns `true` once the entire splash animation has finished.
    pub fn is_done(&self) -> bool {
        self.elapsed() >= PHASE_FADE_OUT_END
    }

    /// Draw the splash over the full `ui` rect. Returns `true` while the splash
    /// is still the active screen (caller should skip inner-app drawing), `false`
    /// when splash is completely finished.
    pub fn draw(&mut self, ui: &mut egui::Ui) -> bool {
        let t = self.elapsed();
        if t >= PHASE_FADE_OUT_END {
            self.logo_texture = None;
            return false;
        }

        // Global alpha: full during logo-in / hold / tagline phases, then fade-out.
        let global_alpha = if t < PHASE_HOLD_ALL_END {
            1.0_f32
        } else {
            let fade = (t - PHASE_HOLD_ALL_END) / (PHASE_FADE_OUT_END - PHASE_HOLD_ALL_END);
            1.0 - fade.clamp(0.0, 1.0)
        };

        let rect = ui.max_rect();
        let painter = ui.painter_at(rect);

        // Background — very dark charcoal (#12131e) matching SudaDesign's deep dark theme.
        let bg_alpha = if t < 0.10 {
            (t / 0.10).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let bg = egui::Color32::from_rgba_unmultiplied(18, 19, 30, (bg_alpha * global_alpha * 255.0) as u8);
        painter.rect_filled(rect, 0.0, bg);

        // Logo spring-scale animation: 0.60 -> 1.00 with smooth ease-out spring
        let logo_alpha = if t < PHASE_LOGO_IN_END {
            (t / PHASE_LOGO_IN_END).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let logo_scale = if t < PHASE_LOGO_IN_END {
            let p = (t / PHASE_LOGO_IN_END).clamp(0.0, 1.0);
            let spring = 1.0 - (1.0 - p).powi(3) * (1.0 + 2.5 * (1.0 - p));
            0.60 + 0.40 * spring.clamp(0.0, 1.1)
        } else {
            1.0_f32
        };

        // Screen-adaptive balanced layout:
        // Automatically prevents any overlap between Logo, Title, Tagline, and Progress Bar.
        let screen_h = rect.height();
        let is_compact = screen_h < 480.0;

        let (logo_max_dim, name_font_size, name_gap, tag_font_size, tag_gap, bar_bottom_offset) = if is_compact {
            (screen_h * 0.36, 22.0_f32, 8.0_f32, 10.5_f32, 6.0_f32, 20.0_f32)
        } else {
            (screen_h * 0.34, 26.0_f32, 14.0_f32, 12.0_f32, 8.0_f32, 32.0_f32)
        };

        let logo_size = (rect.width() * 0.45).min(logo_max_dim) * logo_scale;
        let bar_y = rect.bottom() - bar_bottom_offset;

        // Vertically center the entire content block in the space between top and the progress bar
        let total_block_h = logo_size + name_gap + name_font_size + tag_gap + tag_font_size;
        let content_center_y = (rect.top() + (bar_y - 20.0)) * 0.5;
        let block_top = (content_center_y - total_block_h * 0.5).max(rect.top() + 10.0);

        let logo_center = egui::pos2(rect.center().x, block_top + logo_size * 0.5);
        let logo_rect = egui::Rect::from_center_size(logo_center, egui::vec2(logo_size, logo_size));

        if let Some(tex) = &self.logo_texture {
            let effective_alpha = (logo_alpha * global_alpha).clamp(0.0, 1.0);
            let tint = egui::Color32::from_rgba_unmultiplied(255, 255, 255, (effective_alpha * 255.0) as u8);
            painter.image(tex.id(), logo_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), tint);
        }

        // App name: SudaDesign
        let name_y = logo_rect.bottom() + name_gap;
        let name_alpha = (logo_alpha * global_alpha * 255.0) as u8;
        let name_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, name_alpha);
        painter.text(
            egui::pos2(rect.center().x, name_y),
            egui::Align2::CENTER_TOP,
            "SudaDesign",
            egui::FontId::proportional(name_font_size),
            name_color,
        );

        // Slogan / Tagline: CREATE  •  EDIT  •  INSPIRE
        // Placed with guaranteed margin below the app name and above the progress bar
        let tag_y = name_y + name_font_size + tag_gap;
        if t > PHASE_HOLD_END {
            let tag_alpha_f = ((t - PHASE_HOLD_END) / (PHASE_TAG_IN_END - PHASE_HOLD_END)).clamp(0.0, 1.0);
            let tag_alpha = (tag_alpha_f * global_alpha * 215.0) as u8;
            let tag_color = egui::Color32::from_rgba_unmultiplied(160, 200, 240, tag_alpha);
            painter.text(
                egui::pos2(rect.center().x, tag_y),
                egui::Align2::CENTER_TOP,
                "CREATE   •   EDIT   •   INSPIRE",
                egui::FontId::proportional(tag_font_size),
                tag_color,
            );
        }

        // Progress bar — smooth rounded pill
        let progress = if t < PHASE_LOGO_IN_END {
            0.0_f32
        } else if t < PHASE_HOLD_ALL_END {
            ((t - PHASE_LOGO_IN_END) / (PHASE_HOLD_ALL_END - PHASE_LOGO_IN_END)).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let bar_h = 3.0_f32;
        let bar_total_w = (rect.width() * 0.48).min(280.0);
        let bar_x = rect.center().x - bar_total_w / 2.0;

        // Background track (dim translucent)
        let track_alpha = (logo_alpha * global_alpha * 0.22 * 255.0) as u8;
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::vec2(bar_total_w, bar_h)),
            bar_h / 2.0,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, track_alpha),
        );

        // Active fill (SudaDesign Cyan accent)
        let fill_w = bar_total_w * progress;
        if fill_w > 0.0 {
            let fill_alpha = (global_alpha * 255.0) as u8;
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::vec2(fill_w, bar_h)),
                bar_h / 2.0,
                egui::Color32::from_rgba_unmultiplied(0, 210, 255, fill_alpha),
            );
        }

        // Keep repainting at 60 fps until we're done.
        ui.ctx().request_repaint_after(Duration::from_millis(16));
        true
    }

    fn elapsed(&self) -> f32 {
        self.start.elapsed().as_secs_f32()
    }
}

/// Load the embedded logo PNG as an egui texture. Returns `None` on decode failure
/// (paint will just skip the logo gracefully).
fn load_logo(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    // Decode PNG to raw RGBA8 pixels using the `image` crate already in the dependency tree.
    // We use a minimal approach: rely on `egui_extras` or direct `image` decode.
    // Since `photocraft-codecs` is not available here (wrong layer), we decode with a
    // lightweight embedded decoder. The `png` crate is already a transitive dependency.
    let logo_image = decode_png_to_egui_image(LOGO_BYTES)?;
    Some(ctx.load_texture("sudadesign_logo", logo_image, egui::TextureOptions::LINEAR))
}

/// Decode PNG bytes to an [`egui::ColorImage`]. Uses the `png` crate which is already
/// present in the dependency graph via `photocraft-codecs` → `png`.
fn decode_png_to_egui_image(bytes: &[u8]) -> Option<egui::ColorImage> {
    use std::io::Cursor;
    let cursor = Cursor::new(bytes);
    let decoder = png::Decoder::new(cursor);
    let mut reader = decoder.read_info().ok()?;
    let buf_size = reader.output_buffer_size()?;
    let mut buf = vec![0u8; buf_size];
    let info = reader.next_frame(&mut buf).ok()?;
    let width = info.width as usize;
    let height = info.height as usize;
    let bytes_used = info.buffer_size();
    let raw = &buf[..bytes_used];

    // Convert to RGBA8.
    let pixels: Vec<egui::Color32> = match info.color_type {
        png::ColorType::Rgba => raw
            .chunks_exact(4)
            .map(|c| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
            .collect(),
        png::ColorType::Rgb => raw
            .chunks_exact(3)
            .map(|c| egui::Color32::from_rgb(c[0], c[1], c[2]))
            .collect(),
        _ => return None,
    };

    if pixels.len() != width * height {
        return None;
    }
    Some(egui::ColorImage::new([width, height], pixels))
}

