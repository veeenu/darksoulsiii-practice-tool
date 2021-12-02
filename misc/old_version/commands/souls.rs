use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::Command;
use crate::Context;
use crate::config::get_symbol;

pub(crate) struct SoulsPointer {
  pointer: PointerChain<u32>,
  quantity: i32,
  hotkey: Option<i32>,
  label: imgui::ImString,
}

impl SoulsPointer {
  pub(crate) fn new(
    pointer: PointerChain<u32>,
    quantity: i32,
    hotkey: Option<i32>,
  ) -> SoulsPointer {
    let label = imgui::ImString::new(format!(
      "Add souls ({})",
      hotkey.and_then(get_symbol).unwrap_or("".to_string())
    ));
    SoulsPointer {
      pointer,
      quantity,
      hotkey,
      label,
    }
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
  fn display(&mut self, ctx: &Context<'_>) -> bool {
    let ui = ctx.frame;
    let clicked = ui.button(&self.label);
    ui.same_line();
    if let Some(souls) = self.pointer.read() {
      ui.text(ImString::new(format!("[{}]", souls)));
    } else {
      ui.text(ImString::new("[N/A]"));
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
      self.incr();
    }
  }

  fn is_valid(&self) -> bool {
    self.pointer.eval().is_some()
  }
}
