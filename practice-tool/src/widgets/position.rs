use parking_lot::Mutex;

use crate::memedit::PointerChain;
use crate::util::KeyState;

use super::Widget;

#[derive(Debug)]
pub(crate) struct SavePosition {
    ptr_angle: PointerChain<f32>,
    ptr_pos: PointerChain<[f32; 3]>,
    hotkey: KeyState,
    modifier: KeyState,
    saved_position: Mutex<[f32; 4]>,
}

impl SavePosition {
    pub(crate) fn new(
        ptr: (PointerChain<f32>, PointerChain<[f32; 3]>),
        hotkey: KeyState,
        modifier: KeyState,
    ) -> Self {
        SavePosition {
            ptr_angle: ptr.0,
            ptr_pos: ptr.1,
            hotkey,
            modifier,
            saved_position: Default::default(),
        }
    }
}

impl Widget for SavePosition {
    fn render(&mut self, ui: &imgui::Ui) {
        let pos = self.ptr_pos.read();
        let angle = self.ptr_angle.read();
        let saved_pos = *self.saved_position.lock();

        let (read_pos, valid) = if let (Some(pos), Some(angle)) = (pos, angle) {
            ([pos[0], pos[2], pos[1], angle,], true)
        } else {
            ([0f32; 4], false)
        };

        let _token = ui.begin_disabled(valid);
        ui.text(format!(
            "Position [{:9.2}  {:9.2}  {:9.2}  {:9.2}]",
            read_pos[0], read_pos[1], read_pos[2], read_pos[3]
        ));
        ui.same_line();
        ui.text(format!("Load ({})", self.hotkey));
        ui.text(format!(
            "         [{:9.2}  {:9.2}  {:9.2}  {:9.2}]",
            saved_pos[1], saved_pos[3], saved_pos[2], saved_pos[0],
        ));
        ui.same_line();
        ui.text(format!("Save ({} + {})", self.modifier, self.hotkey));
    }

    fn interact(&mut self) {
        let key_up = self.hotkey.keyup();
        let mod_down = self.modifier.is_key_down();

        if key_up && mod_down {
            if let (Some(pos), Some(angle)) = (self.ptr_pos.read(), self.ptr_angle.read()) {
                *self.saved_position.lock() = [angle, pos[0], pos[1], pos[2]];
            }
        } else if key_up {
            let saved_pos = *self.saved_position.lock();
            self.ptr_pos
                .write([saved_pos[1], saved_pos[2], saved_pos[3]]);
            self.ptr_angle.write(saved_pos[0]);
        }
    }
}
