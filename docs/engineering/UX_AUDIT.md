# Suda Design — UI/UX Engineering & Ergonomics Audit

**Audit Date:** November 2026  
**Auditor:** Professional UI/UX Architect & Mobile Interaction Engineer  
**Focus:** Touch Ergonomics, Multi-Device Adaptation, Screen Real Estate & Interaction Friction

---

## 1. Executive Summary

Suda Design offers comprehensive Photoshop-level feature completeness (68 tools, 631 menu items, full layer styles, vector shapes, masks, filters). However, its user experience currently reflects a **desktop-first direct port** rather than a **touch-native creative suite**. 

Users on touchscreens face severe interaction friction due to:
1. Complete dependency on physical modifier keys (`Shift`, `Alt`, `Ctrl`).
2. Accidental tool marks caused by lack of touch-gesture arbitration.
3. Narrow desktop menus and tool flyouts vulnerable to finger touch jitter.
4. Dock panels consuming up to 40% of landscape phone screens.
5. Soft keyboard (IME) occluding dialog inputs and confirmation buttons.

---

## 2. Exhaustive UX Component Audit

### 2.1 Toolbar & Tool Selection
- **Desktop Paradigm:** 1-column or 2-column icon rail (30–36 pt width) with tiny corner flyout triangles. Right-click or mouse hold opens nested tools.
- **Touch Reality:**
  - `crates/ui-egui/src/panels/toolbar.rs` uses `resp.is_pointer_button_down_on()` for long-press timing ($0.35\text{ s}$).
  - Finger jitter (2–6 pt contact shift) causes the hit-test to fail, aborting the long-press timer.
  - Slicing out of the button boundaries triggers `tool_flyout_drag_open` prematurely.
  - **Verdict:** Unreliable tool switching on mobile without a stylus.

### 2.2 Direct Canvas Manipulation & Gesture Arbitration
- **Desktop Paradigm:** Left click = Paint/Select. Space + Drag = Pan. Wheel / Alt+Wheel = Zoom.
- **Touch Reality:**
  - `crates/ui-egui/src/canvas.rs` registers a single touch contact as an immediate click/drag (`Sense::click_and_drag()`).
  - When a user intends to pinch-to-zoom or pan with two fingers, the asynchronous landing of the first finger (even by 10 ms) produces a stray dab or selection anchor before the pinch is recognized.
  - **Verdict:** Frequent unwanted canvas marks requiring repetitive Undos.

### 2.3 The Missing Modifiers Dilemma
- **Functionality Crippled on Touchscreens:**
  - **Free Transform:** Proportional scaling requires `Shift`. Skew/Distort requires `Ctrl`. Center-scaling requires `Alt`.
  - **Selection Tools:** Adding to selection requires `Shift`. Subtracting requires `Alt`. Intersecting requires `Shift+Alt`.
  - **Brush / Pencil:** Rapid color picking requires holding `Alt` (Eyedropper toggle). Straight stroke requires `Shift`.
  - **Zoom Tool:** Zooming out requires `Alt`.
- **Current Fallback:** `Window > Modifier Keys` is a tiny floating panel anchored at `(-420, 80)`, virtually inaccessible on phones.
- **Verdict:** Critical workflow bottleneck.

### 2.4 Menu Hierarchy & Tap Accuracy
- **Desktop Paradigm:** Cascading hierarchical menu bar (`File > New / Open / Export...`, `Layer > New Adjustment Layer > Curves...`).
- **Touch Reality:**
  - Top titles have been padded (`vec2(10.0, 7.0)` in `menus/bar.rs`).
  - Dropdown menu rows retain a height of 20–24 pt.
  - According to Fitts' Law and mobile ergonomics standards (Material 3 / Apple HIG), finger touch targets must be at least **44×44 pt** (or at minimum 40 pt vertical spacing).
  - Users frequently mis-tap adjacent menu items (Fat-Finger Problem).
  - **Verdict:** Submenus require extreme precision, causing user fatigue.

### 2.5 Screen Real Estate: Phones vs Tablets vs Desktops
- **Phone Landscape (20:9 / 21:9 Aspect Ratio):**
  - Logical height is constrained (~360–412 pt).
  - The Dock panel on the right occupies 240–340 pt width (~35–40% of screen width).
  - The canvas is squeezed into a narrow central strip.
- **Phone Portrait:**
  - The canvas is tall and narrow. Opening the Layers panel or Color panel covers over 50% of the active design.
- **Verdict:** Rigid dock layout fails on small form factors; needs a sliding drawer overlay model.

### 2.6 Virtual Keyboard (IME) Occlusion
- **Current Behavior:**
  - In `apps/photocraft-android/src/lib.rs`, `SafeAreaInsets.ime` tracks the soft keyboard height.
  - When typing text in dialogs (`Image Size`, `Canvas Size`, `New Document`) or text layers, the keyboard rises from the bottom (occupying 40–50% of the viewport).
  - Open dialogs remain centered, leaving inputs and "OK / Cancel" buttons buried underneath the keyboard.
- **Verdict:** Severe usability defect preventing document setup or precise numeric editing.

### 2.7 Complex Dialogs vs Dedicated Workspaces
- **Current Behavior:**
  - Workspaces like `Select and Mask`, `Liquify`, `Curves`, and `Filter Gallery` render as modal floating windows packed with sliders and checkboxes.
  - On mobile, these dialogs obscure the entire canvas preview and make fine slider adjustments nearly impossible.
- **Verdict:** Violates modern mobile design principles; complex tools need immersive workspaces with collapsible bottom sheets.

---

## 3. Quantitative Touch Ergonomics Assessment

| Interaction Area | Current Touch Target | Minimum Required (Mobile Standard) | Defect Severity |
| :--- | :---: | :---: | :---: |
| Main Toolbar Buttons | 36 × 36 pt | 44 × 44 pt (Phone) / 38 × 38 pt (Tablet) | Medium |
| Tool Flyout Sub-items | 26 × 26 pt | 40 × 40 pt | **High** |
| Top Menu Titles | 30 × 26 pt | 44 × 36 pt | Medium |
| Dropdown Menu Items | 180 × 22 pt | 220 × 40 pt | **Critical** |
| Transform Handles | 8 × 8 pt visual | 24 × 24 pt touch receiver | **Critical** |
| Dock Panel Headers | 24 pt height | 40 pt height | Medium |
| Slider Thumbs | 10 × 14 pt | 28 × 28 pt | **High** |
| Modifier Keys (Mobile) | None (Hidden) | 44 × 44 pt Floating HUD | **Critical** |

---

## 4. Remediation Architecture & Touch-Native UX Specifications

To solve these deficiencies systematically without degrading the desktop experience:

### Specification 1: Floating Virtual Modifier HUD (`ModifierHud`)
- **Visual Design:** Compact, semi-transparent floating pill containing `Shift`, `Alt`, and `Ctrl`.
- **Interaction Model:**
  - **Tap-to-Lock:** Tap toggles persistent latching (visual active state with accent glow).
  - **Hold-to-Modify:** Touch and hold with thumb while drawing or transforming with the other hand / stylus.
  - **Draggable & Dockable:** Can be repositioned along the canvas edges or snapped adjacent to the toolbar.
  - **Toggle Visibility:** Fast toggle button in the top bar to show/hide.

### Specification 2: Deferred Gesture Arbitration & Touch Slop (`TouchGestureArbiter`)
- **Pending Gesture Buffer:**
  - When a touch begins on the canvas, buffer the initial point for 20 ms or until movement exceeds 6 pt.
  - If a second finger lands within this window: cancel pending stroke cleanly, activate 2-finger Pan/Pinch-Zoom navigation.
  - If no second touch occurs and threshold is reached: commit the pending point into the tool pipeline without latency penalty.

### Specification 3: Mobile Slide-Over Drawer Dock (`MobileDrawerDock`)
- **On Phones:**
  - Dock does not occupy persistent layout width.
  - Panels (Layers, Properties, Colors) slide over from the right edge as a drawer overlay with a semi-transparent scrim.
  - Tapping the canvas or dragging the drawer handle automatically dismisses the drawer, giving 100% canvas visibility during editing.

### Specification 4: IME Auto-Lift & Dialog Viewport Adaptation (`ImeViewportLift`)
- Dynamically calculate `viewport_height - insets.ime`.
- Offset active dialogs upward so the focused text edit field and dialog buttons remain centered in the visible area above the soft keyboard.

### Specification 5: Quick Actions Radial / Floating Bar (`QuickActionsBar`)
- Instant touch access to essential frequent actions:
  - `Undo`, `Redo`, `Deselect`, `Invert Selection`, `Duplicate Layer`, `Fit Screen`, `Eyedropper`.

