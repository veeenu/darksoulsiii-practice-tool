use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;
use log::*;

use super::Command;

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
      label: String::from(label),
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
  fn display(&self, ui: &imgui::Ui) -> bool {
    let mut value = self.get().unwrap_or(false);

    ui.checkbox(&ImString::new(&self.label), &mut value);

    ui.is_item_clicked(imgui::MouseButton::Left)
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if (is_active && is_interacting)
      || self
        .hotkey
        .map(|k| ui.is_key_released(k as _))
        .unwrap_or(false)
    {
      self.toggle();
    }
  }

  fn is_valid(&self) -> bool {
    self.get().is_some()
  }
}
