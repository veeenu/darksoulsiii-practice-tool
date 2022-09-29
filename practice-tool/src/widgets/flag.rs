use libds3::memedit::Bitflag;

use super::Widget;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct Flag {
    label: String,
    bitflag: Bitflag<u8>,
    hotkey: KeyState,
}

impl Flag {
    pub(crate) fn new(label: &str, bitflag: Bitflag<u8>, hotkey: KeyState) -> Self {
        Flag { label: format!("{} ({})", label, hotkey), bitflag, hotkey }
    }
}

impl Widget for Flag {
    fn render(&mut self, ui: &imgui::Ui) {
        let state = self.bitflag.get();

        if let Some(mut state) = state {
            if ui.checkbox(&self.label, &mut state) {
                self.bitflag.set(state);
            }
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

    // fn interact_ui(&mut self) {
    //     // self.bitflag.toggle();
    // }
}
