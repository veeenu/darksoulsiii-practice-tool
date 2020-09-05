mod base_addresses;
mod config;

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

use std::collections::HashSet;
use std::path::PathBuf;

//
// Dependencies imports
//

use log::*;
use simplelog::*;

//
// Crate imports
//

use base_addresses::BaseAddresses;

enum PracticeToolState {
  Uninit,
  Initialized(BaseAddresses),
}

pub struct DarkSoulsIIIPracticeTool {
  dll_path: PathBuf,
  config: config::Config,
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
        for idx in 0..15 {
          ui.columns(3, im_str!(""), false);
          ui.set_column_width(0, 16.);
          ui.set_column_width(1, size[0] - 144.);
          ui.set_column_width(2, 128.);

          let is_current_row = idx == self.current_row;
          let color = if is_current_row {
            [1., 1., 0., 1.]
          } else {
            [1., 1., 1., 1.]
          };

          ui.text_colored(
            color,
            ImString::new(if idx == self.current_row { ">" } else { "" }),
          );
          ui.next_column();
          ui.text_colored(
            color,
            &imgui::ImString::new(format!("Prova: {} {} {}", idx, size[0], size[1])),
          );
          ui.next_column();
          ui.text_colored(color, im_str!("VK_OEM_MINUS"));
          ui.next_column();
        }

        use winapi::um::winuser::{VK_DOWN, VK_UP};
        if ui.is_key_released(VK_DOWN as _) {
          self.current_row = usize::min(15, self.current_row + 1);
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
