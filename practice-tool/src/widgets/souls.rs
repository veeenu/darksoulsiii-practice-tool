use crate::memedit::PointerChain;
use crate::util::KeyState;

use super::Widget;

#[derive(Debug)]
pub(crate) struct Souls {
    label: String,
    ptr: PointerChain<u32>,
    hotkey: KeyState,
    amount: u32,
}

impl Souls {
    pub(crate) fn new(amount: u32, ptr: PointerChain<u32>, hotkey: KeyState) -> Self {
        Souls {
            label: format!("Souls ({})", hotkey),
            ptr,
            hotkey,
            amount,
        }
    }

    fn add(&self) -> Option<u32> {
        let cur_souls = self.ptr.read();

        cur_souls.and_then(|souls| {
            self.ptr.write(souls + self.amount);
            Some(souls + self.amount)
        })
    }
}

impl Widget for Souls {
    fn render(&self, ui: &imgui::Ui) {
        if let Some(souls) = self.ptr.read() {
            ui.text(format!("{} [{:>10}]", self.label, souls));
        } else {
            ui.text_disabled(format!("{} [          ]", self.label));
        }
    }

    fn interact(&mut self) {
        if self.hotkey.keyup() {
            self.add();
        }
    }
}
