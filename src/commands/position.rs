use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::Command;

pub(crate) struct PositionPointer {
  x: PointerChain<f32>,
  y: PointerChain<f32>,
  z: PointerChain<f32>,
  saved_x: f32,
  saved_y: f32,
  saved_z: f32,
  hotkey_load: Option<i32>,
  hotkey_save: Option<i32>,
}

impl PositionPointer {
  pub(crate) fn new(
    x: PointerChain<f32>,
    y: PointerChain<f32>,
    z: PointerChain<f32>,
    hotkey_load: Option<i32>,
    hotkey_save: Option<i32>,
  ) -> PositionPointer {
    PositionPointer {
      x,
      y,
      z,
      saved_x: 0.,
      saved_y: 0.,
      saved_z: 0.,
      hotkey_load,
      hotkey_save,
    }
  }

  fn read(&self) -> Option<(f32, f32, f32)> {
    if let (Some(x), Some(y), Some(z)) = (self.x.read(), self.y.read(), self.z.read()) {
      Some((x, y, z))
    } else {
      None
    }
  }

  fn save(&mut self) {
    if let Some((x, y, z)) = self.read() {
      self.saved_x = x;
      self.saved_y = y;
      self.saved_z = z;
    }
  }

  fn load(&self) {
    self.x.write(self.saved_x);
    self.y.write(self.saved_y);
    self.z.write(self.saved_z);
  }
}

impl Command for PositionPointer {
  fn display(&self, ui: &imgui::Ui) -> bool {
    let (sx, sy, sz) = (self.saved_x, self.saved_y, self.saved_z);
    let (cx, cy, cz) = self.read().unwrap_or((0.0, 0.0, 0.0));

    ui.text(ImString::new(format!(
      "Position [{:9.2}  {:9.2} {:9.2}] <- [{:9.2}  {:9.2} {:9.2}]",
      cx, cy, cz, sx, sy, sz
    )));

    false
  }

  fn interact(&mut self, ui: &imgui::Ui, is_active: bool, is_interacting: bool) {
    if self
      .hotkey_load
      .map(|k| ui.is_key_released(k as _))
      .unwrap_or(false)
    {
      self.load();
    }
    if self
      .hotkey_save
      .map(|k| ui.is_key_released(k as _))
      .unwrap_or(false)
    {
      self.save();
    }
  }

  fn is_valid(&self) -> bool {
    self.x.read().is_some()
  }
}
