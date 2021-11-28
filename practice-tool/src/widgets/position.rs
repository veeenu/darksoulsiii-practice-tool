use parking_lot::Mutex;

use crate::memedit::PointerChain;
use crate::pointers::Position;
use crate::style::StyleState;
use crate::util::KeyState;

use super::Command;

#[derive(Debug)]
pub(crate) struct SavePosition {
    label: String,
    ptr: PointerChain<Position>,
    hotkey: KeyState,
    modifier: KeyState,
    saved_position: Mutex<Position>,
}

impl SavePosition {
    pub(crate) fn new(ptr: PointerChain<Position>, hotkey: KeyState, modifier: KeyState) -> Self {
        SavePosition {
            label: format!("Position ({})", hotkey),
            ptr,
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

        let pos = self.ptr.read();

        if let Some(ref pos) = pos {
            if key_up && mod_down {
                // Save
                *self.saved_position.lock() = pos.clone();
            } else if key_up {
                // Load
                let mut saved_pos = (*self.saved_position.lock()).clone();
                saved_pos.unknown = pos.unknown;
                self.ptr.write(saved_pos);
            }
        }

        if let Some(pos) = pos {
            let token = StyleState::InactiveValid.get_style_token(ui);
            ui.text(format!(
                "{} [{:>3.2} {:>3.2} {:>3.2} {:>3.2}]",
                self.label, pos.x, pos.y, pos.z, pos.w
            ));
            token.pop();
        } else {
            let token = StyleState::InactiveInvalid.get_style_token(ui);
            ui.text(&self.label);
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
