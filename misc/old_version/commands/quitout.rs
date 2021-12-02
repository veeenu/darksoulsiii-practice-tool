use hudhook::memory::PointerChain;
use hudhook::*;

use log::*;

use super::Command;
use crate::Context;
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
      hotkey.and_then(get_symbol).unwrap_or_else(|| "".to_string())
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
  fn display(&mut self, ctx: &Context<'_>) -> bool {
    if ctx.frame.button(&self.label) {
      self.interact(ctx, true);
      true
    } else {
      false
    }
  }

  fn interact(&mut self, ctx: &Context<'_>, is_interacting: bool) {
    let is_interacting = is_interacting && ctx.controller.pressed(|s| s.a);
    let hotkey_pressed = self
      .hotkey
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false);
    if is_interacting || hotkey_pressed {
      self.quitout()
    }
  }

  fn is_valid(&self) -> bool {
    QuitoutPointer::is_valid(self)
  }
}
