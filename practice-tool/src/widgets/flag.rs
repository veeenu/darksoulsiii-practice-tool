use crate::memedit::Bitflag;
use crate::style::StyleState;
use crate::util::KeyState;

use super::Command;

#[derive(Debug)]
pub(crate) struct Flag {
    label: String,
    bitflag: Bitflag<u8>,
    hotkey: KeyState,
}

impl Flag {
    pub(crate) fn new(label: &str, bitflag: Bitflag<u8>, hotkey: KeyState) -> Self {
        Flag {
            label: format!("{} ({})", label, hotkey),
            bitflag,
            hotkey,
        }
    }
}

impl Command for Flag {
    fn render(&self, ui: &imgui::Ui) {
        if self.hotkey.keyup() {
            self.bitflag.toggle();
        }

        let state = self.bitflag.get();

        if let Some(mut state) = state {
            let token = StyleState::InactiveValid.get_style_token(ui);
            ui.checkbox(&self.label, &mut state);
            token.pop();
        } else {
            let token = StyleState::InactiveInvalid.get_style_token(ui);
            ui.checkbox(&self.label, &mut false);
            token.pop();
        }
    }
}