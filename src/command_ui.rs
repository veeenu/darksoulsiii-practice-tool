use crate::memory::{FlagPointer, PositionPointer, QuitoutPointer, SoulsPointer};
use crate::palette;

use hudhook::prelude::*;
use imgui::{im_str, ColorStackToken, ImString, StyleColor};

pub(crate) trait Command {
  fn display(&self, ui: &imgui::Ui, active: bool, hotkey: Option<String>, size: [f32; 2]);
  fn interact(&self);
}

fn apply_colors(ui: &imgui::Ui, active: bool, valid: bool) -> imgui::ColorStackToken {
  if active {
    if valid {
      ui.push_style_colors(&[
        (StyleColor::Text, palette::ORANGE),
        //(StyleColor::Text, palette::DARK_GRAY),
        //(StyleColor::WindowBg, palette::ORANGE),
      ])
    } else {
      ui.push_style_colors(&[
        (StyleColor::Text, palette::DARK_ORANGE),
        //(StyleColor::Text, palette::DARK_GRAY),
        //(StyleColor::WindowBg, palette::DARK_ORANGE),
      ])
    }
  } else {
    if valid {
      ui.push_style_colors(&[
        (StyleColor::Text, palette::DARK_GRAY),
        //(StyleColor::Text, palette::DARK_GRAY),
        //(StyleColor::WindowBg, palette::DARK_ORANGE),
      ])
    } else {
      ui.push_style_colors(&[
        (StyleColor::Text, palette::GRAY),
        //(StyleColor::Text, palette::DARK_GRAY),
        //(StyleColor::WindowBg, palette::DARK_ORANGE),
      ])
    }
  }
}

impl Command for FlagPointer {
  fn display(&self, ui: &imgui::Ui, active: bool, hotkey: Option<String>, size: [f32; 2]) {
    ui.columns(3, im_str!(""), false);
    ui.set_column_width(0, 16.);
    ui.set_column_width(1, size[0] - 144.);
    ui.set_column_width(2, 128.);

    let value = self.get();

    let style_token = apply_colors(ui, active, value.is_some());

    ui.text(ImString::new(if active { ">" } else { "" }));
    ui.next_column();
    ui.text(ImString::new(format!(
      "{} {}",
      if value.unwrap_or(false) { '*' } else { ' ' },
      self.label
    )));
    ui.next_column();
    if let Some(hotkey) = hotkey {
      ui.text(ImString::new(hotkey));
    } else {
      ui.text(im_str!(""));
    }
    ui.next_column();

    style_token.pop(&ui);
  }

  fn interact(&self) {
    self.toggle();
  }
}

impl Command for QuitoutPointer {
  fn display(&self, ui: &imgui::Ui, active: bool, hotkey: Option<String>, size: [f32; 2]) {
    ui.columns(3, im_str!(""), false);
    ui.set_column_width(0, 16.);
    ui.set_column_width(1, size[0] - 144.);
    ui.set_column_width(2, 128.);

    let valid = self.is_valid();

    let style_token = apply_colors(ui, active, valid);

    ui.text(ImString::new(if active { ">" } else { "" }));
    ui.next_column();
    ui.text(ImString::new(format!("  {}", self.label)));
    ui.next_column();
    if let Some(hotkey) = hotkey {
      ui.text(ImString::new(hotkey));
    }
    ui.next_column();

    style_token.pop(&ui);
  }

  fn interact(&self) {
    self.quitout()
  }
}
