#![feature(once_cell)]

mod config;
mod memedit;
mod pointers;
mod util;
mod widgets;

use std::time::Instant;

use hudhook::hooks::dx11::{ImguiRenderLoop, ImguiRenderLoopFlags};
use imgui::*;
use libds3::{wait_option, PARAMS};

use crate::pointers::PointerChains;

struct PracticeTool {
    config: config::Config,
    widgets: Vec<Box<dyn widgets::Widget>>,
    pointers: PointerChains,
    log: Vec<(Instant, String)>,

    is_shown: bool,
}

impl PracticeTool {
    fn new() -> Self {
        use simplelog::*;
        hudhook::utils::alloc_console();
        log_panics::init();

        fn load_config() -> Result<config::Config, String> {
            let config_path = crate::util::get_dll_path()
                .map(|mut path| {
                    path.pop();
                    path.push("jdsd_dsiii_practice_tool.toml");
                    path
                })
                .ok_or_else(|| "Couldn't find config file".to_string())?;
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
            },
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
            },
        }

        if let Some(err) = config_err {
            debug!("{:?}", err);
        }

        let pointers: PointerChains =
            pointers::detect_version().expect("Couldn't detect version!").into();

        let widgets = config.make_commands(&pointers);

        {
            let mut params = PARAMS.write();
            if let Some(mut darksign) = wait_option(|| unsafe {
                if let Err(e) = params.refresh() {
                    error!("{}", e);
                }
                params.get_equip_param_goods()
            })
            .find(|i| i.id == 117)
            .and_then(|p| p.param)
            {
                darksign.icon_id = 116;
            }
        }

        info!("Initialized");

        PracticeTool { config, pointers, widgets, is_shown: false, log: Vec::new() }
    }

    fn render_visible(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        imgui::Window::new("##tool_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.8)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(ui, || {
                if flags.focused {
                    for w in self.widgets.iter_mut() {
                        w.interact();
                    }
                }
                for w in self.widgets.iter_mut() {
                    w.render(ui);
                }
            });
    }

    fn render_closed(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
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

                if let Some(igt) = libds3::pointers::IGT.read() {
                    let millis = (igt % 1000) / 10;
                    let total_seconds = igt / 1000;
                    let seconds = total_seconds % 60;
                    let minutes = total_seconds / 60 % 60;
                    let hours = total_seconds / 3600;
                    ui.text(format!(
                        "IGT {:02}:{:02}:{:02}.{:02}",
                        hours, minutes, seconds, millis
                    ));
                }

                if flags.focused {
                    for w in self.widgets.iter_mut() {
                        w.interact();
                    }
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_logs(&mut self, ui: &mut imgui::Ui, _flags: &ImguiRenderLoopFlags) {
        let io = ui.io();

        let [dw, dh] = io.display_size;
        let [ww, wh] = [dw * 0.3, 14.0 * 6.];

        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];

        Window::new("##logs")
            .position_pivot([1., 1.])
            .position([dw * 0.95, dh * 0.8], Condition::Always)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .size([ww, wh], Condition::Always)
            .bg_alpha(0.0)
            .build(ui, || {
                for _ in 0..20 {
                    ui.text("");
                }
                for l in self.log.iter() {
                    ui.text(&l.1);
                }
                ui.set_scroll_here_y();
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        if flags.focused && self.config.settings.display.keyup() {
            self.is_shown = !self.is_shown;
            if !self.is_shown {
                self.pointers.mouse_enable.write(0u8);
            }
        }

        if self.is_shown {
            self.pointers.mouse_enable.write(1u8);
            self.render_visible(ui, flags);
        } else {
            self.render_closed(ui, flags);
        }

        for w in &mut self.widgets {
            if let Some(logs) = w.log() {
                let now = Instant::now();
                self.log.extend(logs.into_iter().map(|l| (now, l)));
            }
            self.log.retain(|(tm, _)| tm.elapsed() < std::time::Duration::from_secs(5));
        }

        self.render_logs(ui, flags);
    }
}

hudhook::hudhook!(PracticeTool::new().into_hook());
