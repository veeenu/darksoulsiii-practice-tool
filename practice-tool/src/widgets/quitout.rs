use crate::memedit::PointerChain;
use crate::util::KeyState;

use super::Command;

#[derive(Debug)]
pub(crate) struct Quitout {
    label: String,
    ptr: PointerChain<u8>,
    hotkey: KeyState,
}

impl Quitout {
    pub(crate) fn new(ptr: PointerChain<u8>, hotkey: KeyState) -> Self {
        Quitout {
            label: format!("Quitout ({})", hotkey),
            ptr,
            hotkey,
        }
    }
}

impl Command for Quitout {
    fn render(&self, ui: &imgui::Ui) {
        if self.hotkey.keyup() {
            self.ptr.write(1);
        }

        ui.text(&self.label);
    }
}
