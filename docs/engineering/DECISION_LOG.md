# Suda Design — Architecture Decision Log (ADR)

This document tracks all significant architectural decisions, alternatives evaluated, trade-offs, and rationale.

---

## ADR-001: Unified Virtual Input Bus for Modifiers (`VirtualInputBus`)
- **Status:** Approved / In Implementation (Phase 1)
- **Context:**
  Photoshop parity operations (proportional transform, central zoom, color sampling, selection modes) rely on `Shift`, `Alt`, and `Ctrl`. On Android touchscreens, physical keyboards are rarely present.
- **Problem:**
  Currently, `ShellUi::sticky_alt/shift/command` is trapped inside a hidden desktop window (`Window > Modifier Keys`). The canvas and tools check `ctx.input(|i| i.modifiers)` directly.
- **Options Considered:**
  1. *Add modal buttons inside each tool's options bar:* Too cluttered; each tool would reinvent aspect ratio toggles and color sampling.
  2. *Rely strictly on multi-touch gestures:* Complex, non-standard, hard to discover, cannot handle 3 concurrent modifiers.
  3. *Centralized Virtual Floating Modifier HUD (`ModifierHud`):* A sleek, floating draggable pill providing tap-to-lock and hold-to-modify states for `Shift`, `Alt`, and `Ctrl`, integrated directly into `PhotocraftApp` input resolution.
- **Decision:** Option 3. Implement `ModifierHud` that feeds a unified modifier state combined with physical keyboard input (`physical_modifiers | virtual_modifiers`).
- **Consequences:** All 68 tools, transforms, selections, and navigation immediately gain full modifier capabilities on touchscreens with zero tool-specific logic changes.

---

## ADR-002: Deferred Touch Gesture Arbiter (`TouchGestureArbiter`)
- **Status:** Approved / In Implementation (Phase 1)
- **Context:**
  Touching the canvas with one finger immediately fires `ToolEvent::Down`, committing a dab or stroke. When a user intends to pan/zoom with two fingers, the asynchronous landing of the first finger causes stray dabs.
- **Problem:**
  Mobile users are forced to repeatedly Undo stray marks whenever navigating the canvas.
- **Options Considered:**
  1. *Require switching to the Hand tool to navigate:* Destroys workflow velocity compared to Procreate or Infinite Painter.
  2. *Fixed long delay on all brush strokes:* Adds unacceptable input lag to drawing.
  3. *Deferred Touch Slop Window (20 ms or 6 pt):* Hold the initial stroke point in a pending buffer for up to 20 ms. If a second finger touches down, abort the pending stroke without marking the canvas and initiate 2-finger pan/zoom. If motion exceeds 6 pt or time expires without a second touch, immediately flush the stroke to the tool. Stylus input bypasses the buffer.
- **Decision:** Option 3.
- **Consequences:** Eliminates accidental canvas marks during navigation while maintaining instantaneous drawing feel for stylus and deliberate finger strokes.

---

## ADR-003: Unified Adaptive Screen Architecture (`AdaptiveUi`)
- **Status:** Approved / In Implementation (Phase 1)
- **Context:**
  Mobile checks are fragmented between `apps/photocraft-android/src/responsive.rs` and `crates/ui-egui/src/dialogs.rs:is_mobile_screen`.
- **Decision:** Centralize screen profile resolution into `crates/ui-egui/src/responsive_layout.rs` so all crates, panels, dialogs, and toolbars share the exact same breakpoints and safe-area metrics.
- **Consequences:** Eliminates layout discrepancies and visual clipping across devices.

---

## ADR-004: Floating Smart Quick Actions HUD (`QuickActions`)
- **Status:** Implemented & Validated (Phase 2)
- **Context:**
  High-frequency operations like Undo, Redo, Fit Screen, Deselect, Invert Selection, Duplicate Layer, and Color Eyedropper are buried inside desktop-style cascading menus (`Edit`, `Select`, `Layer`, `View`). On touch devices, accessing these repeatedly severely degrades artist productivity and flow.
- **Decision:** Implement a floating, draggable, minimizable Quick Actions bar (`crates/ui-egui/src/input/quick_actions.rs`) enabled by default on mobile touchscreens. When minimized, it sits as a compact `⚡` badge. When expanded, it offers instant 1-tap thumb access to primary actions with dynamic enabled/disabled validation.
- **Consequences:** Dramatically accelerates touch painting workflows and eliminates deep menu diving.

---

## ADR-005: IME Auto-Lift Modal Dialog Framework (`ImeAutoLift`)
- **Status:** Implemented & Validated (Phase 3)
- **Context:**
  On Android, modal dialogs (New Document, Layer Style, Image Size, Text properties) were positioned at `Align2::CENTER_CENTER` of the entire screen. When the soft keyboard (IME) opened to type names or numeric parameters, the bottom 40-50% of the screen was occluded, hiding the input fields and OK/Cancel buttons.
- **Decision:** Inspect runtime `SafeAreaInsets.current().ime` dynamically and provide `set_ime_inset` / `get_ime_inset` in `crates/ui-egui/src/dialogs.rs`. When `ime > 0.0`, modal dialogs automatically translate upward by `ime * 0.5` (or more), keeping all input fields and action buttons perfectly visible and accessible.
- **Consequences:** Eliminates keyboard occlusion across all Android devices and ensures seamless text and number entry.

---

## ADR-006: Mobile Slide-Over Drawer Dock Architecture (`SlideOverDock`)
- **Status:** Implemented & Validated (Phase 2)
- **Context:**
  On compact smartphone screens in landscape orientation, keeping the Layers dock panel open side-by-side consumed 240+ pt out of an ~800 pt screen width (over 45% of the viewport), starving the canvas.
- **Decision:**
  1. Default phone portrait and landscape modes to start with all dock groups collapsed, giving 100% full screen width to the canvas on document open.
  2. The icon rail remains available on the screen edge to summon any panel (Layers, Color, Properties) on demand as an accordion drawer.
  3. Tapping the canvas automatically collapses any open dock drawer, instantly returning 100% canvas width for uninterrupted painting.
- **Consequences:** Reclaims maximum screen real estate on mobile phones while preserving instantaneous access to full professional panels.

---

## ADR-007: Full-Height Mobile Layers Drawer & Pinned Actions Architecture (`FullHeightLayersDrawer`)
- **Status:** Implemented & Validated (Phase 2 / Phase 3)
- **Context:**
  Inside the desktop-derived dock column on mobile devices, the Layers panel was squeezed beneath the top title bar, tool options bar, and collapsed group tab strips, leaving only a ~30 pt vertical slit for layer rows. When layer rows exceeded this slit, the footer bar containing `fx` (Layer Styles), Layer Mask, Adjustments, Group, New Layer, Delete, and Link was pushed off-screen, forcing artists to scroll continuously up and down to manage layers.
- **Decision:**
  1. Implement a dedicated full-height slide-over Layers drawer (`crates/ui-egui/src/panels/layers.rs::mobile_layers_drawer`) in `egui::Area` at `Order::Foreground`.
  2. The drawer spans 100% of the screen height (`screen.height()`) from top to bottom, with a soft backdrop scrim over the canvas that dismisses on tap.
  3. Bounded the layer rows scroll area to `(ui.available_height() - footer).max(40.0)` with `auto_shrink: [false, false]`, mathematically guaranteeing that the footer bar (`widgets::panel_footer`) is permanently pinned and 100% visible at the bottom of the drawer.
  4. Enlarged mobile footer touch targets to 34 pt for thumb-friendly interaction with `fx`, adjustments, and layer actions.
- **Consequences:** Eliminates vertical cramping, displays all layers together without scrolling on tall screens, keeps the entire `fx` list and action buttons visible at all times, and achieves Procreate / Photoshop-iPad class UX.

---

## ADR-008: Non-Extract Native Keyboard, Adaptive Insets & Keystroke Shortcut Bridge (`AdaptiveVirtualKeyboard`)
- **Status:** Implemented & Validated (Phase 3 / Mobile Excellence)
- **Context:**
  When invoking the virtual keyboard on Android in landscape orientation, Android's `InputMethodService` defaulted to **Extract Mode** (`ExtractEditText` + fullscreen keyboard), popping up a completely opaque system overlay that completely covered the canvas artwork, leaving users unable to see what they were typing or where their text was placed. Furthermore, the keyboard could only be triggered while actively inside a text field, completely blocking mobile/tablet users from executing classic keystroke shortcuts (e.g. `Ctrl+S`, `Ctrl+Z`, `Ctrl+D`, `B`, `E`, `V`).
- **Decision:**
  1. **Non-Extract Enforcement:** Subclassed `InputEnabledSurfaceView` in `MainActivity.kt` and overrode `onCreateInputConnection` to unconditionally force `EditorInfo.IME_FLAG_NO_EXTRACT_UI | EditorInfo.IME_FLAG_NO_FULLSCREEN`. This permanently prevents Android from switching to the opaque fullscreen extract dialog, ensuring the user's standard native phone keyboard (Gboard, Samsung Keyboard, etc.) appears docked at the bottom or floating, preserving 100% canvas visibility.
  2. **Adaptive Canvas Inset Reserving:** In `apps/photocraft-android/src/lib.rs`, when `insets.ime > 0.0`, a bottom panel spacer is dynamically inserted for `insets.ime`, automatically adjusting the canvas area above the keyboard so artwork is never occluded.
  3. **Universal Keyboard Summoning:** Added `soft_keyboard_requested: bool` in `ShellUi` and added `[⌨]` toggle buttons in the Quick Actions HUD and title bar, allowing artists to summon their native keyboard at any time without selecting the Type tool.
  4. **Dual-Mode Keystroke Bridge:** In `apps/photocraft-android/src/ime.rs`, differentiated text processing between:
     - *Mode A (Text Entry):* Emits `egui::Event::Text` and `Key::Backspace` for focused text edits and Type tool.
     - *Mode B (Command & Shortcuts):* Emits `egui::Event::Key` combining characters mapped via `char_to_key` with active latched modifiers (`sticky_command`, `sticky_alt`, `sticky_shift`), and immediately clears the IME buffer so subsequent keystrokes act as instantaneous discrete shortcuts (`Ctrl+S`, `Ctrl+Z`, `Ctrl+D`, tool shortcuts `B`, `E`, `V`, `[` / `]`, etc.).
  5. **Keyboard Accessory Strip (`KeyboardBar`):** Built `crates/ui-egui/src/input/keyboard_bar.rs`, anchored directly above the keyboard with one-tap modifier chips (`Ctrl`, `Alt`, `Shift`), high-frequency action chips (`Undo`, `Redo`, `Save`, `Copy`, `Paste`, `Deselect`, `Fit`), quick tool chips (`Move`, `Brush`, `Eraser`, `Type`), and a clean keyboard dismiss button.
- **Consequences:** Solves the critical mobile keyboard occlusion bug permanently, lets users type with their familiar native keyboard while viewing their artwork live, and opens up the entire Photoshop keyboard shortcut system to touch and tablet devices.
