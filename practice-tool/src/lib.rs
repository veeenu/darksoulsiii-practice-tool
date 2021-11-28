#![feature(once_cell)]

mod config;
mod memedit;
mod pointers;
mod style;
mod util;
mod widgets;

use hudhook::hooks::dx11::ImguiRenderLoop;

use imgui::*;
use pointers::PointerChains;

struct PracticeTool {
    config: config::Config,
    widgets: Vec<Box<dyn widgets::Command>>,

    is_shown: bool,
}

impl PracticeTool {
    fn new() -> Self {
        use simplelog::*;
        hudhook::init::alloc_console();

        fn load_config() -> Result<config::Config, String> {
            let config_path = crate::util::get_dll_path()
                .map(|mut path| {
                    path.pop();
                    path.push("jdsd_dsiii_practice_tool.toml");
                    path
                })
                .ok_or_else(|| format!("Couldn't find config file"))?;
            let config_content = std::fs::read_to_string(config_path)
                .map_err(|e| format!("Couldn't read config file: {:?}", e))?;
            println!("{}", config_content);
            config::Config::parse(&config_content).map_err(String::from)
        }

        let (config, config_err) = match load_config() {
            Ok(config) => (config, None),
            Err(e) => (config::Config::default(), Some(e)),
        };

        let log_file = crate::util::get_dll_path()
            .map(|mut path| {
                path.pop();
                path.push("jdsd_dsiii_practice_tool.log");
                path
            })
            .map(std::fs::File::create);

        match log_file {
            Some(Ok(log_file)) => {
                CombinedLogger::init(vec![
                    TermLogger::new(
                        config.settings.log_level.inner(),
                        Config::default(),
                        TerminalMode::Mixed,
                    ),
                    WriteLogger::new(
                        config.settings.log_level.inner(),
                        Config::default(),
                        log_file,
                    ),
                ])
                .ok();
            }
            e => {
                CombinedLogger::init(vec![TermLogger::new(
                    LevelFilter::Debug, // config.settings.log_level.to_level_filter(),
                    Config::default(),
                    TerminalMode::Mixed,
                )])
                .ok();

                match e {
                    None => error!("Could not construct log file path"),
                    Some(Err(e)) => error!("Could not initialize log file: {:?}", e),
                    _ => unreachable!(),
                }
            }
        }

        if let Some(err) = config_err {
            debug!("{:?}", err);
        }

        let widgets = {
            let pointers: PointerChains = pointers::detect_version()
                .expect("Couldn't detect version!")
                .into();

            let widgets = config.make_commands(&pointers);
            debug!("Config: {:?}", config);
            debug!("Widgets: {:?}", widgets);
            widgets
        };

        log_panics::init();
        PracticeTool { config, widgets, is_shown: false }
    }

    fn render_visible(&mut self, ui: &mut imgui::Ui) {
        imgui::Window::new("##tool_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.0)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(ui, || {
                for w in &self.widgets {
                    w.render(ui);
                }
            });
    }

    fn render_closed(&mut self, ui: &mut imgui::Ui) {
        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];
        imgui::Window::new("##msg_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.0)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(ui, || {
                ui.text("johndisandonato's Dark Souls III Practice Tool is active");
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut imgui::Ui) {
        if self.config.settings.display.keyup() {
            self.is_shown = !self.is_shown;
        }

        if self.is_shown {
            self.render_visible(ui);
        } else {
            self.render_closed(ui);
        }
    }
}

fn init() {}

hudhook::hudhook!(
    {
        init();
    },
    [hudhook::hooks::dx11::hook_imgui(PracticeTool::new())]
);
