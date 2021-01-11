use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

pub(crate) struct SoulsPointer {
  pointer: PointerChain<u32>,
  quantity: i32,
  hotkey: Option<i32>,
  label: imgui::ImString,
}

impl SoulsPointer {
  pub(crate) fn new(pointer: PointerChain<u32>, quantity: i32, hotkey: Option<i32>) -> SoulsPointer {
    let label = imgui::ImString::new(format!("Add souls ({})", hotkey.and_then(get_symbol).unwrap_or("".to_string())));
    SoulsPointer { pointer, quantity, hotkey, label }
  }

  pub(crate) fn incr(&self) {
    if let Some(cur_souls) = self.pointer.read() {
      let cur_souls = cur_souls as i32;
      let new_souls = cur_souls + self.quantity;
      self.pointer.write(new_souls as _);
    }
  }
}

impl Command for SoulsPointer {
  fn display(&self, ui: &imgui::Ui) -> bool {
    let clicked = ui.button(
      &self.label,
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
