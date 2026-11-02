//! SudaDesign Android IME & Virtual Keyboard Input Bridge.
//!
//! Connects Android's `GameActivity` / `GameTextInput` subsystem to `egui`.
//!
//! Root cause addressed:
//! `winit` on Android ignores `InputEvent::TextEvent` and `InputEvent::TextAction`,
//! causing virtual keyboard typing, predictive text, suggestions, and deletions to be dropped.
//! This bridge intercepts `GameTextInputState` from `AndroidApp` every frame in `raw_input_hook`,
//! computes differential edits (deletions and insertions), and injects them as native `egui::Event`s.

#[cfg(target_os = "android")]
use winit::platform::android::activity::input::{ImeOptions, InputType, TextInputAction, TextInputState};
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

/// Pure differential text calculator for IME state updates.
///
/// Given `prev` (text prior to current frame) and `next` (new text from IME),
/// finds the longest common prefix and suffix to isolate:
/// - `deleted_chars`: how many characters were removed from `prev`
/// - `inserted_text`: slice of `next` that was inserted
pub fn diff_text<'a>(prev: &str, next: &'a str) -> (usize, &'a str) {
    if prev == next {
        return (0, "");
    }

    // 1. Longest common character prefix
    let mut prev_chars = prev.char_indices();
    let mut next_chars = next.char_indices();
    let mut prefix_byte_len = 0;

    loop {
        match (prev_chars.next(), next_chars.next()) {
            (Some((p_idx, p_c)), Some((_, n_c))) if p_c == n_c => {
                prefix_byte_len = p_idx + p_c.len_utf8();
            }
            _ => break,
        }
    }

    // If there is no common prefix and both strings are non-empty,
    // treat as a full replacement rather than risking false suffix matches
    // that would corrupt input fields where cursor is at the end.
    if prefix_byte_len == 0 {
        return (prev.chars().count(), next);
    }

    let prev_rem = match prev.get(prefix_byte_len..) {
        Some(s) => s,
        None => "",
    };
    let next_rem = match next.get(prefix_byte_len..) {
        Some(s) => s,
        None => "",
    };

    // 2. Longest common character suffix of the remaining slices
    let mut p_rev = prev_rem.chars().rev();
    let mut n_rev = next_rem.chars().rev();
    let mut suffix_byte_len = 0;

    while let (Some(pc), Some(nc)) = (p_rev.next(), n_rev.next()) {
        if pc == nc {
            suffix_byte_len += pc.len_utf8();
        } else {
            break;
        }
    }

    let prev_mid_end = prev_rem.len().saturating_sub(suffix_byte_len);
    let next_mid_end = next_rem.len().saturating_sub(suffix_byte_len);

    let prev_mid = match prev_rem.get(..prev_mid_end) {
        Some(s) => s,
        None => "",
    };
    let next_mid = match next_rem.get(..next_mid_end) {
        Some(s) => s,
        None => "",
    };

    let deleted_chars = prev_mid.chars().count();
    (deleted_chars, next_mid)
}

/// State tracker for Android Virtual Keyboard (IME) communication.
#[derive(Default)]
pub struct AndroidImeBridge {
    /// Cached text from last frame's `TextInputState`.
    pub last_text: String,
    /// ID of the currently focused egui widget.
    pub last_focus: Option<egui::Id>,
    /// Whether the soft keyboard is currently visible.
    pub keyboard_visible: bool,
    /// Counter of consecutive frames without any keyboard-eligible focus.
    pub unfocused_frames: u32,
}

impl AndroidImeBridge {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(target_os = "android")]
    pub fn sync(
        &mut self,
        app: &AndroidApp,
        ctx: &egui::Context,
        raw_input: &mut egui::RawInput,
        has_type_edit: bool,
    ) {
        let current_focus = ctx.memory(|m| m.focused());
        let wants_keyboard = ctx.egui_wants_keyboard_input() || has_type_edit || current_focus.is_some();

        if wants_keyboard {
            self.unfocused_frames = 0;
            if current_focus != self.last_focus {
                self.last_focus = current_focus;
                self.last_text.clear();
                app.set_text_input_state(TextInputState::default());
                app.set_ime_editor_info(
                    InputType::TYPE_CLASS_TEXT,
                    TextInputAction::None,
                    ImeOptions::IME_FLAG_NO_FULLSCREEN | ImeOptions::IMG_FLAG_NO_EXTRACT_UI,
                );
            }
            if !self.keyboard_visible {
                app.show_soft_input(true);
                self.keyboard_visible = true;
            }
        } else {
            self.unfocused_frames = self.unfocused_frames.saturating_add(1);
            // Require 10 consecutive frames (~160ms) of no focus before hiding the keyboard.
            // This prevents focus transitions (e.g. DragValue becoming inline TextEdit, or tapping fields)
            // from causing the virtual keyboard to flutter or close prematurely.
            if self.unfocused_frames >= 10 && self.keyboard_visible {
                app.hide_soft_input(true);
                self.keyboard_visible = false;
                self.last_text.clear();
                self.last_focus = None;
            }
        }

        // 2. Poll and process GameTextInputState while keyboard is active
        if self.keyboard_visible {
            let state = app.text_input_state();
            let (deleted_chars, inserted_str) = diff_text(&self.last_text, &state.text);

            if deleted_chars > 0 || !inserted_str.is_empty() {
                // Emit Backspace key events for deletions
                for _ in 0..deleted_chars {
                    raw_input.events.push(egui::Event::Key {
                        key: egui::Key::Backspace,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    });
                    raw_input.events.push(egui::Event::Key {
                        key: egui::Key::Backspace,
                        physical_key: None,
                        pressed: false,
                        repeat: false,
                        modifiers: egui::Modifiers::NONE,
                    });
                }

                // Emit Text or Enter events for insertions
                if !inserted_str.is_empty() {
                    for (i, part) in inserted_str.split('\n').enumerate() {
                        if i > 0 {
                            raw_input.events.push(egui::Event::Key {
                                key: egui::Key::Enter,
                                physical_key: None,
                                pressed: true,
                                repeat: false,
                                modifiers: egui::Modifiers::NONE,
                            });
                            raw_input.events.push(egui::Event::Key {
                                key: egui::Key::Enter,
                                physical_key: None,
                                pressed: false,
                                repeat: false,
                                modifiers: egui::Modifiers::NONE,
                            });
                        }
                        if !part.is_empty() {
                            raw_input.events.push(egui::Event::Text(part.to_owned()));
                        }
                    }
                }

                self.last_text = state.text;
                ctx.request_repaint();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_text_empty_to_char() {
        let (del, ins) = diff_text("", "a");
        assert_eq!(del, 0);
        assert_eq!(ins, "a");
    }

    #[test]
    fn test_diff_text_append() {
        let (del, ins) = diff_text("abc", "abcd");
        assert_eq!(del, 0);
        assert_eq!(ins, "d");
    }

    #[test]
    fn test_diff_text_backspace() {
        let (del, ins) = diff_text("abcd", "abc");
        assert_eq!(del, 1);
        assert_eq!(ins, "");
    }

    #[test]
    fn test_diff_text_multiple_backspace() {
        let (del, ins) = diff_text("Hello World", "Hello");
        assert_eq!(del, 6);
        assert_eq!(ins, "");
    }

    #[test]
    fn test_diff_text_replace_in_middle() {
        let (del, ins) = diff_text("Hello World", "Hello Beautiful World");
        assert_eq!(del, 0);
        assert_eq!(ins, "Beautiful ");
    }

    #[test]
    fn test_diff_text_arabic_unicode() {
        let (del, ins) = diff_text("", "سودان");
        assert_eq!(del, 0);
        assert_eq!(ins, "سودان");

        let (del2, ins2) = diff_text("سودان", "سوداني");
        assert_eq!(del2, 0);
        assert_eq!(ins2, "ي");

        let (del3, ins3) = diff_text("سوداني", "سودان");
        assert_eq!(del3, 1);
        assert_eq!(ins3, "");
    }

    #[test]
    fn test_diff_text_enter_newline() {
        let (del, ins) = diff_text("1920", "1920\n");
        assert_eq!(del, 0);
        assert_eq!(ins, "\n");
    }

    #[test]
    fn test_diff_text_full_replacement() {
        let (del, ins) = diff_text("1920", "3840");
        assert_eq!(del, 4);
        assert_eq!(ins, "3840");
    }

    #[test]
    fn test_diff_text_delete_word_in_middle() {
        let (del, ins) = diff_text("Hello World Today", "Hello Today");
        assert_eq!(del, 6);
        assert_eq!(ins, "");
    }
}
