use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::Command;
use crate::Context;
use crate::config::get_symbol;

pub(crate) struct CycleSpeedPointer {
  pointer: PointerChain<f32>,
  hotkey: Option<i32>,
  label: imgui::ImString,
  values: Vec<f32>,
}

impl CycleSpeedPointer {
  pub(crate) fn new(
    pointer: PointerChain<f32>,
    hotkey: Option<i32>,
    values: Option<Vec<f32>>,
  ) -> CycleSpeedPointer {
    let label = imgui::ImString::new(format!(
      "Speed ({})",
      hotkey.and_then(get_symbol).unwrap_or("".to_string())
    ));
    CycleSpeedPointer {
      pointer,
      hotkey,
      label,
      values: values.unwrap_or_else(|| vec![1.0, 2.0, 4.0]),
    }
  }

  pub(crate) fn cycle(&self) -> Option<f32> {
    /*let next = match self.pointer.read() {
    Some(x) if x <= 0.25 => Some(0.5),
    Some(x) if x <= 0.5 => Some(1.),
    Some(x) if x <= 1. => Some(2.),
    Some(x) if x <= 2. => Some(4.),
    Some(x) if x <= 4. => Some(8.),
    Some(_) => Some(0.25),
    None => None,
    };*/
    let next = self.pointer.read().map(|speed| {
      *self
        .values
        .iter()
        .find(|&&x| x > speed)
        .unwrap_or_else(|| self.values.iter().next().unwrap_or(&1.0))
    });
    next.map(|speed| self.pointer.write(speed));
    next
  }

  pub(crate) fn read(&self) -> Option<f32> {
    self.pointer.read()
  }
}

impl Command for CycleSpeedPointer {
  fn display(&mut self, ctx: &Context<'_>) -> bool {
    let ui = ctx.frame;
    let clicked = ui.button(&self.label);
    ui.same_line();
    if let Some(speed) = self.read() {
      ui.text(ImString::new(format!("[{:3.2}x]", speed)));
    } else {
      ui.text(ImString::new("[ N/A ]"));
    }

    if clicked {
      self.interact(ctx, true);
    }

    clicked
  }

  fn interact(&mut self, ctx: &Context<'_>, is_interacting: bool) {
    let is_interacting = is_interacting && ctx.controller.pressed(|s| s.a);
    let hotkey_pressed = self
      .hotkey
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false);
    if is_interacting || hotkey_pressed {
      self.cycle();
    }
  }

  fn is_valid(&self) -> bool {
    self.read().is_some()
  }
}
