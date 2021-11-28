#![feature(once_cell)]

mod memedit;
mod pointers;
mod style;
mod util;
mod widgets;

use hudhook::hooks::dx11::ImguiRenderLoop;

use imgui::*;
use pointers::PointerChains;

struct PracticeTool {
    // pointers: SyncLazy<pointers::PointerChains>,
    widgets: Vec<Box<dyn widgets::Command>>,
}

impl PracticeTool {
    fn new() -> Self {
        use simplelog::*;
        hudhook::init::alloc_console();

        fn load_config() -> Result<widgets::config::Config, String> {
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
            widgets::config::Config::parse(&config_content).map_err(String::from)
        }

        let (config, config_err) = match load_config() {
            Ok(config) => (config, None),
            Err(e) => (widgets::config::Config::default(), Some(e)),
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

        debug!("{:#?}", config);
        if let Some(err) = config_err {
            debug!("{:?}", err);
        }

        log_panics::init();
        PracticeTool {
            widgets: {
                let pointers: PointerChains = pointers::detect_version()
                    .expect("Couldn't detect version!")
                    .into();

                let widgets = config.make_commands(&pointers);
                debug!("Config: {:?}", config);
                debug!("Widgets: {:?}", widgets);
                widgets
            },
        }
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut imgui::Ui) {
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

                for w in &self.widgets {
                    w.render(ui);
                }
                // if let Some(pos) = position {
                //     ui.text(format!(
                //         "{:.2}  {:.2}  {:.2}  {:.2}  ({:.2})",
                //         pos.x, pos.y, pos.z, pos.w, pos.unknown
                //     ));
                // } else {
                //     ui.text("No position found");
                // }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
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
