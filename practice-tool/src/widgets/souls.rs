use libds3::memedit::PointerChain;

use super::Widget;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct Souls {
    label: String,
    ptr: PointerChain<u32>,
    hotkey: KeyState,
    amount: u32,
}

impl Souls {
    pub(crate) fn new(amount: u32, ptr: PointerChain<u32>, hotkey: KeyState) -> Self {
        Souls { label: format!("Add {} Souls ({})", amount, hotkey), ptr, hotkey, amount }
    }

    fn add(&self) -> Option<u32> {
        let cur_souls = self.ptr.read();

        cur_souls.map(|souls| {
            self.ptr.write(souls + self.amount);
            souls + self.amount
        })
    }
}

impl Widget for Souls {
    fn render(&mut self, ui: &imgui::Ui) {
        let scale = super::scaling_factor(ui);
        let souls = self.ptr.read();
        let _token = ui.begin_disabled(souls.is_none());

        if ui.button_with_size(&self.label, [super::BUTTON_WIDTH * scale, super::BUTTON_HEIGHT]) {
            self.add();
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if self.hotkey.keyup(ui) {
            self.add();
        }
    }
}
