use std::cmp::Ordering;

use crate::memedit::PointerChain;
use crate::util::KeyState;

use super::Widget;

#[derive(Debug)]
pub(crate) struct CycleSpeed {
    label: String,
    ptr: PointerChain<f32>,
    hotkey: KeyState,
    values: Vec<f32>,
}

impl CycleSpeed {
    pub(crate) fn new(values: &[f32], ptr: PointerChain<f32>, hotkey: KeyState) -> Self {
        let mut values = values.to_vec();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        CycleSpeed {
            label: format!("Speed ({})", hotkey),
            ptr,
            hotkey,
            values,
        }
    }

    fn cycle(&self) -> Option<f32> {
        let next = self.ptr.read().map(|speed| {
            *self
                .values
                .iter()
                .find(|&&x| x > speed)
                .unwrap_or_else(|| self.values.iter().next().unwrap_or(&1.0))
        });
        next.map(|speed| self.ptr.write(speed));
        next
    }
}

impl Widget for CycleSpeed {
    fn render(&self, ui: &imgui::Ui) {
        if let Some(speed) = self.ptr.read() {
            ui.text(format!("{} [{:4.2}]", self.label, speed));
        } else {
            ui.text_disabled(format!("{} [    ]", self.label));
        }
    }

    fn interact(&mut self) {
        if self.hotkey.keyup() {
            self.cycle();
        }
    }
}