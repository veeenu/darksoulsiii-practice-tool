use crate::memedit::Bitflag;
use crate::style::StyleState;
use crate::util::KeyState;

pub(crate) struct Flag {
    bitflag: Bitflag<u8>,
    hotkey: KeyState,
}

impl Flag {
    pub(crate) fn new(bitflag: Bitflag<u8>, hotkey: i32) -> Self {
        Flag {
            bitflag,
            hotkey: KeyState::new(hotkey),
        }
    }

    pub(crate) fn render(&self, ui: &imgui::Ui) {
        if self.hotkey.keyup() {
            self.bitflag.toggle();
        }

        let state = self.bitflag.get();

        if let Some(mut state) = state {
            let token = StyleState::InactiveValid.get_style_token(ui);
            ui.checkbox("test", &mut state);
            token.pop();
        } else {
            let token = StyleState::InactiveInvalid.get_style_token(ui);
            ui.checkbox("test", &mut false);
            token.pop();
        }
    }
}
