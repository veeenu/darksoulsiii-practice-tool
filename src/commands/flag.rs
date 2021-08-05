use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;
use log::*;

use super::Command;
use crate::config::get_symbol;

pub(crate) struct FlagPointer {
  label: String,
  chain: PointerChain<u8>,
  bit: u8,
  hotkey: Option<i32>,
}

impl FlagPointer {
  pub(crate) fn new(
    label: &str,
    chain: PointerChain<u8>,
    bit: u8,
    hotkey: Option<i32>,
  ) -> FlagPointer {
    info!("Building flag pointer {}", label);
    FlagPointer {
      label: format!(
        "{} ({})",
        label,
        hotkey
          .and_then(get_symbol)
          .unwrap_or_else(|| "".to_string())
      ),
      chain,
      bit,
      hotkey,
    }
  }

  pub(crate) fn toggle(&self) {
    let mask = 1 << self.bit;
    if let Some(x) = self.chain.read() {
      self.chain.write(match x & mask {
        0 => x | mask,
        _ => x & (!mask),
      });
    }
  }

  pub(crate) fn get(&self) -> Option<bool> {
    self.chain.read().map(|x| (x & (1 << self.bit)) != 0)
  }
}

impl Command for FlagPointer {
  fn display(&mut self, ctx: &RenderContext) -> bool {
    let mut value = self.get().unwrap_or(false);

    if ctx.frame.checkbox(&ImString::new(&self.label), &mut value) {
      self.interact(ctx, true);
    }

    false
  }

  fn interact(&mut self, ctx: &RenderContext, is_interacting: bool) {
    let is_interacting = is_interacting && ctx.controller.pressed(|s| s.a);
    let hotkey_pressed = self
      .hotkey
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false);
    if is_interacting || hotkey_pressed {
      self.toggle();
    }
  }

  fn is_valid(&self) -> bool {
    self.get().is_some()
  }
}
