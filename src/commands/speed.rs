use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::{Command, BUTTON_HEIGHT, BUTTON_WIDTH};
use crate::config::get_symbol;

pub(crate) struct CycleSpeedPointer {
  pointer: PointerChain<f32>,
  hotkey: Option<i32>,
  label: imgui::ImString,
}

impl CycleSpeedPointer {
  pub(crate) fn new(pointer: PointerChain<f32>, hotkey: Option<i32>) -> CycleSpeedPointer {
    let label = imgui::ImString::new(format!(
      "Speed ({})",
      hotkey.and_then(get_symbol).unwrap_or("".to_string())
    ));
    CycleSpeedPointer {
      pointer,
      hotkey,
      label,
    }
  }

  pub(crate) fn cycle(&self) -> Option<f32> {
    let next = match self.pointer.read() {
      Some(x) if x <= 0.25 => Some(0.5),
      Some(x) if x <= 0.5 => Some(1.),
      Some(x) if x <= 1. => Some(2.),
      Some(x) if x <= 2. => Some(4.),
      Some(x) if x <= 4. => Some(8.),
      Some(_) => Some(0.25),
      None => None,
    };
    next.map(|speed| self.pointer.write(speed));
    next
  }

  pub(crate) fn read(&self) -> Option<f32> {
    self.pointer.read()
  }
}

impl Command for CycleSpeedPointer {
  fn display(&mut self, ui: &imgui::Ui) -> bool {
    let clicked = ui.button(&self.label, [BUTTON_WIDTH, BUTTON_HEIGHT]);
    ui.same_line(0.);
    if let Some(speed) = self.read() {
      ui.text(ImString::new(format!("[{:3.2}x]", speed)));
    } else {
      ui.text(ImString::new("[ N/A ]"));
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
      self.cycle();
    }
  }

  fn is_valid(&self) -> bool {
    self.read().is_some()
  }
}
