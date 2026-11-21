# Suda Design — Code Quality & Maintainability Audit

**Audit Date:** November 2026  
**Auditor:** Code Quality & Software Reliability Engineer  
**Scope:** Workspace Health, Lint Compliance, File Cohesion, Error Handling & Test Coverage

---

## 1. Executive Summary

Suda Design demonstrates an extraordinarily high standard of Rust engineering discipline in terms of memory safety, crash prevention, and compiler lints. 

The workspace enforces a strict **Zero-Panic Policy** across all 32 production crates. Non-test code is forbidden from calling `unwrap()`, `expect()`, `panic!()`, `todo!()`, or `unimplemented!()`.

However, the rapid accumulation of desktop features and Photoshop parity commands has led to **several massive monolithic files** with high cyclomatic complexity, tight coupling in the presentation layer, and repetitive dialog layout boilerplate.

---

## 2. Compiler Lints & Zero-Panic Integrity

### 2.1 Workspace Lint Configuration
In `Cargo.toml` (workspace root):
```toml
[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
```
- **Audit Verification:** Verified across all production crate roots. Tests are quarantined and allowed to assert via standard macros.
- **Unsafe Code:** `#![forbid(unsafe_code)]` is active workspace-wide. The only isolated exception is `photocraft-tablet` (`crates/tablet/src/macos.rs` and `src/windows.rs`), which interfaces with AppKit and Win32 pointer messages, each block backed by formal `// SAFETY:` proofs.

---

## 3. High-Complexity & Monolithic Files

Our repository analysis identified the following files exceeding 1,000 lines of code with multiple overlapping responsibilities:

| File Path | Line Count | Primary Responsibilities | Risk & Coupling |
| :--- | :---: | :--- | :--- |
| `crates/ui-egui/src/tools/transform_tool.rs` | **3,158** | 8 handles, skew, warp, perspective, 9-point pivot, multi-layer transform, raster preview. | Very High. Modifying touch handle sizing touches mathematical coordinate code. |
| `crates/ui-egui/src/canvas.rs` | **2,698** | Display texture cache, GPU view transform, tool input routing, cursors, guides, rulers. | **Critical Bottleneck.** Combines rendering, navigation, and input dispatch. |
| `crates/ui-egui/src/dialogs.rs` | **1,345** | 30+ parameter dialogs, mobile checks, curves layout, color picker bindings. | High. High layout code duplication across different dialogs. |
| `crates/ui-egui/src/menus/workspace_ui.rs` | **812** | Workspaces, modifier keys panel, custom aspect ratios, shell dialogs. | Medium. Mixes modifier state with custom PAR and workspaces. |
| `crates/ui-egui/src/panels/toolbar.rs` | **579** | Toolbar layout, 22 slot flyouts, double-column logic, long-press timer, color chips. | High. Input logic tightly bound to egui widget layout. |

### Recommendation:
Refactor through progressive extraction:
1. Extract `canvas/touch_gesture.rs` out of `canvas.rs` to isolate touch arbitration from rendering.
2. Extract common `dialogs/dialog_builder.rs` to eliminate repeated egui window setup code.
3. Decouple `ModifierKeys` from `workspace_ui.rs` into a dedicated `input/modifiers.rs` module.

---

## 4. Code Duplication & Pattern Consistency

### 4.1 Dialog Sizing & Responsive Checks
- In `crates/ui-egui/src/dialogs.rs`:
  Multiple dialogs hand-roll screen boundary checks:
  ```rust
  let screen = ctx.content_rect();
  let max_w = screen.width() - 40.0;
  let max_h = screen.height() - 60.0;
  ```
- This calculation is repeated in `curves_dialog`, `image_size_dialog`, `canvas_size_dialog`, and `preferences_dialog`.
- **Solution:** Provide a unified `DialogFrame` abstraction that automatically handles safe areas, screen clamping, and IME insets.

### 4.2 Button & Hit Target Styling
- Buttons across `panels/toolbar.rs`, `panels/layers.rs`, `menus/bar.rs`, and `tools/` independently define sizing and padding (`vec2(8.0, 6.0)`, `vec2(6.0, 3.0)`, `36.0`, `30.0`).
- **Solution:** Centralize touch metrics into `theme::Tokens` or `responsive::TouchMetrics`.

---

## 5. Test Coverage & Quality Assurance Infrastructure

### 5.1 Existing Strengths:
1. **Fuzzing & Panic Hunt:** `cargo test -p photocraft-engine --test panic_hunt` aggressively fuzzes all 631 commands with adversarial JSON parameters to verify zero panics.
2. **Oracle Corpora:** Automated tests compare Photoshop-authored PSDs and psd-tools test files against Suda Design imports (`crates/io/tests/it/corpus.rs`).
3. **Layering Gate:** `cargo xtask layers` ensures strict architectural boundaries in CI.

### 5.2 Identified Test Gaps:
1. **Touch Gesture Multi-finger Simulation:** No automated unit tests for 2-finger pinch vs 1-finger stroke arbitration under asynchronous arrival conditions.
2. **IME Inset Layout Tests:** No headless regression tests validating dialog position when `SafeAreaInsets.ime > 0.0`.
3. **Modifier State Latching Tests:** Need unit tests for virtual modifier key latching and concurrent modifier resolution.

---

## 6. Actionable Code Quality Goals

1. **Zero Clippy Warnings:** Maintain `-D warnings` across all touched crates.
2. **Modular Input Architecture:** Separate raw event handling from canvas drawing.
3. **Comprehensive Unit Testing:** Every new UX feature (Virtual Modifiers, Gesture Arbiter, Mobile Drawers) must land with 100% test pass rate and automated regression coverage.

