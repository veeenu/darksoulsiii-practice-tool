mod commands;
pub mod config;
mod memory;
mod palette;
pub mod utils;

//
// HudHook imports
//

use hudhook::*;
use imgui::*;

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
use memory::{BaseAddresses, PointerChains};

enum PracticeToolState {
  Uninit,
  Initialized(BaseAddresses),
}

pub struct DarkSoulsIIIPracticeTool {
  // dll_path: PathBuf,
  config: config::Config,
  commands: Vec<Box<dyn Command>>,
  pointer_chains: Option<PointerChains>,
  capturing: bool,
  focus: (bool, Option<u8>),
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

    let (config, config_load_err) = match config::Config::load_from_file(&config_path) {
      Ok(config) => (config, None),
      Err(e) => (config::Config::default(), Some(e)),
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

    if let Some(e) = config_load_err {
      error!("{}", e);
    }

    debug!("DLL path: {:?}", dll_path);
    info!(
      "Loading configuration from {:?}: {:#?}",
      config_path, config
    );
    info!("Logging to {:?}", log_path);

    Box::new(DarkSoulsIIIPracticeTool {
      // dll_path,
      config,
      commands: vec![],
      pointer_chains: None,
      capturing: false,
      focus: (false, None),
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
          if let Some(pointer_chains) = v.make_commands() {
            self.commands = self
              .config
              .command
              .iter()
              .filter_map(|cmd| cmd.try_to_command(&pointer_chains))
              .collect();
            self.pointer_chains = Some(pointer_chains);
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
      self.focus();
    }

    // Always process hotkeys
    for cmd in self.commands.iter_mut() {
      cmd.interact(ui, false, false);
    }

    // Always display the meme color
    /*self.pointer_chains.as_ref().map(|p| {
      use std::time::{SystemTime, Duration};
      let h = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis();
      let h = ((h / 10) % 360) as f32;

      fn hsv2rgb(h: f32, s: f32, b: f32) -> u32 {
        let h = h / 60.;
        let i = h.trunc() as u8;
        let f = h.fract();

        let p = b * (1. - s);
        let q = b * (1. - s * f);
        let t = b * (1. - s * (1. - f));

        let (r, g, b) = match i {
          0 => (b, t, p),
          1 => (q, b, p),
          2 => (p, b, t),
          3 => (p, q, b),
          4 => (t, p, b),
          _ => (b, p, q)
        };
        let (r, g, b) = (r * 255., g * 255., b * 255.);
        let (r, g, b) = (r as u32, g as u32, b as u32);

        r << 16 | g << 8 | b
      }

      let mut f: [u16; 90] = [0; 90]; // p.format_string.read()?;
      format!("<TEXTFORMAT LEADING='%d'><FONT LETTERSPACING='%d' COLOR='#{:06x}'>%s</FONT></TEXTFORMAT>\0\0", hsv2rgb(h, 0.75, 0.75))
        .encode_utf16()
        .into_iter()
        .enumerate()
        .for_each(|(idx, val): (usize, u16)| {
          f[idx] = val;
        });
      p.format_string.write(f)?;

      None::<Option<()>>
    });*/

    // Don't do anything else if we're not visible
    if !self.capturing {
      let stack_token = ui.push_style_vars({
        &[
          StyleVar::WindowRounding(0.),
          StyleVar::FrameBorderSize(0.),
          StyleVar::WindowBorderSize(0.),
        ]
      });

      imgui::Window::new(im_str!("##msg_window"))
        .position([16., 16.], imgui::Condition::Always)
        .bg_alpha(0.0)
        .flags({
          WindowFlags::NO_DECORATION
            | WindowFlags::NO_COLLAPSE
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
        })
        .build(ui, || {
          ui.text(im_str!(
            "johndisandonato's Dark Souls III Practice Tool is active"
          ));
        });

      stack_token.pop(ui);
      return;
    }

    let stack_token = ui.push_style_vars({
      &[
        StyleVar::WindowRounding(0.),
        StyleVar::FrameBorderSize(0.),
        StyleVar::WindowBorderSize(1.),
      ]
    });

    imgui::Window::new(im_str!("johndisandonato's Dark Souls III Practice Tool"))
      .position([32., 32.], imgui::Condition::Always)
      .bg_alpha(if self.focus.0 { 0.95 } else { 0.6 })
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
          | WindowFlags::NO_SCROLLBAR
      })
      .build(ui, || {
        for cmd in self.commands.iter_mut() {
          let valid = cmd.is_valid();
          let style_token = apply_colors(ui, false, valid);

          cmd.display(ui);
          style_token.pop(&ui);
        }

        ui.separator();

        // === Help box ===
        ui.text(imgui::ImString::new(format!(
          concat!(
            "Show / Hide    : {}\n",
            // "Down / Up      : {} / {}\n",
            // "Left / Right   : {} / {}\n",
          ),
          config::get_symbol(self.config.settings.display as _).unwrap(),
          /*config::get_symbol(self.config.settings.down as _).unwrap(),
          config::get_symbol(self.config.settings.up as _).unwrap(),
          config::get_symbol(self.config.settings.left as _).unwrap(),
          config::get_symbol(self.config.settings.right as _).unwrap(),*/
        )));
      });

    stack_token.pop(ui);
  }

  fn focus(&mut self) {
    #[cfg(feature = "focus")]
    {
      let r = self.pointer_chains.as_ref();
      if !self.focus.0 {
        self.focus = (true, r.and_then(|c| c.mouse_enable.read()));
        r.and_then(|c| c.mouse_enable.write(2));
      } else if let Some(v) = self.focus.1 {
        self.focus = (false, None);
        r.and_then(|c| c.mouse_enable.write(v));
      }
    }
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
