use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::{BUTTON_WIDTH, BUTTON_HEIGHT, Command};

pub(crate) struct SoulsPointer{
  pointer: PointerChain<u32>,
  hotkey: Option<i32>
}

impl SoulsPointer {
  pub(crate) fn new( pointer: PointerChain<u32>, hotkey: Option<i32>) -> SoulsPointer {
    SoulsPointer { pointer, hotkey }
  }

  pub(crate) fn incr(&self) {
    if let Some(cur_souls) = self.pointer.read() {
      self.pointer.write(cur_souls + 10000);
    }
  }
}

impl Command for SoulsPointer {
  fn display(&self, ui: &imgui::Ui) -> bool {
    let clicked = ui.button(
      &ImString::new("Increase souls"),
      [BUTTON_WIDTH, BUTTON_HEIGHT],
    );
    ui.same_line(0.);
    if let Some(souls) = self.pointer.read() {
      ui.text(ImString::new(format!("[{}]", souls)));
    } else {
      ui.text(ImString::new("[N/A]"));
    }
    clicked
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if (is_active && is_interacting)
      || self
        .hotkey
        .map(|k| ui.is_key_released(k as _))
        .unwrap_or(false)
    {
      self.incr();
    }
  }

  fn is_valid(&self) -> bool {
    self.pointer.eval().is_some()
  }
}
