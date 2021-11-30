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
use crate::util::GlobalKeys;
use crate::widgets::{Widget, WidgetList};

struct PracticeTool {
    config: config::Config,
    widgets_stack: Vec<Arc<Mutex<Box<dyn widgets::Widget>>>>,

    keys: GlobalKeys,
    log: Vec<String>,

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
            widgets_stack,
            keys: GlobalKeys::new(),
            is_shown: false,
            log: Vec::new(),
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
                let want_exit = { self.widgets_stack.last_mut().unwrap().lock().want_exit() };

                if self.keys.down.keyup() {
                    // Send cursor down event to current widget
                    self.widgets_stack.last_mut().unwrap().lock().cursor_down();
                } else if self.keys.up.keyup() {
                    // Send cursor up event to current widget
                    self.widgets_stack.last_mut().unwrap().lock().cursor_up();
                } else if (self.keys.esc.keyup() || want_exit) && self.widgets_stack.len() > 1 {
                    // Exit event: pop a widget from the stack
                    self.widgets_stack.pop();
                } else if self.keys.enter.keyup() {
                    // Send enter event to current widget, interact if event is ignored
                    let child = {
                        self.widgets_stack
                            .last_mut()
                            .unwrap()
                            .lock()
                            .enter(ui)
                            .clone()
                    };
                    if let Some(child) = child {
                        self.widgets_stack.push(child);
                    } else {
                        self.widgets_stack.last_mut().unwrap().lock().interact_ui();
                    }
                }

                {
                    for w in self.widgets_stack.iter().rev() {
                        w.lock().interact();
                    }
                }
                for w in &mut self.widgets_stack {
                    w.lock().render(ui);
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

                for w in self.widgets_stack.iter().rev() {
                    w.lock().interact();
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_logs(&mut self, ui: &mut imgui::Ui) {
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
                    ui.text(&l);
                }
                ui.set_scroll_here_y();
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

        for w in &mut self.widgets_stack {
            if let Some(logs) = w.lock().log() {
                self.log.extend(logs);
            }
        }

        self.render_logs(ui);
    }
}

fn init() {}

hudhook::hudhook!(
    {
        init();
    },
    [hudhook::hooks::dx11::hook_imgui(PracticeTool::new())]
);
