mod base_addresses;
mod config;

//
// HudHook imports
//

use hudhook::hook;
use hudhook::memory::{create_thread, PointerChain};
use hudhook::prelude::*;

use imgui::im_str;

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

use base_addresses::BaseAddresses;

enum PracticeToolState {
  Uninit,
  Initialized(BaseAddresses),
}

pub struct DarkSoulsIIIPracticeTool {
  dll_path: PathBuf,
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

    CombinedLogger::init(vec![
      TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed),
      WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        std::fs::File::create(log_path).unwrap(),
      ),
    ])
    .ok();

    // 0x1408D06F8 + 9
    Box::new(DarkSoulsIIIPracticeTool {
      dll_path,
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

  fn render_inner(&mut self, ui: &mut imgui::Ui) {
    info!("Render loop.");
    /*imgui::Window::new(im_str!("Hello"))
    .size([320.0, 256.0], imgui::Condition::FirstUseEver)
    .build(ui, || {
      ui.text(im_str!("Hello world!"));
      ui.separator();
      ui.text(im_str!("Hello world 2!"));
    });*/
  }
}

impl RenderLoop for DarkSoulsIIIPracticeTool {
  fn render(&mut self, ui: &mut imgui::Ui) {
    use PracticeToolState::*;

    match self.state {
      Uninit => self.initialize(),
      Initialized(_) => self.render_inner(ui),
    }
  }
}

hook!(DarkSoulsIIIPracticeTool::new());
