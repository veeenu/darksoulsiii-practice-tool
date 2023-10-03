use libds3::prelude::*;

use super::Widget;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct NudgePosition {
    position: PointerChain<[f32; 3]>,
    nudge: f32,
    nudge_up: KeyState,
    nudge_down: KeyState,
    nudge_up_label: String,
    nudge_down_label: String,
}

impl NudgePosition {
    pub(crate) fn new(
        position: PointerChain<[f32; 3]>,
        nudge: f32,
        nudge_up: KeyState,
        nudge_down: KeyState,
    ) -> Self {
        let nudge_up_label = format!("Nudge up ({})", nudge_up);
        let nudge_down_label = format!("Nudge down ({})", nudge_down);
        NudgePosition { position, nudge, nudge_up, nudge_down, nudge_up_label, nudge_down_label }
    }

    fn do_nudge_up(&mut self) {
        if let Some([x, y, z]) = self.position.read() {
            self.position.write([x, y + self.nudge, z]);
        }
    }

    fn do_nudge_down(&mut self) {
        if let Some([x, y, z]) = self.position.read() {
            self.position.write([x, y - self.nudge, z]);
        }
    }
}

impl Widget for NudgePosition {
    fn render(&mut self, ui: &imgui::Ui) {
        let valid = self.position.eval().is_some();
        let _token = ui.begin_disabled(!valid);

        let button_width = super::BUTTON_WIDTH * super::scaling_factor(ui);

        if ui
            .button_with_size(&self.nudge_up_label, [button_width * 0.5 - 4., super::BUTTON_HEIGHT])
        {
            self.do_nudge_up();
        }
        ui.same_line();
        if ui.button_with_size(&self.nudge_down_label, [
            button_width * 0.5 - 4.,
            super::BUTTON_HEIGHT,
        ]) {
            self.do_nudge_down();
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if ui.is_any_item_active() {
            return;
        }

        if self.nudge_up.is_key_down(ui) {
            self.do_nudge_up();
        } else if self.nudge_down.is_key_down(ui) {
            self.do_nudge_down();
        }
    }
}
