mod commands;
pub mod config;
mod memory;
mod palette;
// mod treenav;

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

pub enum DarkSoulsIIIPracticeToolStub {
  Uninitialized(config::Config),
  Initialized(DarkSoulsIIIPracticeTool),
}

impl DarkSoulsIIIPracticeToolStub {
  fn new() -> Box<DarkSoulsIIIPracticeToolStub> {
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

    Box::new(DarkSoulsIIIPracticeToolStub::Uninitialized(config))
  }

  fn initialize(config: config::Config) -> DarkSoulsIIIPracticeToolStub {
    info!("Initializing practice tool.");
    if let Some(base_addresses) = BaseAddresses::detect_version() {
      info!("Matched version: {:?}", base_addresses.version);
      let pointer_chains = base_addresses.make_commands();
      let commands = pointer_chains.as_ref().and_then(|pointer_chains| {
        Some(
          config
            .command
            .iter()
            .filter_map(|cmd| cmd.try_to_command(&pointer_chains))
            .collect(),
        )
      });

      if let Some(commands) = commands {
        DarkSoulsIIIPracticeToolStub::Initialized(DarkSoulsIIIPracticeTool {
          config,
          commands,
          pointer_chains,
          base_addresses,
          capturing: false,
          ctl_state: ControllerState::new(),
        })
      } else {
        panic!("No commands supplied")
      }
    } else {
      panic!("Could not detect version!");
    }
  }
}

struct ControllerState(hudhook::ControllerState, hudhook::ControllerState);

impl ControllerState {
  fn new() -> ControllerState {
    ControllerState(Default::default(), Default::default())
  }

  fn update(&mut self, new_state: &hudhook::ControllerState) {
    *self = ControllerState(self.1.clone(), new_state.clone())
  }

  fn pressed<F>(&self, f: F) -> bool
  where
    F: Fn(&hudhook::ControllerState) -> bool,
  {
    f(&self.1) && !f(&self.0)
  }

  fn released<F>(&self, f: F) -> bool
  where
    F: Fn(&hudhook::ControllerState) -> bool,
  {
    f(&self.0) && !f(&self.1)
  }
}

pub struct DarkSoulsIIIPracticeTool {
  config: config::Config,
  base_addresses: BaseAddresses,
  commands: Vec<Box<dyn Command + Send + Sync>>,
  pointer_chains: Option<PointerChains>,
  capturing: bool,
  ctl_state: ControllerState,
}

impl DarkSoulsIIIPracticeTool {
  fn render(&mut self, ctx: RenderContext) {
    // Utility function for applying colors
    fn apply_colors<'a>(ui: &'a imgui::Ui, active: bool, valid: bool) -> ColorStackToken<'a> {
      if active && valid {
        ui.push_style_color(StyleColor::Text, palette::ORANGE)
      } else if active && !valid {
        ui.push_style_color(StyleColor::Text, palette::DARK_ORANGE)
      } else if valid {
        ui.push_style_color(StyleColor::Text, palette::GRAY)
      } else {
        ui.push_style_color(StyleColor::Text, palette::DARK_GRAY)
      }
    }

    // Rendering code
    let ui = ctx.frame;
    self.ctl_state.update(&ctx.controller);

    // Always process display toggle
    if ui.is_key_index_released(self.config.settings.display as _)
      || self.ctl_state.released(|s| s.rb && s.lb && s.start)
    {
      self.capturing = !self.capturing;
      self.focus();
    }

    // Always process hotkeys
    {
      for cmd in self.commands.iter_mut() {
        cmd.interact(&ctx, false);
      }
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
      let stack_tokens = vec![
        ui.push_style_var(StyleVar::WindowRounding(0.)),
        ui.push_style_var(StyleVar::FrameBorderSize(0.)),
        ui.push_style_var(StyleVar::WindowBorderSize(0.)),
      ];
      imgui::Window::new(im_str!("##msg_window"))
        .position([16., 16.], Condition::Always)
        .bg_alpha(0.0)
        .flags({
          WindowFlags::NO_TITLE_BAR
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
        })
        .build(ui, || {
          ui.text(im_str!(
            "johndisandonato's Dark Souls III Practice Tool is active"
          ));
        });

      for st in stack_tokens.into_iter().rev() {
        st.pop();
      }
    } else {
      let stack_tokens = vec![
        ui.push_style_var(StyleVar::WindowRounding(0.)),
        ui.push_style_var(StyleVar::FrameBorderSize(0.)),
        ui.push_style_var(StyleVar::WindowBorderSize(1.)),
      ];

      imgui::Window::new(im_str!("johndisandonato's Dark Souls III Practice Tool"))
        .position([32., 32.], imgui::Condition::Always)
        .flags({
          WindowFlags::NO_TITLE_BAR
            | WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            | WindowFlags::NO_SCROLLBAR
        })
        .build(ui, || {
          for cmd in self.commands.iter_mut() {
            let valid = cmd.is_valid();
            let style_token = apply_colors(ui, false, valid);

            cmd.display(&ctx);
            style_token.pop();
          }

          ui.separator();

          // === Help box ===
          ui.text(imgui::ImString::new(format!(
            concat!("Show / Hide    : {} or START + LB + RB\n",),
            config::get_symbol(self.config.settings.display as _).unwrap(),
          )));
        });

      for st in stack_tokens.into_iter().rev() {
        st.pop();
      }
    }
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

impl RenderLoop for DarkSoulsIIIPracticeToolStub {
  fn render(&mut self, ctx: RenderContext) {
    use DarkSoulsIIIPracticeToolStub::*;

    match self {
      Uninitialized(config) => *self = DarkSoulsIIIPracticeToolStub::initialize(config.clone()),
      Initialized(practice_tool) => practice_tool.render(ctx),
    }
  }

  fn is_visible(&self) -> bool {
    // self.capturing
    true
  }

  fn is_capturing(&self) -> bool {
    if let DarkSoulsIIIPracticeToolStub::Initialized(t) = self {
      t.capturing
    } else {
      false
    }
  }
}

hudhook!(DarkSoulsIIIPracticeToolStub::new());
