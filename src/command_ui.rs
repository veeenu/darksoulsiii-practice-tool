use crate::memory::{
  FlagPointer, PositionPointerLoader, PositionPointerSaver, QuitoutPointer, SoulsPointer,
};

use hudhook::prelude::*;
use imgui::ImString;
use log::*;

pub(crate) trait Command {
  fn display(&self, ui: &imgui::Ui);
  fn interact(&mut self);
  fn is_valid(&self) -> bool;
  fn id<'a>(&'a self) -> &'a str;
}

// TODO create wrapper methods in memory.rs to cleanup visibility issues

impl Command for FlagPointer {
  fn display(&self, ui: &imgui::Ui) {
    let value = self.get();

    ui.text(ImString::new(format!(
      "{} {}",
      if value.unwrap_or(false) { '*' } else { ' ' },
      self.label
    )));
  }

  fn interact(&mut self) {
    self.toggle();
  }

  fn id<'a>(&'a self) -> &'a str {
    &self.id
  }

  fn is_valid(&self) -> bool {
    self.get().is_some()
  }
}

impl Command for QuitoutPointer {
  fn display(&self, ui: &imgui::Ui) {
    ui.text(ImString::new(format!("  Quitout")));
  }

  fn interact(&mut self) {
    self.quitout()
  }

  fn id(&self) -> &str {
    "quitout"
  }

  fn is_valid(&self) -> bool {
    QuitoutPointer::is_valid(self)
  }
}

impl Command for SoulsPointer {
  fn display(&self, ui: &imgui::Ui) {
    if let Some(souls) = self.0.read() {
      ui.text(ImString::new(format!("  Increase souls [{}]", souls)));
    } else {
      ui.text(ImString::new(format!("  Increase souls")));
    }
  }

  fn interact(&mut self) {
    self.incr()
  }

  fn id<'a>(&'a self) -> &'a str {
    "souls"
  }

  fn is_valid(&self) -> bool {
    self.0.eval().is_some()
  }
}

impl Command for PositionPointerSaver {
  fn display(&self, ui: &imgui::Ui) {
    if let Some((x, y, z)) = self.read() {
      ui.text(ImString::new(format!(
        "  Save position [{:9.2}  {:9.2} {:9.2}]",
        x, y, z
      )));
    } else {
      ui.text(ImString::new(format!("  Save position")));
    }
  }

  fn interact(&mut self) {
    self.save();
  }

  fn id(&self) -> &str {
    "save_position"
  }

  fn is_valid(&self) -> bool {
    self.read().is_some()
  }
}

impl Command for PositionPointerLoader {
  fn display(&self, ui: &imgui::Ui) {
    let (x, y, z) = self.get_saved();
    ui.text(ImString::new(format!(
      "  Load position [{:9.2}  {:9.2} {:9.2}]",
      x, y, z
    )));
  }

  fn interact(&mut self) {
    self.load();
  }

  fn id(&self) -> &str {
    "load_position"
  }

  fn is_valid(&self) -> bool {
    true
  }
}
