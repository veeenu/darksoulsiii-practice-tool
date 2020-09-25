mod command_ui;
mod config;
mod memory;
mod palette;

//
// HudHook imports
//

use hudhook::memory::PointerChain;
use hudhook::*;

use imgui::{im_str, ImString, StyleVar, WindowFlags};

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

use command_ui::*;
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
  state: PracticeToolState, // all_no_damage: PointerChain<u8>,
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

    let config = config::Config::load_from_file(&config_path);

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
    info!("Loading configuration from {:?}", config_path);
    info!("Logging to {:?}", log_path);

    Box::new(DarkSoulsIIIPracticeTool {
      dll_path,
      config,
      commands: vec![],
      current_row: 0,
      capturing: true,
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
          self.commands = v.make_commands();
          Initialized(v)
        }
        None => panic!("Could not detect version!"),
      },
      _ => unreachable!(),
    }
  }

  fn render_inner<'a>(&mut self, ctx: RenderContext<'a>) {
    // Utility function for applying colors
    use imgui::{ColorStackToken, StyleColor};
    fn apply_colors(ui: &imgui::Ui, active: bool, valid: bool) -> ColorStackToken {
      if active {
        if valid {
          ui.push_style_colors(&[(StyleColor::Text, palette::ORANGE)])
        } else {
          ui.push_style_colors(&[(StyleColor::Text, palette::DARK_ORANGE)])
        }
      } else {
        if valid {
          ui.push_style_colors(&[(StyleColor::Text, palette::GRAY)])
        } else {
          ui.push_style_colors(&[(StyleColor::Text, palette::DARK_GRAY)])
        }
      }
    }

    // Rendering code
    let ui = ctx.frame;

    // Always process display toggle
    if self.config.is_key_released(ui, "display") {
      self.capturing = !self.capturing;
    }

    // Always process hotkeys
    for cmd in &mut self.commands {
      if self.config.is_key_released(ui, cmd.id()) {
        cmd.interact();
      }
    }

    // Don't do anything else if we're not visible
    if !self.capturing {
      return;
    }

    // let fonts = ui.fonts().fonts();
    // info!("{}", fonts.len());
    // let font_token = ui.push_font(fonts[if ctx.display_size[0] > 1920. && fonts.len() > 1 { 1 } else { 0 }]);

    let size = [
      f32::floor(ctx.display_size[0] / 3.),
      f32::floor(ctx.display_size[1]),
    ];

    let stack_token = ui.push_style_vars({
      &[
        StyleVar::WindowRounding(0.),
        StyleVar::FrameBorderSize(0.),
        StyleVar::WindowBorderSize(0.),
      ]
    });

    imgui::Window::new(im_str!("johndisandonato's Dark Souls III Practice Tool"))
      .position([0., 0.], imgui::Condition::FirstUseEver)
      .size(size, imgui::Condition::Always)
      .bg_alpha(0.6)
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
          | WindowFlags::NO_SCROLLBAR
      })
      .build(ui, || {
        ui.columns(3, im_str!(""), false);
        ui.set_column_width(0, 16.);
        ui.set_column_width(1, size[0] - 144.);
        ui.set_column_width(2, 128.);

        for (idx, cmd) in self.commands.iter_mut().enumerate() {
          let active = self.current_row == idx;
          let valid = cmd.is_valid();
          let style_token = apply_colors(ui, active, valid);

          // === Cursor column ===
          ui.text(ImString::new(format!("{}", if active { ">" } else { "" })));
          ui.next_column();

          // === Command column ===
          cmd.display(ui);
          if active && self.config.is_key_released(ui, "interact") {
            cmd.interact();
          }

          // === Hotkey column ===
          ui.next_column();
          if let Some(hotkey) = self.config.get_mapping(cmd.id()) {
            ui.text(ImString::new(format!(
              "{}",
              config::get_symbol(hotkey as _).unwrap_or_else(String::new)
            )))
          } else {
            ui.text(im_str!(""));
          }
          ui.next_column();

          style_token.pop(&ui);
        }

        ui.separator();

        // === Help box ===
        let style_token = apply_colors(ui, false, true);
        ui.next_column();
        ui.text(ImString::new(format!(
          "  Execute command: {}",
          config::get_symbol(self.config.get_mapping("interact").unwrap() as _).unwrap()
        )));
        ui.next_column();
        ui.next_column();

        ui.next_column();
        ui.text(ImString::new(format!(
          "  Show / Hide    : {}",
          config::get_symbol(self.config.get_mapping("display").unwrap() as _).unwrap(),
        )));
        ui.next_column();
        ui.next_column();

        ui.next_column();
        ui.text(ImString::new(format!(
          "  Previous / Next: {} / {}",
          config::get_symbol(self.config.get_mapping("prev").unwrap() as _).unwrap(),
          config::get_symbol(self.config.get_mapping("next").unwrap() as _).unwrap(),
        )));
        ui.next_column();
        ui.next_column();

        style_token.pop(&ui);

        // === Process prev/next commands ===
        if self.config.is_key_released(ui, "next") {
          self.current_row = usize::min(self.commands.len() - 1, self.current_row + 1);
          trace!("Current row {}", self.current_row);
        } else if self.config.is_key_released(ui, "prev") {
          self.current_row = self.current_row.saturating_sub(1);
          trace!("Current row {}", self.current_row);
        }
      });

    stack_token.pop(ui);
    // font_token.pop(ui);
  }
}

impl RenderLoop for DarkSoulsIIIPracticeTool {
  fn render<'a>(&mut self, ctx: RenderContext<'a>) {
    use PracticeToolState::*;

    match self.state {
      Uninit => self.initialize(),
      Initialized(_) => self.render_inner(ctx),
    }
  }

  fn is_visible(&self) -> bool {
    self.capturing
  }

  fn is_capturing(&self) -> bool {
    self.capturing
  }
}

hudhook!(DarkSoulsIIIPracticeTool::new());
