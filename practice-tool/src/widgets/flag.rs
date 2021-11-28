use crate::memedit::Bitflag;
use crate::util::KeyState;

use super::Widget;

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

impl Widget for Flag {
    fn render(&self, ui: &imgui::Ui) {
        let state = self.bitflag.get();

        if let Some(mut state) = state {
            ui.checkbox(&self.label, &mut state);
        } else {
            let token = ui.begin_disabled(true);
            ui.checkbox(&self.label, &mut false);
            token.end();
        }
    }

    fn interact(&mut self) {
        if self.hotkey.keyup() {
            self.bitflag.toggle();
        }
    }
}
