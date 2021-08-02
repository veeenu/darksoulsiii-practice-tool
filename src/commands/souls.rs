use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::Command;
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
  fn display(&mut self, ctx: &RenderContext) -> bool {
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

  fn interact(&mut self, ctx: &RenderContext, is_interacting: bool) {
    //if (is_active && is_interacting)
    //  || self
    if self
        .hotkey
        .map(|k| ctx.frame.is_key_index_released(k as _))
        .unwrap_or(false)
      || (self.is_valid() && is_interacting)
    {
      self.incr();
    }
  }

  fn is_valid(&self) -> bool {
    self.pointer.eval().is_some()
  }
}
