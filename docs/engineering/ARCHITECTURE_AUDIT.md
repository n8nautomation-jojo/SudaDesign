# Suda Design — Architecture Engineering Audit

**Audit Date:** November 2026  
**Auditor:** CTO & Principal Systems Architect  
**Codebase:** Rust 100% Native (eframe/egui on wgpu) — Branch: `feature/suda-signature-brushes`  
**Status:** Baseline Established — 0 Layering Violations across 32 Crates

---

## 1. Executive Architectural Assessment

Suda Design possesses a rigorous, clean-room, layer-enforced architecture designed for high scalability, zero panics, and Photoshop parity. The engine separates data models, imaging algorithms, compositing, command dispatch, and UI presentation into 7 distinct layers (L0 through L6).

However, while the backend engine is mathematically decoupled and pure, the **input, presentation, and gesture layers** carry deep structural desktop assumptions that create friction on touch-native devices.

---

## 2. Workspace Crate Map & Enforced Layering

The workspace consists of 32 crates strictly enforced by `cargo xtask layers`:

```text
Layer Level          Crates                                            Responsibility & Boundaries
-------------------------------------------------------------------------------------------------------------------
L0: Foundation       geom, cms, color, raster                          Pure mathematics, ICC color management,
                     psd, codecs, tablet, raw, affinity, heif, ipp     pixel formats, COW tiles, format decoders.
                                                                       Zero workspace dependencies.
-------------------------------------------------------------------------------------------------------------------
L1: Document Model   doc                                               Document, Layer, LayerContent, Mask,
                                                                       Effects, SmartObjects. Pure data, no rendering.
-------------------------------------------------------------------------------------------------------------------
L2: Algorithmic Core ops, paint, algo, text, vector                    History/Undo, brush engine & dabs, image
                                                                       filters/transforms, HarfBuzz type, Bezier paths.
-------------------------------------------------------------------------------------------------------------------
L3: Compositor &     compose (CPU oracle), gpu (wgpu pipelines),       Tile compositing, blend modes, GPU shaders,
    Format           format (.pcraft native format)                    native serialization with field-drop guards.
-------------------------------------------------------------------------------------------------------------------
L4: I/O & Plugins    io (PSD/PSB import/export), plugins (WASM)        File translation, third-party sandboxed plugins.
-------------------------------------------------------------------------------------------------------------------
L5: Engine           engine                                            Session, command registry (631 live commands),
                                                                       parameter validation, document lifecycle.
-------------------------------------------------------------------------------------------------------------------
L6: Presentation     ui-egui, automation                               egui/eframe GUI shell, tool state, MCP server.
-------------------------------------------------------------------------------------------------------------------
Applications         photocraft (desktop), photocraft-android,         Native binaries and packaging targets.
                     photocraft-cli, photocraft-web                    No Tauri, no webviews, no Electron.
```

### Architectural Strengths:
1. **Unidirectional Dependency Flow:** No circular dependencies. L0 never imports L1+; L1 never imports L2+.
2. **Zero-Panic Policy:** Every crate root carries `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
3. **Format Integrity:** `photocraft-format` uses compile-time exhaustiveness checks to prevent silent dropping of document fields.
4. **Pure Command Pattern:** All mutations route through `Session::run(cmd_id, params)`, making automation, scripting, and undo/redo deterministic.

---

## 3. State Management & Data Ownership

### 3.1 Session vs Document Ownership
```mermaid
graph TD
    App[PhotocraftApp (ui-egui)] -->|Owns| Sess[Session (engine)]
    App -->|Owns| UIState[UiState (ui-egui)]
    Sess -->|Owns Arc/Mutex| Docs[HashMap<DocId, Document>]
    Sess -->|Owns| CmdReg[CommandRegistry]
    Docs -->|Contains| Layers[Layer Tree & COW Tiles]
    UIState -->|Tracks| View[Viewport Zoom / Center]
    UIState -->|Tracks| ActiveTool[Tool & Tool Options]
    UIState -->|Tracks| Dock[Dock Panels State]
```

- **`Document` (in `photocraft-doc`):** Completely immutable during read operations. Layers use sparse tile grids with Copy-On-Write (`photocraft-raster::Surface`).
- **`History` (in `photocraft-ops`):** Tile-level delta compression; undoing or redoing an operation restores tile references without deep-copying full multi-megapixel buffers.
- **`UiState` (in `photocraft-ui-egui::state`):** Encapsulates tool selection, viewport transformations (`zoom`, `center`, `rotation`), panel layout, and dialog states.

---

## 4. Android Native Layer & Platform Bridges

### 4.1 Native Integration Architecture
The Android application (`apps/photocraft-android`) uses Android GameActivity (`winit 0.30` backend) interfacing directly with native Rust code via JNI and C bindings:

1. **JNI Services (`services.rs`):**
   - Dispatches SAF (Storage Access Framework) file pickers (`OpenDocument`, `CreateDocument`).
   - Relies on streaming byte buffers and internal cache persistence (`session_store.rs`).
2. **Thread Scheduling:**
   - Detects Android big.LITTLE ARM topology and caps Rayon global workers to `(num_cpus / 2).clamp(2, 4)` (`lib.rs:296`).
   - Prevents thermal runaway and SurfaceFlinger starvation.
3. **Safe Area Insets (`responsive.rs`):**
   - Lock-free atomic storage (`INSET_LEFT`, `INSET_TOP`, `INSET_RIGHT`, `INSET_BOTTOM`, `INSET_IME`).
   - Reports cutouts, punch-holes, system navigation bars, and IME height without mutex contention.

---

## 5. Architectural Deficiencies & Root Causes

Our codebase inspection reveals four structural architectural bottlenecks:

### Defect A1: Lack of an Unified Input & Gesture Abstraction Layer
- **Location:** `crates/ui-egui/src/canvas.rs`, `crates/ui-egui/src/input/`.
- **Finding:** Currently, `canvas.rs` binds mouse events (`egui::PointerButton::Primary`, `Secondary`, `Middle`) directly to tool actions. Touch events are converted by `eframe` into simulated pointer clicks and pinches.
- **Consequence:** 
  - There is no central **Gesture Arbiter**.
  - A single touch down immediately dispatches `ToolEvent::Down`, causing a brush dab before an incoming second touch can be classified as a pan/zoom gesture.

### Defect A2: Modifier Keys Coupled Exclusively to Hardware Keyboards
- **Location:** `crates/ui-egui/src/canvas.rs:655-660`, `crates/ui-egui/src/menus/workspace_ui.rs:25-36`.
- **Finding:** Tool modifiers (`Shift` for aspect ratio / straight strokes, `Alt` for color sampling / central zoom, `Ctrl` for transform skew) inspect `ctx.input(|i| i.modifiers)` directly. 
- **Consequence:** `ShellUi::sticky_alt/shift/command` exists in `workspace_ui.rs`, but it is tied to an obscure floating desktop window (`window.panel.modifierKeys`). The core canvas and tools do not have a unified virtual modifier bus.

### Defect A3: Fragmentation of Responsive Screen Profiles
- **Location:** `apps/photocraft-android/src/responsive.rs` vs `crates/ui-egui/src/dialogs.rs:is_mobile_screen`.
- **Finding:** The detection of mobile screens is split:
  - `responsive.rs` calculates `ScreenProfile` (`PhoneLandscape`, `PhonePortrait`, `TabletLandscape`, `TabletPortrait`).
  - `dialogs.rs` has a separate ad-hoc heuristic `is_mobile_screen(ctx)` checking `< 600 dp`.
  - Many UI panels do not consult either, remaining fixed at desktop proportions.
- **Consequence:** Dialogs, menus, and panels do not share a single source of truth for responsive layout decisions.

### Defect A4: Heavy Monolithic Canvas Module
- **Location:** `crates/ui-egui/src/canvas.rs` (2,698 lines).
- **Finding:** `canvas.rs` mixes texture uploading, GPU pipeline binding, navigation, tool event dispatch, guide snapping, cursor rendering, and dialog canvases in one oversized module.
- **Consequence:** High cognitive load, difficult isolated testing, and increased regression risk when adjusting touch logic.

---

## 6. Architectural Decision Plan (ADRs)

1. **ADR-001: Centralized Virtual Input Abstraction Bus (`VirtualInputBus`):**
   Unify physical keyboard modifiers and on-screen floating modifiers into a single resolver that feeds `ToolEvent`.
2. **ADR-002: Deferred Touch Gesture Arbiter (`TouchGestureArbiter`):**
   Introduce a spatial-temporal buffer (20ms / 6pt threshold) on canvas touch input to disambiguate single-touch painting from two-finger navigation cleanly.
3. **ADR-003: Unified Adaptive Screen Architecture (`AdaptiveUi`):**
   Promote `ScreenProfile` into a shared, centralized core module inside `ui-egui`, making screen metrics accessible to all panels, dialogs, and toolbars.

