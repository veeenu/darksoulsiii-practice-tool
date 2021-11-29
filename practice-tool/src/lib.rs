#![feature(once_cell)]

mod config;
mod memedit;
mod pointers;
mod style;
mod util;
mod widgets;

use std::sync::Arc;

use imgui::*;

use hudhook::hooks::dx11::ImguiRenderLoop;
use parking_lot::Mutex;

use crate::pointers::PointerChains;
use crate::widgets::{Widget, WidgetList};

struct PracticeTool {
    config: config::Config,
    root_widget: Arc<Mutex<Box<dyn widgets::Widget>>>,
    widgets_stack: Vec<Arc<Mutex<Box<dyn widgets::Widget>>>>,

    is_shown: bool,
}

impl PracticeTool {
    fn new() -> Self {
        use simplelog::*;
        hudhook::utils::alloc_console();

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

        let root_widget = {
            let pointers: PointerChains = pointers::detect_version()
                .expect("Couldn't detect version!")
                .into();

            let widgets = config.make_commands(&pointers);
            debug!("Config: {:?}", config);
            debug!("Widgets: {:?}", widgets);
            Arc::new(Mutex::new(
                Box::new(WidgetList::new(widgets)) as Box<dyn Widget>
            ))
        };

        let widgets_stack = vec![Arc::clone(&root_widget)];

        log_panics::init();
        PracticeTool {
            config,
            root_widget,
            widgets_stack,
            is_shown: false,
        }
    }

    fn render_visible(&mut self, ui: &mut imgui::Ui) {
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
                if self.config.settings.down.keyup() {
                    self.widgets_stack.last_mut().unwrap().lock().cursor_down();
                } else if self.config.settings.up.keyup() {
                    self.widgets_stack.last_mut().unwrap().lock().cursor_up();
                } else if self.config.settings.right.keyup() {
                    let child = {
                        self.widgets_stack
                            .last_mut()
                            .unwrap()
                            .lock()
                            .enter()
                            .clone()
                    };
                    if let Some(child) = child {
                        self.widgets_stack.push(child);
                    }
                } else if self.config.settings.left.keyup() && self.widgets_stack.len() > 1 {
                    self.widgets_stack.pop();
                }

                let mut w = self.widgets_stack.last_mut().unwrap().lock();
                w.interact();
                w.render(ui);
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

                // self.widgets_stack.last_mut().unwrap().lock().interact();
                for w in self.widgets_stack.iter().rev() {
                    w.lock().interact();
                }
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
