use std::mem;

use serde::Deserialize;

use super::Widget;
use crate::util::KeyState;

#[derive(Deserialize, Debug, Clone, Copy)]
pub(crate) enum OpenMenuKind {
    #[serde(rename = "travel")]
    Travel,
    #[serde(rename = "attune")]
    Attune,
}

#[derive(Debug)]
pub(crate) struct OpenMenu {
    kind: OpenMenuKind,
    travel_ptr: usize,
    attune_ptr: usize,
    hotkey: Option<KeyState>,
}

impl OpenMenu {
    pub(crate) fn new(
        kind: OpenMenuKind,
        travel_ptr: usize,
        attune_ptr: usize,
        hotkey: Option<KeyState>,
    ) -> Self {
        Self { kind, travel_ptr, attune_ptr, hotkey }
    }

    fn call(&self) {
        let ptr = match self.kind {
            OpenMenuKind::Travel => self.travel_ptr,
            OpenMenuKind::Attune => self.attune_ptr,
        };

        let stack_space = [0u8; 0x48];

        let func: extern "system" fn(*const u8) = unsafe { mem::transmute(ptr) };
        func(stack_space.as_ptr());
    }
}

impl Widget for OpenMenu {
    fn render(&mut self, ui: &imgui::Ui) {
        let scale = super::scaling_factor(ui);
        let label = match self.kind {
            OpenMenuKind::Travel => "Warp menu",
            OpenMenuKind::Attune => "Attune menu",
        };

        if ui.button_with_size(label, [super::BUTTON_WIDTH * scale, super::BUTTON_HEIGHT]) {
            self.call();
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if ui.is_any_item_active() {
            return;
        }

        if self.hotkey.map(|k| k.keyup(ui)).unwrap_or(false) {
            self.call();
        }
    }
}
