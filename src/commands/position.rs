use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::ImString;

use super::Command;
use crate::config::get_symbol;

pub(crate) struct PositionPointer {
  x: PointerChain<f32>,
  y: PointerChain<f32>,
  z: PointerChain<f32>,
  a: PointerChain<f32>,
  saved_x: f32,
  saved_y: f32,
  saved_z: f32,
  saved_a: f32,
  hotkey_load: Option<i32>,
  hotkey_save: Option<i32>,
  load_label: ImString,
  save_label: ImString,
}

impl PositionPointer {
  pub(crate) fn new(
    x: PointerChain<f32>,
    y: PointerChain<f32>,
    z: PointerChain<f32>,
    a: PointerChain<f32>,
    hotkey_load: Option<i32>,
    hotkey_save: Option<i32>,
  ) -> PositionPointer {
    let save_label = ImString::new(format!(
      "Save ({})",
      hotkey_save
        .and_then(get_symbol)
        .unwrap_or_else(|| "".to_string())
    ));
    let load_label = ImString::new(format!(
      "Load ({})",
      hotkey_load
        .and_then(get_symbol)
        .unwrap_or_else(|| "".to_string())
    ));
    PositionPointer {
      x,
      y,
      z,
      a,
      saved_x: 0.,
      saved_y: 0.,
      saved_z: 0.,
      saved_a: 0.,
      hotkey_load,
      hotkey_save,
      load_label,
      save_label,
    }
  }

  fn read(&self) -> Option<(f32, f32, f32, f32)> {
    if let (Some(x), Some(y), Some(z), Some(a)) =
      (self.x.read(), self.y.read(), self.z.read(), self.a.read())
    {
      Some((x, y, z, a))
    } else {
      None
    }
  }

  fn save(&mut self) {
    if let Some((x, y, z, a)) = self.read() {
      self.saved_x = x;
      self.saved_y = y;
      self.saved_z = z;
      self.saved_a = a;
    }
  }

  fn load(&self) {
    self.x.write(self.saved_x);
    self.y.write(self.saved_y);
    self.z.write(self.saved_z);
    self.a.write(self.saved_a);
  }
}

impl Command for PositionPointer {
  fn display(&mut self, ctx: &RenderContext) -> bool {
    let ui = ctx.frame;
    let (sx, sy, sz, sa) = (self.saved_x, self.saved_y, self.saved_z, self.saved_a);
    let (cx, cy, cz, ca) = self.read().unwrap_or((0.0, 0.0, 0.0, 0.0));

    ui.text(ImString::new(format!(
      "Position [{:9.2}  {:9.2}  {:9.2}  {:9.2}]\n         [{:9.2}  {:9.2}  {:9.2}  {:9.2}]",
      cx, cy, cz, ca, sx, sy, sz, sa
    )));
    ui.same_line();
    
    let valid = self.is_valid();
    if ui.button(&self.load_label) && valid {
      self.load()
    }
    ui.same_line();
    if ui.button(&self.save_label) && valid {
      self.save();
    }
    ui.new_line();

    false
  }

  fn interact(&mut self, ctx: &RenderContext, is_interacting: bool) {
    if self
      .hotkey_load
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false)
    {
      self.load();
    }
    if self
      .hotkey_save
      .map(|k| ctx.frame.is_key_index_released(k as _))
      .unwrap_or(false)
      || (self.is_valid() && is_interacting)
    {
      self.save();
    }
  }

  fn is_valid(&self) -> bool {
    self.x.read().is_some()
  }
}
