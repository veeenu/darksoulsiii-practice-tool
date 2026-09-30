use std::ffi::CStr;

use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use windows::Win32::UI::Input::XboxController::*;

use crate::practice_tool::GAMEPAD_STATE;

const LT: u32 = 1 << 16;
const RT: u32 = 1 << 17;
const KEYBOARD: u32 = 1 << 18;

const KEYBOARD_SLOTS: usize = 5;

const GRID_FRAMES: usize = 10;

const COLOR_A: [f32; 4] = [0.4, 0.75, 0.25, 1.0];
const COLOR_B: [f32; 4] = [0.85, 0.2, 0.2, 1.0];
const COLOR_X: [f32; 4] = [0.2, 0.45, 0.9, 1.0];
const COLOR_Y: [f32; 4] = [0.95, 0.8, 0.15, 1.0];
const COLOR_OTHER: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const COLOR_KEYBOARD: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const COLOR_GRID: [f32; 4] = [0.5, 0.5, 0.5, 0.25];

const GAMEPAD_ROWS: [(&str, u32, [f32; 4]); 16] = [
    ("a", XINPUT_GAMEPAD_A.0 as u32, COLOR_A),
    ("b", XINPUT_GAMEPAD_B.0 as u32, COLOR_B),
    ("x", XINPUT_GAMEPAD_X.0 as u32, COLOR_X),
    ("y", XINPUT_GAMEPAD_Y.0 as u32, COLOR_Y),
    ("lb", XINPUT_GAMEPAD_LEFT_SHOULDER.0 as u32, COLOR_OTHER),
    ("rb", XINPUT_GAMEPAD_RIGHT_SHOULDER.0 as u32, COLOR_OTHER),
    ("lt", LT, COLOR_OTHER),
    ("rt", RT, COLOR_OTHER),
    ("ls", XINPUT_GAMEPAD_LEFT_THUMB.0 as u32, COLOR_OTHER),
    ("rs", XINPUT_GAMEPAD_RIGHT_THUMB.0 as u32, COLOR_OTHER),
    ("up", XINPUT_GAMEPAD_DPAD_UP.0 as u32, COLOR_OTHER),
    ("down", XINPUT_GAMEPAD_DPAD_DOWN.0 as u32, COLOR_OTHER),
    ("left", XINPUT_GAMEPAD_DPAD_LEFT.0 as u32, COLOR_OTHER),
    ("right", XINPUT_GAMEPAD_DPAD_RIGHT.0 as u32, COLOR_OTHER),
    ("start", XINPUT_GAMEPAD_START.0 as u32, COLOR_OTHER),
    ("back", XINPUT_GAMEPAD_BACK.0 as u32, COLOR_OTHER),
];

/// Fixed-size ring buffer of per-frame input samples.
struct History {
    samples: Box<[u32]>,
    head: usize,
}

impl History {
    fn new(len: usize) -> Self {
        Self { samples: vec![0; len.max(1)].into_boxed_slice(), head: 0 }
    }

    fn push(&mut self, sample: u32) {
        self.samples[self.head] = sample;
        self.head = (self.head + 1) % self.samples.len();
    }

    /// Iterates from the oldest to the newest sample.
    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        let (newer, older) = self.samples.split_at(self.head);
        older.iter().chain(newer).copied()
    }

    fn clear(&mut self, mask: u32) {
        self.samples.iter_mut().for_each(|sample| *sample &= !mask);
    }
}

#[derive(Default, Clone, Copy)]
struct KeySlot {
    key: Option<imgui::Key>,
    last_down: u64,
}

pub(crate) struct InputViewer {
    label: String,
    hotkey: Option<Key>,
    enabled: bool,
    history: History,
    key_slots: [KeySlot; KEYBOARD_SLOTS],
    tick: u64,
}

impl InputViewer {
    /// `seconds` is approximated as 60 frames each.
    pub(crate) fn new(seconds: usize, hotkey: Option<Key>) -> Self {
        InputViewer {
            label: hotkey
                .as_ref()
                .map(|k| format!("Input viewer ({})", k))
                .unwrap_or_else(|| "Input viewer".to_string()),
            hotkey,
            enabled: true,
            history: History::new(seconds * 60),
            key_slots: Default::default(),
            tick: 0,
        }
    }

    fn sample(&mut self, ui: &imgui::Ui) {
        let gamepad = GAMEPAD_STATE.load().Gamepad;
        let threshold = XINPUT_GAMEPAD_TRIGGER_THRESHOLD.0 as u8;

        let mut sample = gamepad.wButtons.0 as u32;
        if gamepad.bLeftTrigger > threshold {
            sample |= LT;
        }
        if gamepad.bRightTrigger > threshold {
            sample |= RT;
        }

        self.tick += 1;
        let keyboard_keys = imgui::Key::VARIANTS
            .into_iter()
            .filter(|&k| k as u32 <= imgui::Key::KeypadEqual as u32);

        for key in keyboard_keys.filter(|&k| ui.is_key_down(k)) {
            let slot = self.key_slot(key);
            self.key_slots[slot].last_down = self.tick;
            sample |= KEYBOARD << slot;
        }

        self.history.push(sample);
    }

    /// Returns the slot holding `key`, evicting the least recently used one
    /// if the key doesn't have a slot yet.
    fn key_slot(&mut self, key: imgui::Key) -> usize {
        if let Some(slot) = self.key_slots.iter().position(|s| s.key == Some(key)) {
            return slot;
        }

        let (slot, _) = self.key_slots.iter().enumerate().min_by_key(|(_, s)| s.last_down).unwrap();
        self.key_slots[slot].key = Some(key);
        self.history.clear(KEYBOARD << slot);
        slot
    }
}

fn key_name(key: Option<imgui::Key>) -> &'static str {
    // SAFETY: ImGui returns a pointer to a static, nul-terminated string.
    key.and_then(|k| unsafe { CStr::from_ptr(imgui::sys::igGetKeyName(k as _)) }.to_str().ok())
        .unwrap_or("")
}

/// Draws `text` on the background draw list at `size` pixels, which imgui-rs
/// doesn't expose.
fn add_text_sized(_ui: &imgui::Ui, pos: [f32; 2], color: [f32; 4], size: f32, text: &str) {
    let color: imgui::ImColor32 = color.into();

    // SAFETY: a frame is being built, as witnessed by `_ui`, and the text range
    // is valid for the duration of the call.
    unsafe {
        imgui::sys::ImDrawList_AddText_FontPtr(
            imgui::sys::igGetBackgroundDrawList(),
            imgui::sys::igGetFont(),
            size,
            imgui::sys::ImVec2 { x: pos[0], y: pos[1] },
            color.to_bits(),
            text.as_ptr().cast(),
            text.as_ptr().add(text.len()).cast(),
            0.0,
            std::ptr::null(),
        );
    }
}

impl Widget for InputViewer {
    fn render(&mut self, ui: &imgui::Ui) {
        ui.checkbox(&self.label, &mut self.enabled);
    }

    fn render_closed(&mut self, ui: &imgui::Ui) {
        if !self.enabled {
            return;
        }

        // Bottom right corner, above the souls counter.
        let [dw, dh] = ui.io().display_size;
        let [right, bottom] = [dw * 0.95, dh * 0.88];
        let width = dw * 0.2;
        let left = right - width;
        let font_scale = 0.6;
        let row_h = ui.current_font_size() * font_scale;
        let bar_h = row_h * 0.7;
        let len = self.history.samples.len();
        let sample_w = width / len as f32;

        let keyboard_rows = self
            .key_slots
            .iter()
            .enumerate()
            .map(|(i, slot)| (key_name(slot.key), KEYBOARD << i, COLOR_KEYBOARD));
        let rows = GAMEPAD_ROWS.into_iter().chain(keyboard_rows);
        let top = bottom - row_h * (GAMEPAD_ROWS.len() + KEYBOARD_SLOTS) as f32;

        let draw_list = ui.get_background_draw_list();

        // Anchored to the buffer's slots, so the grid scrolls with the samples.
        let grid_offset = (GRID_FRAMES - self.history.head % GRID_FRAMES) % GRID_FRAMES;
        for i in (grid_offset..len).step_by(GRID_FRAMES) {
            let x = left + i as f32 * sample_w;
            draw_list.add_line([x, top], [x, bottom], COLOR_GRID).build();
        }

        for (row, (label, mask, color)) in rows.enumerate() {
            let y = top + row_h * (row as f32 + 0.5);
            let label_w = ui.calc_text_size(label)[0] * font_scale;

            add_text_sized(
                ui,
                [left - label_w - row_h * 0.5, y - row_h * 0.5],
                color,
                row_h,
                label,
            );
            draw_list.add_line([left, y], [right, y], color).build();

            // Trailing released sample closes a press still in progress.
            let mut press_start = None;
            for (i, sample) in self.history.iter().chain([0]).enumerate() {
                match (sample & mask != 0, press_start) {
                    (true, None) => press_start = Some(i),
                    (false, Some(start)) => {
                        draw_list
                            .add_rect(
                                [left + start as f32 * sample_w, y - bar_h * 0.5],
                                [left + i as f32 * sample_w, y + bar_h * 0.5],
                                color,
                            )
                            .filled(true)
                            .build();
                        press_start = None;
                    },
                    _ => {},
                }
            }
        }
    }

    // Sampling happens here as this runs every frame, regardless of UI state.
    fn interact(&mut self, ui: &imgui::Ui) {
        if self.hotkey.map(|k| k.is_pressed(ui)).unwrap_or(false) {
            self.action();
        }

        self.sample(ui);
    }

    fn action(&mut self) {
        self.enabled = !self.enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_order() {
        let mut history = History::new(3);
        assert_eq!(history.iter().collect::<Vec<_>>(), [0, 0, 0]);

        for sample in 1..=4 {
            history.push(sample);
        }
        assert_eq!(history.iter().collect::<Vec<_>>(), [2, 3, 4]);

        history.clear(0b10);
        assert_eq!(history.iter().collect::<Vec<_>>(), [0, 1, 4]);
    }
}
