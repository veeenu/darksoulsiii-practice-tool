use super::Widget;
use crate::memedit::PointerChain;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct Quitout {
    label: String,
    ptr: PointerChain<u8>,
    hotkey: KeyState,
}

impl Quitout {
    pub(crate) fn new(ptr: PointerChain<u8>, hotkey: KeyState) -> Self {
        Quitout { label: format!("Quitout ({})", hotkey), ptr, hotkey }
    }
}

impl Widget for Quitout {
    fn render(&mut self, ui: &imgui::Ui) {
        ui.button_with_size(&self.label, [super::BUTTON_WIDTH, super::BUTTON_HEIGHT]);
    }

    fn interact(&mut self) {
        if self.hotkey.keyup() {
            self.ptr.write(1);
        }
    }

    // fn interact_ui(&mut self) {
    //     self.ptr.write(1);
    // }
}
