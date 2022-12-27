use libds3::memedit::PointerChain;

use super::Widget;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct SavePosition {
    ptr_angle: PointerChain<f32>,
    ptr_pos: PointerChain<[f32; 3]>,
    hotkey: KeyState,
    modifier: KeyState,
    saved_position: [f32; 4],
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
            saved_position: [0f32; 4],
        }
    }

    fn save_position(&mut self) {
        if let (Some(pos), Some(angle)) = (self.ptr_pos.read(), self.ptr_angle.read()) {
            self.saved_position = [pos[0], pos[1], pos[2], angle];
        }
    }

    fn load_position(&mut self) {
        self.ptr_pos.write([
            self.saved_position[0],
            self.saved_position[1],
            self.saved_position[2],
        ]);
        self.ptr_angle.write(self.saved_position[3]);
    }
}

impl Widget for SavePosition {
    fn render(&mut self, ui: &imgui::Ui) {
        let pos = self.ptr_pos.read();
        let angle = self.ptr_angle.read();
        let saved_pos = self.saved_position;

        let (read_pos, valid) = if let (Some(pos), Some(angle)) = (pos, angle) {
            ([pos[0], pos[2], pos[1], angle], true)
        } else {
            ([0f32; 4], false)
        };

        let _token = ui.begin_disabled(!valid);
        let button_width = super::BUTTON_WIDTH * super::scaling_factor(ui);

        if ui.button_with_size(format!("Load ({})", self.hotkey), [
            button_width * 0.33 - 4.,
            super::BUTTON_HEIGHT,
        ]) {
            self.load_position();
        }
        ui.same_line();
        if ui.button_with_size(format!("Save ({} + {})", self.modifier, self.hotkey), [
            button_width * 0.67 - 4.,
            super::BUTTON_HEIGHT,
        ]) {
            self.save_position();
        }
        ui.text(format!(
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            read_pos[0], read_pos[1], read_pos[2], read_pos[3]
        ));
        ui.text(format!(
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            saved_pos[0], saved_pos[1], saved_pos[2], saved_pos[3],
        ));
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        let key_up = self.hotkey.keyup(ui);
        let mod_down = self.modifier.is_key_down(ui);

        if key_up && mod_down {
            self.save_position();
        } else if key_up {
            self.load_position();
        }
    }
}
