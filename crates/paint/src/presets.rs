//! Built-in Suda Design Signature brush presets.
//!
//! Procedurally generated sampled tips, textures and mathematical dynamics designed clean-room
//! specifically for Suda Design. Every preset is plain [`BrushSettings`] data, serialises to
//! JSON, journals, and can be edited or saved like any user preset.
//!
//! Organized in 5 professional categories:
//! 1. Inking & Calligraphy
//! 2. Pencils & Sketching
//! 3. Oils & Acrylics
//! 4. Watercolors
//! 5. Textures & Effects

use crate::brush::{
    BrushPreset, BrushSettings, ColorDynamics, Control, Dynamic, MaskMode, Pattern, PatternStyle, Scattering, ShapeDynamics, Smoothing,
    Texture, TipShape, Transfer,
};
use crate::mixer::MixerSettings;
use crate::procedural::{bristle_tip, chalk_tip, charcoal_tip, spatter_tip, sponge_tip};

/// Suda Design Signature preset categories, in Brushes panel order.
pub const GROUPS: [&str; 5] = [
    "Inking & Calligraphy",
    "Pencils & Sketching",
    "Oils & Acrylics",
    "Watercolors",
    "Textures & Effects",
];

fn preset(id: &str, group: &str, name: &str, brush: BrushSettings) -> BrushPreset {
    BrushPreset {
        id: Some(id.to_string()),
        name: name.to_string(),
        brush,
        builtin: true,
        group: group.to_string(),
        folder: Vec::new(),
        captures: Default::default(),
    }
}

fn pressure() -> Dynamic {
    Dynamic::controlled(Control::PenPressure)
}

fn pressure_min(minimum: f32) -> Dynamic {
    Dynamic { minimum, ..pressure() }
}

fn size_dyn(size: Dynamic) -> ShapeDynamics {
    ShapeDynamics { enabled: true, size, ..Default::default() }
}

fn transfer(opacity: Dynamic, flow: Dynamic) -> Transfer {
    Transfer { enabled: true, opacity, flow, ..Default::default() }
}

fn paper(seed: u32, depth: f32) -> Texture {
    Texture { enabled: true, pattern: Pattern::Procedural { style: PatternStyle::Paper, size: 128, seed }, depth, ..Default::default() }
}

fn canvas(seed: u32, depth: f32) -> Texture {
    Texture { enabled: true, pattern: Pattern::Procedural { style: PatternStyle::Canvas, size: 64, seed }, depth, ..Default::default() }
}

fn inking_and_calligraphy(base: &BrushSettings) -> Vec<BrushPreset> {
    let p = |id: &str, name: &str, b: BrushSettings| preset(id, "Inking & Calligraphy", name, b);
    vec![
        // 1. Fine Ink: Sharp, smooth monoline with subtle pressure sizing and smoothing
        p(
            "suda.inking.fine_ink",
            "Fine Ink",
            BrushSettings {
                size: 6.0,
                hardness: 1.0,
                spacing: 0.03,
                shape_dynamics: size_dyn(pressure_min(0.25)),
                smoothing: Smoothing { enabled: true, amount: 0.35, ..Default::default() },
                ..base.clone()
            },
        ),
        // 2. Technical Pen: Crisp, uniform line weight with high precision and steady flow
        p(
            "suda.inking.technical_pen",
            "Technical Pen",
            BrushSettings {
                size: 3.5,
                hardness: 1.0,
                spacing: 0.04,
                pressure_size: false,
                pressure_opacity: false,
                flow: 1.0,
                smoothing: Smoothing { enabled: true, amount: 0.2, ..Default::default() },
                ..base.clone()
            },
        ),
        // 3. Expressive Ink: Sumi-e style dynamic brush with dramatic line weight taper
        p(
            "suda.inking.expressive_ink",
            "Expressive Ink",
            BrushSettings {
                size: 24.0,
                hardness: 0.9,
                spacing: 0.04,
                shape_dynamics: size_dyn(pressure_min(0.08)),
                transfer: transfer(Dynamic::default(), pressure_min(0.4)),
                smoothing: Smoothing { enabled: true, amount: 0.4, ..Default::default() },
                ..base.clone()
            },
        ),
        // 4. Arabic Calligraphy: Slanted chisel nib (60°), authentic Qalam thick/thin transitions, pressure flow & smoothing
        p(
            "suda.calligraphy.arabic",
            "Arabic Calligraphy",
            BrushSettings {
                size: 32.0,
                hardness: 0.95,
                roundness: 0.18,
                angle: 60.0,
                spacing: 0.035,
                shape_dynamics: size_dyn(pressure_min(0.35)),
                transfer: transfer(Dynamic::default(), pressure_min(0.6)),
                smoothing: Smoothing { enabled: true, amount: 0.35, ..Default::default() },
                ..base.clone()
            },
        ),
        // 5. Chisel Calligraphy: Broad Western/Latin chisel nib (45°), crisp formal Gothic/Italic lettering
        p(
            "suda.calligraphy.chisel",
            "Chisel Calligraphy",
            BrushSettings {
                size: 28.0,
                hardness: 1.0,
                roundness: 0.22,
                angle: 45.0,
                spacing: 0.035,
                shape_dynamics: size_dyn(pressure_min(0.45)),
                smoothing: Smoothing { enabled: true, amount: 0.25, ..Default::default() },
                ..base.clone()
            },
        ),
        // 6. Signature Pen: Fast, responsive fountain/gel pen for natural signatures and notes
        p(
            "suda.inking.signature_pen",
            "Signature Pen",
            BrushSettings {
                size: 5.0,
                hardness: 0.85,
                spacing: 0.04,
                shape_dynamics: size_dyn(pressure_min(0.3)),
                transfer: transfer(pressure_min(0.5), Dynamic::default()),
                smoothing: Smoothing { enabled: true, amount: 0.25, ..Default::default() },
                ..base.clone()
            },
        ),
    ]
}

fn pencils_and_sketching(base: &BrushSettings) -> Vec<BrushPreset> {
    let p = |id: &str, name: &str, b: BrushSettings| preset(id, "Pencils & Sketching", name, b);
    vec![
        // 1. Hard Pencil: 2H light drafting pencil with subtle paper tooth
        p(
            "suda.pencil.hard",
            "Hard Pencil",
            BrushSettings {
                size: 2.5,
                hardness: 1.0,
                spacing: 0.06,
                shape_dynamics: size_dyn(pressure_min(0.5)),
                transfer: transfer(pressure_min(0.2), pressure_min(0.3)),
                texture: Texture { mode: MaskMode::Subtract, invert: true, scale: 0.5, ..paper(11, 0.4) },
                ..base.clone()
            },
        ),
        // 2. Soft Pencil: 4B/6B artist pencil with deep graphite shading and paper tooth
        p(
            "suda.pencil.soft",
            "Soft Pencil",
            BrushSettings {
                size: 7.0,
                hardness: 0.55,
                spacing: 0.06,
                shape_dynamics: size_dyn(pressure_min(0.35)),
                transfer: transfer(pressure_min(0.15), Dynamic::default()),
                texture: Texture { mode: MaskMode::Subtract, invert: true, scale: 0.75, ..paper(21, 0.75) },
                ..base.clone()
            },
        ),
        // 3. Mechanical Pencil: 0.5mm consistent drafting lead for precision sketches
        p(
            "suda.pencil.mechanical",
            "Mechanical Pencil",
            BrushSettings {
                size: 2.0,
                hardness: 0.95,
                spacing: 0.05,
                shape_dynamics: size_dyn(pressure_min(0.7)),
                transfer: transfer(pressure_min(0.4), Dynamic::default()),
                smoothing: Smoothing { enabled: true, amount: 0.15, ..Default::default() },
                ..base.clone()
            },
        ),
        // 4. Graphite: Broad graphite stick for tonal blocking and hatching
        p(
            "suda.pencil.graphite",
            "Graphite",
            BrushSettings {
                size: 32.0,
                hardness: 0.4,
                roundness: 0.45,
                angle: 30.0,
                spacing: 0.08,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    size: pressure_min(0.4),
                    angle: Dynamic::controlled(Control::Direction),
                    ..Default::default()
                },
                transfer: transfer(pressure_min(0.25), Dynamic::default()),
                texture: Texture { mode: MaskMode::Subtract, invert: true, scale: 1.0, ..paper(35, 0.85) },
                ..base.clone()
            },
        ),
        // 5. Charcoal: Gritty vine charcoal with procedural broken streaks
        p(
            "suda.pencil.charcoal",
            "Charcoal",
            BrushSettings {
                size: 45.0,
                tip: TipShape::Sampled(charcoal_tip(64, 12)),
                spacing: 0.12,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    size: pressure_min(0.45),
                    angle: Dynamic::controlled(Control::Direction),
                    ..Default::default()
                },
                transfer: transfer(pressure_min(0.25), Dynamic::default()),
                texture: Texture { mode: MaskMode::Subtract, invert: true, ..paper(5, 0.7) },
                ..base.clone()
            },
        ),
        // 6. Textured Sketch: Expressive chalk/pastel with scattered grain
        p(
            "suda.pencil.textured_sketch",
            "Textured Sketch",
            BrushSettings {
                size: 36.0,
                tip: TipShape::Sampled(chalk_tip(64, 25)),
                spacing: 0.14,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    angle: Dynamic::jitter(1.0),
                    size: Dynamic { jitter: 0.15, minimum: 0.3, ..pressure() },
                    roundness: Dynamic { jitter: 0.25, minimum: 0.5, ..Default::default() },
                    ..Default::default()
                },
                scattering: Scattering { enabled: true, scatter: Dynamic::jitter(0.18), both_axes: true, ..Default::default() },
                texture: Texture { mode: MaskMode::Subtract, invert: true, ..canvas(2, 0.55) },
                ..base.clone()
            },
        ),
    ]
}

fn oils_and_acrylics(base: &BrushSettings) -> Vec<BrushPreset> {
    let p = |id: &str, name: &str, b: BrushSettings| preset(id, "Oils & Acrylics", name, b);
    vec![
        // 1. Round Oil: Rich body, buttery blending with mixer settings
        p(
            "suda.oil.round",
            "Round Oil",
            BrushSettings {
                size: 55.0,
                hardness: 0.75,
                spacing: 0.05,
                shape_dynamics: size_dyn(pressure_min(0.35)),
                mixer: MixerSettings { wet: 0.4, mix: 0.5, load: 0.7, flow: 1.0, ..Default::default() },
                texture: canvas(4, 0.25),
                ..base.clone()
            },
        ),
        // 2. Flat Oil: Palette knife / chisel oil stroke for blocking and impasto edges
        p(
            "suda.oil.flat",
            "Flat Oil",
            BrushSettings {
                size: 48.0,
                hardness: 0.85,
                roundness: 0.25,
                angle: 0.0,
                spacing: 0.04,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    size: pressure_min(0.4),
                    angle: Dynamic::controlled(Control::Direction),
                    ..Default::default()
                },
                mixer: MixerSettings { wet: 0.3, mix: 0.4, load: 0.8, flow: 1.0, ..Default::default() },
                ..base.clone()
            },
        ),
        // 3. Bristle Brush: Natural hog bristle with procedural streak filaments
        p(
            "suda.oil.bristle",
            "Bristle Brush",
            BrushSettings {
                size: 60.0,
                tip: TipShape::Sampled(bristle_tip(64, 33)),
                spacing: 0.035,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    size: pressure_min(0.5),
                    angle: Dynamic::controlled(Control::Direction),
                    ..Default::default()
                },
                color_dynamics: ColorDynamics { enabled: true, per_tip: false, fg_bg: Dynamic::jitter(0.12), brightness_jitter: 0.06, ..Default::default() },
                texture: canvas(5, 0.3),
                ..base.clone()
            },
        ),
        // 4. Thick Paint: Heavy body paint with impasto height relief
        p(
            "suda.oil.thick_paint",
            "Thick Paint",
            BrushSettings {
                size: 65.0,
                hardness: 0.9,
                spacing: 0.04,
                shape_dynamics: size_dyn(pressure_min(0.5)),
                texture: Texture { mode: MaskMode::LinearHeight, depth: 0.7, ..paper(44, 0.7) },
                mixer: MixerSettings { wet: 0.2, mix: 0.3, load: 0.95, flow: 1.0, ..Default::default() },
                ..base.clone()
            },
        ),
        // 5. Acrylic Detail: Crisp synthetic filbert acrylic brush for sharp lines and highlights
        p(
            "suda.oil.acrylic_detail",
            "Acrylic Detail",
            BrushSettings {
                size: 14.0,
                hardness: 0.85,
                roundness: 0.65,
                spacing: 0.04,
                shape_dynamics: size_dyn(pressure_min(0.2)),
                smoothing: Smoothing { enabled: true, amount: 0.25, ..Default::default() },
                ..base.clone()
            },
        ),
        // 6. Color Mixer: Dedicated wet blending brush for smudging and pigment mixing
        p(
            "suda.oil.color_mixer",
            "Color Mixer",
            BrushSettings {
                size: 50.0,
                hardness: 0.4,
                spacing: 0.05,
                shape_dynamics: size_dyn(pressure_min(0.3)),
                mixer: MixerSettings { wet: 0.85, mix: 0.8, load: 0.05, flow: 0.8, ..Default::default() },
                ..base.clone()
            },
        ),
    ]
}

fn watercolors(base: &BrushSettings) -> Vec<BrushPreset> {
    let p = |id: &str, name: &str, b: BrushSettings| preset(id, "Watercolors", name, b);
    vec![
        // 1. Transparent Wash: Broad, fluid watercolor wash for translucent layering
        p(
            "suda.watercolor.transparent_wash",
            "Transparent Wash",
            BrushSettings {
                size: 120.0,
                hardness: 0.0,
                flow: 0.25,
                spacing: 0.1,
                wet_edges: true,
                shape_dynamics: size_dyn(pressure_min(0.4)),
                texture: Texture { scale: 1.5, ..paper(13, 0.3) }.with_pattern_size(256),
                ..base.clone()
            },
        ),
        // 2. Soft Watercolor: Blooming wet-on-wet watercolor bleeding softly into paper
        p(
            "suda.watercolor.soft",
            "Soft Watercolor",
            BrushSettings {
                size: 70.0,
                hardness: 0.15,
                flow: 0.4,
                spacing: 0.08,
                wet_edges: true,
                shape_dynamics: size_dyn(pressure_min(0.3)),
                transfer: transfer(Dynamic::default(), Dynamic { jitter: 0.15, minimum: 0.2, ..pressure() }),
                texture: Texture { scale: 1.0, ..paper(17, 0.35) }.with_pattern_size(256),
                ..base.clone()
            },
        ),
        // 3. Wet Edge: Pronounced pigment accumulation along stroke rims
        p(
            "suda.watercolor.wet_edge",
            "Wet Edge",
            BrushSettings {
                size: 50.0,
                hardness: 0.45,
                flow: 0.65,
                spacing: 0.06,
                wet_edges: true,
                shape_dynamics: size_dyn(pressure_min(0.25)),
                transfer: transfer(pressure_min(0.35), Dynamic::default()),
                texture: Texture { scale: 1.2, ..paper(9, 0.35) }.with_pattern_size(256),
                ..base.clone()
            },
        ),
        // 4. Granulating Wash: Ultramarine/Cobalt pigment settling into paper hollows
        p(
            "suda.watercolor.granulating_wash",
            "Granulating Wash",
            BrushSettings {
                size: 85.0,
                hardness: 0.2,
                flow: 0.35,
                spacing: 0.1,
                noise: true,
                wet_edges: true,
                shape_dynamics: size_dyn(pressure_min(0.35)),
                texture: Texture { mode: MaskMode::Subtract, invert: true, scale: 1.8, ..paper(19, 0.6) }.with_pattern_size(256),
                ..base.clone()
            },
        ),
        // 5. Watercolor Detail: Pointed Kolinsky sable brush for botanical watercolor details
        p(
            "suda.watercolor.detail",
            "Watercolor Detail",
            BrushSettings {
                size: 10.0,
                hardness: 0.7,
                flow: 0.75,
                spacing: 0.04,
                wet_edges: true,
                shape_dynamics: size_dyn(pressure_min(0.15)),
                smoothing: Smoothing { enabled: true, amount: 0.3, ..Default::default() },
                ..base.clone()
            },
        ),
    ]
}

fn textures_and_effects(base: &BrushSettings) -> Vec<BrushPreset> {
    let p = |id: &str, name: &str, b: BrushSettings| preset(id, "Textures & Effects", name, b);
    vec![
        // 1. Fine Grain: Micro-grain airbrush for smooth atmospheric shading
        p(
            "suda.fx.fine_grain",
            "Fine Grain",
            BrushSettings {
                size: 90.0,
                hardness: 0.0,
                flow: 0.25,
                spacing: 0.08,
                noise: true,
                shape_dynamics: size_dyn(pressure_min(0.3)),
                transfer: transfer(Dynamic::default(), pressure_min(0.15)),
                ..base.clone()
            },
        ),
        // 2. Paper Texture: Cold-press watercolor paper grain rubbing
        p(
            "suda.fx.paper_texture",
            "Paper Texture",
            BrushSettings {
                size: 80.0,
                hardness: 0.6,
                spacing: 0.12,
                shape_dynamics: size_dyn(pressure_min(0.4)),
                texture: Texture { mode: MaskMode::Subtract, invert: true, scale: 1.5, ..paper(7, 0.8) },
                ..base.clone()
            },
        ),
        // 3. Canvas Texture: Woven linen canvas texture for traditional art underlays
        p(
            "suda.fx.canvas_texture",
            "Canvas Texture",
            BrushSettings {
                size: 75.0,
                hardness: 0.75,
                spacing: 0.1,
                texture: Texture { mode: MaskMode::Subtract, invert: true, depth: 0.7, ..canvas(1, 0.7) },
                ..base.clone()
            },
        ),
        // 4. Spray: True aerosol spray can with accumulation and droplet dispersion
        p(
            "suda.fx.spray",
            "Spray",
            BrushSettings {
                size: 110.0,
                hardness: 0.0,
                flow: 0.2,
                spacing: 0.08,
                build_up: true,
                build_up_rate: 35.0,
                shape_dynamics: size_dyn(pressure_min(0.3)),
                scattering: Scattering { enabled: true, scatter: Dynamic::jitter(1.2), both_axes: true, count: 2, ..Default::default() },
                ..base.clone()
            },
        ),
        // 5. Splatter: Dynamic ink/paint flings with variable droplet sizes
        p(
            "suda.fx.splatter",
            "Splatter",
            BrushSettings {
                size: 65.0,
                tip: TipShape::Sampled(spatter_tip(64, 42, 16)),
                spacing: 0.55,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    size: Dynamic { jitter: 0.5, minimum: 0.25, ..pressure() },
                    angle: Dynamic::jitter(1.0),
                    flip_x_jitter: true,
                    flip_y_jitter: true,
                    ..Default::default()
                },
                scattering: Scattering { enabled: true, scatter: Dynamic::jitter(1.6), both_axes: true, count: 2, count_jitter: Dynamic::jitter(0.5) },
                ..base.clone()
            },
        ),
        // 6. Stipple: Classic ink dot stippling for engraving and shading
        p(
            "suda.fx.stipple",
            "Stipple",
            BrushSettings {
                size: 42.0,
                tip: TipShape::Sampled(spatter_tip(64, 88, 35)),
                spacing: 0.45,
                shape_dynamics: ShapeDynamics { enabled: true, angle: Dynamic::jitter(1.0), flip_x_jitter: true, ..Default::default() },
                scattering: Scattering { enabled: true, scatter: Dynamic::jitter(0.55), both_axes: true, ..Default::default() },
                transfer: transfer(pressure_min(0.3), Dynamic::default()),
                ..base.clone()
            },
        ),
        // 7. Grunge: Weathered sponge grit for aging and grunge distress
        p(
            "suda.fx.grunge",
            "Grunge",
            BrushSettings {
                size: 70.0,
                tip: TipShape::Sampled(sponge_tip(64, 27)),
                spacing: 0.4,
                shape_dynamics: ShapeDynamics {
                    enabled: true,
                    angle: Dynamic::jitter(1.0),
                    size: Dynamic { jitter: 0.35, minimum: 0.4, ..Default::default() },
                    ..Default::default()
                },
                transfer: transfer(Dynamic { jitter: 0.4, ..Default::default() }, pressure_min(0.3)),
                ..base.clone()
            },
        ),
    ]
}

impl Texture {
    fn with_pattern_size(mut self, n: u32) -> Self {
        if let Pattern::Procedural { size, .. } = &mut self.pattern {
            *size = n;
        }
        self
    }
}

/// The built-in Suda Design Signature preset collection, in display order (grouped by [`GROUPS`]).
pub fn builtin() -> Vec<BrushPreset> {
    let base = BrushSettings { pressure_size: false, spacing: 0.25, ..Default::default() };
    let mut v = inking_and_calligraphy(&base);
    v.extend(pencils_and_sketching(&base));
    v.extend(oils_and_acrylics(&base));
    v.extend(watercolors(&base));
    v.extend(textures_and_effects(&base));
    v
}

/// Find a preset by ID or name (case-insensitive), with fallback for legacy PhotoCraft names.
pub fn find<'a>(presets: &'a [BrushPreset], query: &str) -> Option<&'a BrushPreset> {
    // 1. Direct match by stable ID
    if let Some(p) = presets.iter().find(|p| p.id.as_deref().is_some_and(|id| id.eq_ignore_ascii_case(query))) {
        return Some(p);
    }
    // 2. Direct match by preset name
    if let Some(p) = presets.iter().find(|p| p.name.eq_ignore_ascii_case(query)) {
        return Some(p);
    }
    // 3. Backward-compatibility alias mapping for legacy PhotoCraft names
    let alias = match query.to_ascii_lowercase().as_str() {
        "hard round" | "hard round fine" | "hard round pressure size" | "hard round pressure opacity" | "hard round pressure opacity and flow" => "Fine Ink",
        "soft round" | "soft round large" | "soft round pressure size" | "soft round pressure opacity" => "Soft Watercolor",
        "calligraphy flat" => "Arabic Calligraphy",
        "airbrush soft" | "airbrush pressure flow" => "Fine Grain",
        "hard pencil" => "Hard Pencil",
        "soft pencil" => "Soft Pencil",
        "charcoal" => "Charcoal",
        "chalk" | "pastel" => "Textured Sketch",
        "dry bristle" => "Bristle Brush",
        "oil bristle" => "Round Oil",
        "ink pen" => "Technical Pen",
        "watercolor wash" | "ink wash sponge" => "Transparent Wash",
        "watercolor wet edges" => "Wet Edge",
        "spatter" => "Splatter",
        "stipple" => "Stipple",
        "sponge texture" => "Grunge",
        "canvas texture" => "Canvas Texture",
        _ => return None,
    };
    presets.iter().find(|p| p.name.eq_ignore_ascii_case(alias))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_are_grouped_and_unique() {
        let v = builtin();
        assert_eq!(v.len(), 30, "expected 30 signature presets, got {}", v.len());
        let mut names: Vec<&str> = v.iter().map(|p| p.name.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), v.len(), "duplicate preset names");

        let mut ids: Vec<&str> = v.iter().filter_map(|p| p.id.as_deref()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), v.len(), "duplicate or missing preset IDs");

        for g in GROUPS {
            let count = v.iter().filter(|p| p.group == g).count();
            assert!(count >= 5, "group {g} has only {count} presets");
        }
        assert!(v.iter().all(|p| p.builtin && GROUPS.contains(&p.group.as_str())));

        // Groups are contiguous, in panel order.
        let order: Vec<usize> = v.iter().filter_map(|p| GROUPS.iter().position(|g| *g == p.group)).collect();
        assert!(order.windows(2).all(|w| w[0] <= w[1]));

        // Arabic Calligraphy properties verified
        let arabic = find(&v, "suda.calligraphy.arabic").expect("Arabic Calligraphy by ID");
        assert_eq!(arabic.brush.angle, 60.0);
        assert_eq!(arabic.brush.roundness, 0.18);
        assert_eq!(arabic.brush.hardness, 0.95);
        assert_eq!(arabic.brush.shape_dynamics.size.control, Control::PenPressure);

        // Chisel Calligraphy properties verified
        let chisel = find(&v, "Chisel Calligraphy").expect("Chisel Calligraphy by name");
        assert_eq!(chisel.brush.angle, 45.0);
        assert_eq!(chisel.brush.roundness, 0.22);

        // Legacy name alias fallback works
        assert_eq!(find(&v, "Hard Round").map(|p| p.name.as_str()), Some("Fine Ink"));
        assert_eq!(find(&v, "Soft Round").map(|p| p.name.as_str()), Some("Soft Watercolor"));
        assert_eq!(find(&v, "Calligraphy Flat").map(|p| p.name.as_str()), Some("Arabic Calligraphy"));
    }
}
