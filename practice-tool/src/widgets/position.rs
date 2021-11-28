use parking_lot::Mutex;

use crate::memedit::PointerChain;
use crate::style::StyleState;
use crate::util::KeyState;

use super::Command;

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

impl Command for SavePosition {
    fn render(&self, ui: &imgui::Ui) {
        let key_up = self.hotkey.keyup();
        let mod_down = self.modifier.is_key_down();

        let pos = self.ptr_pos.read();
        let angle = self.ptr_angle.read();

        if let (Some(pos), Some(angle)) = (pos, angle) {
            let saved_pos = if key_up && mod_down {
                // Save
                let sp = [angle, pos[0], pos[1], pos[2]];
                *self.saved_position.lock() = sp;
                sp
            } else if key_up {
                // Load
                let saved_pos = (*self.saved_position.lock()).clone();
                self.ptr_pos
                    .write([saved_pos[1], saved_pos[2], saved_pos[3]]);
                self.ptr_angle.write(saved_pos[0]);
                saved_pos
            } else {
                *self.saved_position.lock()
            };

            let token = StyleState::InactiveValid.get_style_token(ui);
            ui.text(format!(
              "Position [{:9.2}  {:9.2}  {:9.2}  {:9.2}] ({})\n         [{:9.2}  {:9.2}  {:9.2}  {:9.2}] ({})",
                pos[0], pos[2], pos[1], angle, self.hotkey,
                saved_pos[1], saved_pos[3], saved_pos[2], saved_pos[0], self.modifier,
            ));
            token.pop();
        } else {
            let token = StyleState::InactiveInvalid.get_style_token(ui);
            ui.text("Position unavailable");
            token.pop();
        }

        // if let Some(mut state) = state {
        //     let token = StyleState::InactiveValid.get_style_token(ui);
        //     ui.checkbox(&self.label, &mut state);
        //     token.pop();
        // } else {
        //     let token = StyleState::InactiveInvalid.get_style_token(ui);
        //     ui.checkbox(&self.label, &mut false);
        //     token.pop();
        // }
    }
}
