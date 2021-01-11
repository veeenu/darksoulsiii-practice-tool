mod commands;
pub mod config;
mod memory;
mod palette;
pub mod utils;

//
// HudHook imports
//

use hudhook::*;

use imgui::{im_str, StyleVar, WindowFlags};

//
// Stdlib imports
//

use std::path::PathBuf;

//
// Dependencies imports
//

use log::*;
use simplelog::*;

//
// Crate imports
//

use commands::*;
use memory::BaseAddresses;

enum PracticeToolState {
  Uninit,
  Initialized(BaseAddresses),
}

pub struct DarkSoulsIIIPracticeTool {
  dll_path: PathBuf,
  config: config::Config,
  commands: Vec<Box<dyn Command>>,
  current_row: usize,
  capturing: bool,
  state: PracticeToolState,
}

impl DarkSoulsIIIPracticeTool {
  fn new() -> Box<DarkSoulsIIIPracticeTool> {
    log_panics::init();

    let dll_path = std::fs::canonicalize(
      &PathBuf::from(get_dll_path().unwrap_or_else(|| String::from("")))
        .parent()
        .unwrap(), // Unwrap OK: path is always going to have a parent
    )
    .unwrap();

    let mut log_path = dll_path.clone();
    log_path.push("jdsd_dsiii_practice_tool.log");

    let mut config_path = dll_path.clone();
    config_path.push("jdsd_dsiii_practice_tool.toml");

    let config = match config::Config::load_from_file(&config_path) {
      Ok(config) => config,
      Err(e) => {
        error!("{}", e);
        config::Config::default()
      }
    };

    if config.settings.log_level > log::Level::Info {
      unsafe {
        winapi::um::consoleapi::AllocConsole();
      }
    }

    CombinedLogger::init(vec![
      TermLogger::new(
        config.settings.log_level.to_level_filter(),
        Config::default(),
        TerminalMode::Mixed,
      ),
      WriteLogger::new(
        config.settings.log_level.to_level_filter(),
        Config::default(),
        std::fs::File::create(&log_path).unwrap(),
      ),
    ])
    .ok();

    debug!("DLL path: {:?}", dll_path);
    info!(
      "Loading configuration from {:?}: {:#?}",
      config_path, config
    );
    info!("Logging to {:?}", log_path);

    Box::new(DarkSoulsIIIPracticeTool {
      dll_path,
      config,
      commands: vec![],
      current_row: 0,
      capturing: false,
      state: PracticeToolState::Uninit,
    })
  }

  fn initialize(&mut self) {
    info!("Initializing practice tool.");
    use PracticeToolState::*;

    self.state = match self.state {
      Uninit => match BaseAddresses::detect_version() {
        Some(v) => {
          info!("Matched version: {:?}", v.version);
          // self.commands = v.make_commands().map(;
          if let Some(pointer_chains) = v.make_commands() {
            self.commands = self
              .config
              .command
              .iter()
              .filter_map(|cmd| cmd.try_to_command(&pointer_chains))
              .collect();
          }
          Initialized(v)
        }
        None => panic!("Could not detect version!"),
      },
      _ => unreachable!(),
    }
  }

  fn render_inner(&mut self, ctx: RenderContext<'_>) {
    // Utility function for applying colors
    use imgui::{ColorStackToken, StyleColor};
    fn apply_colors(ui: &imgui::Ui, active: bool, valid: bool) -> ColorStackToken {
      if active && valid {
        ui.push_style_colors(&[(StyleColor::Text, palette::ORANGE)])
      } else if active && !valid {
        ui.push_style_colors(&[(StyleColor::Text, palette::DARK_ORANGE)])
      } else if valid {
        ui.push_style_colors(&[(StyleColor::Text, palette::GRAY)])
      } else {
        ui.push_style_colors(&[(StyleColor::Text, palette::DARK_GRAY)])
      }
    }

    // Rendering code
    let ui = ctx.frame;

    // Always process display toggle
    if ui.is_key_released(self.config.settings.display as _) {
      self.capturing = !self.capturing;
    }

    let interacting = ui.is_key_released(self.config.settings.interact as _);
    // Always process hotkeys
    for (idx, cmd) in self.commands.iter_mut().enumerate() {
      let active = self.current_row == idx && self.capturing;
      cmd.interact(ui, active, interacting);
    }

    /*let (font_id, _col_width, _col_height) = {
      let fonts = ui.fonts().fonts();
      (fonts[0], 14., 13.)
    };*/

    // Don't do anything else if we're not visible
    if !self.capturing {
      //ui.set_mouse_cursor(None);
      let stack_token = ui.push_style_vars({
        &[
          StyleVar::WindowRounding(0.),
          StyleVar::FrameBorderSize(0.),
          StyleVar::WindowBorderSize(0.),
        ]
      });

      imgui::Window::new(im_str!("##msg_window"))
        .position([16., 16.], imgui::Condition::FirstUseEver)
        .bg_alpha(0.0)
        .flags({
          WindowFlags::NO_DECORATION
            | WindowFlags::NO_COLLAPSE
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
        })
        .build(ui, || {
          ui.text(im_str!("johndisandonato's Dark Souls III Practice Tool is active"));
        });

      stack_token.pop(ui);
      return;
    }
    // ui.set_mouse_cursor(Some(imgui::MouseCursor::Arrow));

    // let size = [f32::floor(col_width * 36.), f32::floor(ctx.display_size[1])];

    let stack_token = ui.push_style_vars({
      &[
        StyleVar::WindowRounding(0.),
        StyleVar::FrameBorderSize(0.),
        StyleVar::WindowBorderSize(1.),
      ]
    });

    imgui::Window::new(im_str!("johndisandonato's Dark Souls III Practice Tool"))
      .position([32., 32.], imgui::Condition::FirstUseEver)
      .bg_alpha(0.6)
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
          | WindowFlags::NO_SCROLLBAR
      })
      .build(ui, || {
        // let font_token = ui.push_font(font_id);

        for (idx, cmd) in self.commands.iter_mut().enumerate() {
          let active = self.current_row == idx;
          let valid = cmd.is_valid();
          let style_token = apply_colors(ui, active, valid);

          cmd.display(ui);
          style_token.pop(&ui);
        }

        ui.separator();

        // === Help box ===
        ui.text(imgui::ImString::new(format!(
          "Show / Hide    : {}",
          config::get_symbol(self.config.settings.display as _).unwrap(),
        )));

        ui.text(imgui::ImString::new(format!(
          "Previous / Next: {} / {}",
          config::get_symbol(self.config.settings.prev as _).unwrap(),
          config::get_symbol(self.config.settings.next as _).unwrap(),
        )));

        // Placeholder for debug info
        // ui.next_column();
        // ui.text(ImString::new(format!(
        // )));

        // === Process prev/next commands ===
        // if self.config.is_key_released(ui, "next") {
        if ui.is_key_released(self.config.settings.next as _) {
          self.current_row = usize::min(self.commands.len() - 1, self.current_row + 1);
          trace!("Current row {}", self.current_row);
        } else if ui.is_key_released(self.config.settings.prev as _) {
          self.current_row = self.current_row.saturating_sub(1);
          trace!("Current row {}", self.current_row);
        }

        // style_token.pop(&ui);
        // font_token.pop(ui);
      });

    stack_token.pop(ui);
  }
}

impl RenderLoop for DarkSoulsIIIPracticeTool {
  fn render(&mut self, ctx: RenderContext<'_>) {
    use PracticeToolState::*;

    match self.state {
      Uninit => self.initialize(),
      Initialized(_) => self.render_inner(ctx),
    }
  }

  fn is_visible(&self) -> bool {
    // self.capturing
    true
  }

  fn is_capturing(&self) -> bool {
    self.capturing
  }
}

hudhook!(DarkSoulsIIIPracticeTool::new());
