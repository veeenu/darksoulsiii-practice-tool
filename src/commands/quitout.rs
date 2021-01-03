use hudhook::memory::PointerChain;
use hudhook::*;

use imgui;
use log::*;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};

pub(crate) struct QuitoutPointer {
  pointer: PointerChain<u8>,
  hotkey: Option<i32>,
}

impl QuitoutPointer {
  pub(crate) fn new( pointer: PointerChain<u8>, hotkey: Option<i32>) -> QuitoutPointer {
    QuitoutPointer { pointer, hotkey }
  }

  pub(crate) fn is_valid(&self) -> bool {
    self.pointer.eval().is_some()
  }

  pub(crate) fn quitout(&self) {
    if let None = self.pointer.write(1) {
      error!("Error writing quitout pointer");
    }
  }
}

impl Command for QuitoutPointer {
  fn display(&self, ui: &imgui::Ui) -> bool {
    ui.button(
      &imgui::ImString::new("Quitout"),
      [BUTTON_WIDTH, BUTTON_HEIGHT],
    )
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if (is_active && is_interacting)
      || self
        .hotkey
        .map(|k| ui.is_key_released(k as _))
        .unwrap_or(false)
    {
      self.quitout()
    }
  }

  fn is_valid(&self) -> bool {
    QuitoutPointer::is_valid(self)
  }
}
