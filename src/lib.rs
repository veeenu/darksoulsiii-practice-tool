mod command_ui;
mod config;
mod memory;
mod palette;

//
// HudHook imports
//

use hudhook::hook;
use hudhook::memory::{create_thread, PointerChain};
use hudhook::prelude::*;

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

    info!("DLL path: {:?}", dll_path);

    let mut log_path = dll_path.clone();
    log_path.push("jdsd_dsiii_practice_tool.log");

    let mut config_path = dll_path.clone();
    config_path.push("jdsd_dsiii_practice_tool.toml");

    let config = config::Config::load_from_file(&config_path);

    CombinedLogger::init(vec![
      TermLogger::new(
        config.settings.log_level.to_level_filter(),
        Config::default(),
        TerminalMode::Mixed,
      ),
      WriteLogger::new(
        config.settings.log_level.to_level_filter(),
        Config::default(),
        std::fs::File::create(log_path).unwrap(),
      ),
    ])
    .ok();

    // 0x1408D06F8 + 9
    Box::new(DarkSoulsIIIPracticeTool {
      dll_path,
      config,
      commands: vec![],
      current_row: 0,
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
    let size = [ctx.display_size[0] / 4., ctx.display_size[1]];
    let ui = ctx.frame;

    let stack_token = ui.push_style_vars({
      &[
        StyleVar::WindowRounding(0.),
        StyleVar::FrameBorderSize(0.),
        StyleVar::WindowBorderSize(0.),
      ]
    });

    imgui::Window::new(im_str!("johndisandonato's Dark Souls III Practice Tool"))
      .position([0., 0.], imgui::Condition::FirstUseEver)
      .size(size, imgui::Condition::FirstUseEver)
      .bg_alpha(0.3)
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
          | WindowFlags::NO_SCROLLBAR
      })
      .build(ui, || {
        for (idx, cmd) in self.commands.iter().enumerate() {
          let active = self.current_row == idx;
          cmd.display(ui, active, None, size);
          if active && ui.is_key_released('I' as _) {
            cmd.interact();
          }
        }

        use winapi::um::winuser::{VK_DOWN, VK_UP};
        if ui.is_key_released(VK_DOWN as _) {
          self.current_row = usize::min(self.commands.len() - 1, self.current_row + 1);
          info!("{}", self.current_row);
        } else if ui.is_key_released(VK_UP as _) {
          self.current_row = self.current_row.saturating_sub(1);
          info!("{}", self.current_row);
        }
      });

    stack_token.pop(ui);
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
}

hook!(DarkSoulsIIIPracticeTool::new());
