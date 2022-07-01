use super::Widget;
use crate::memedit::PointerChain;
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
        Souls { label: format!("Souls ({})", hotkey), ptr, hotkey, amount }
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
        let souls = self.ptr.read();
        let _token = ui.begin_disabled(souls.is_none());

        if ui.button_with_size(&self.label, [super::BUTTON_WIDTH, super::BUTTON_HEIGHT]) {
            self.add();
        }
        ui.same_line();

        if let Some(souls) = souls {
            ui.text(format!("[{:>10}]", souls));
        } else {
            ui.text("[          ]");
        }
    }

    fn interact(&mut self) {
        if self.hotkey.keyup() {
            self.add();
        }
    }
}
