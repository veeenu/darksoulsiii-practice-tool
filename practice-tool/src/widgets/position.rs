use std::fmt::Write;

use libds3::memedit::PointerChain;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::nudge_position::NudgePositionStorage;
use practice_tool_core::widgets::position::{Position, PositionStorage};
use practice_tool_core::widgets::Widget;

pub(super) struct SavePosition {
    ptr_angle: PointerChain<f32>,
    ptr_pos: PointerChain<[f32; 3]>,
    saved_position: [f32; 4],
    label_current: String,
    label_stored: String,
    valid: bool,
    nudge: f32,
}

impl SavePosition {
    pub(super) fn new(ptr: (PointerChain<f32>, PointerChain<[f32; 3]>), nudge: f32) -> Self {
        Self {
            ptr_angle: ptr.0,
            ptr_pos: ptr.1,
            saved_position: [0.0; 4],
            label_current: String::new(),
            label_stored: String::new(),
            valid: false,
            nudge,
        }
    }
}

impl PositionStorage for SavePosition {
    fn read(&mut self) {
        if let (Some(pos), Some(angle)) = (self.ptr_pos.read(), self.ptr_angle.read()) {
            self.saved_position = [pos[0], pos[1], pos[2], angle];
            self.valid = true;
        } else {
            self.valid = false;
        }
    }

    fn write(&mut self) {
        self.ptr_pos.write([
            self.saved_position[0],
            self.saved_position[1],
            self.saved_position[2],
        ]);
        self.ptr_angle.write(self.saved_position[3]);
    }

    fn display_current(&mut self) -> &str {
        self.label_current.clear();

        let pos = self.ptr_pos.read();
        let angle = self.ptr_angle.read();

        let (read_pos, valid) = if let (Some(pos), Some(angle)) = (pos, angle) {
            ([pos[0], pos[1], pos[2], angle], true)
        } else {
            ([0f32; 4], false)
        };

        self.valid = valid;

        write!(
            self.label_current,
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            read_pos[0], read_pos[1], read_pos[2], read_pos[3]
        )
        .ok();

        &self.label_current
    }

    fn display_stored(&mut self) -> &str {
        self.label_stored.clear();

        let [x, y, z, a] = self.saved_position;

        write!(self.label_stored, "{:7.1} {:7.1} {:7.1} {:7.1}", x, y, z, a).ok();

        &self.label_stored
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl NudgePositionStorage for SavePosition {
    fn nudge_up(&mut self) {
        if let Some([x, y, z]) = self.ptr_pos.read() {
            self.ptr_pos.write([x, y + self.nudge, z]);
        }
    }

    fn nudge_down(&mut self) {
        if let Some([x, y, z]) = self.ptr_pos.read() {
            self.ptr_pos.write([x, y - self.nudge, z]);
        }
    }
}

pub(crate) fn save_position(
    ptr: (PointerChain<f32>, PointerChain<[f32; 3]>),
    key_load: Option<Key>,
    key_save: Option<Key>,
) -> Box<dyn Widget> {
    Box::new(Position::new(SavePosition::new(ptr, 0.0), key_load, key_save))
}
