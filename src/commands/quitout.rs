use hudhook::memory::PointerChain;
use hudhook::*;

use log::*;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

pub(crate) struct QuitoutPointer {
  pointer: PointerChain<u8>,
  hotkey: Option<i32>,
  label: imgui::ImString,
}

impl QuitoutPointer {
  pub(crate) fn new(pointer: PointerChain<u8>, hotkey: Option<i32>) -> QuitoutPointer {
    let label = imgui::ImString::new(format!(
      "Quitout ({})",
      hotkey.and_then(get_symbol).unwrap_or("".to_string())
    ));
    QuitoutPointer {
      pointer,
      hotkey,
      label,
    }
  }

  pub(crate) fn is_valid(&self) -> bool {
    self.pointer.eval().is_some()
  }

  pub(crate) fn quitout(&self) {
    if self.pointer.write(1).is_none() {
      error!("Error writing quitout pointer");
    }
  }
}

impl Command for QuitoutPointer {
  fn display(&mut self, ui: &imgui::Ui) -> bool {
    ui.button(&self.label, [BUTTON_WIDTH, BUTTON_HEIGHT])
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
