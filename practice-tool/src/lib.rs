#![feature(lazy_cell)]

mod config;
mod util;
mod widgets;

use std::time::Instant;

use const_format::formatcp;
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use imgui::*;
use libds3::prelude::*;
use pkg_version::*;
use widgets::{BUTTON_HEIGHT, BUTTON_WIDTH};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_RSHIFT};

struct FontIDs {
    small: FontId,
    normal: FontId,
    big: FontId,
}

unsafe impl Send for FontIDs {}
unsafe impl Sync for FontIDs {}

enum UiState {
    MenuOpen,
    Closed,
    Hidden,
}

struct PracticeTool {
    config: config::Config,
    widgets: Vec<Box<dyn widgets::Widget>>,
    pointers: PointerChains,
    log: Vec<(Instant, String)>,
    ui_state: UiState,
    fonts: Option<FontIDs>,
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

        if config.settings.log_level.inner() < LevelFilter::Debug || !config.settings.show_console {
            hudhook::utils::free_console();
        }

        let pointers = PointerChains::new();

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

        PracticeTool {
            config,
            pointers,
            widgets,
            ui_state: UiState::Closed,
            log: Vec::new(),
            fonts: None,
        }
    }

    fn render_visible(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
        ui.window("##tool_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.8)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                if flags.focused {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }
                for w in self.widgets.iter_mut() {
                    w.render(ui);
                }

                if ui.button_with_size("Close", [
                    BUTTON_WIDTH * widgets::scaling_factor(ui),
                    BUTTON_HEIGHT,
                ]) {
                    self.ui_state = UiState::Closed;
                    self.pointers.cursor_show.set(false);
                }

                if option_env!("CARGO_XTASK_DIST").is_none()
                    && ui.button_with_size("Eject", [
                        BUTTON_WIDTH * widgets::scaling_factor(ui),
                        BUTTON_HEIGHT,
                    ])
                {
                    self.ui_state = UiState::Closed;
                    self.pointers.cursor_show.set(false);
                    hudhook::lifecycle::eject();
                }
            });
    }

    fn render_closed(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
        let stack_tokens = [
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];
        ui.window("##msg_window")
            .position([16., 16.], Condition::Always)
            .bg_alpha(0.0)
            .flags({
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE
            })
            .build(|| {
                ui.text("johndisandonato's Dark Souls III Practice Tool is active");

                ui.same_line();

                if ui.small_button("Open") {
                    self.ui_state = UiState::MenuOpen;
                }

                ui.same_line();

                if ui.small_button("Help") {
                    ui.open_popup("##help_window");
                }

                ui.modal_popup_config("##help_window")
                    .resizable(false)
                    .movable(false)
                    .title_bar(false)
                    .build(|| {
                        self.pointers.cursor_show.set(true);
                        ui.text(formatcp!(
                            "Dark Souls III Practice Tool v{}.{}.{}",
                            pkg_version_major!() as usize,
                            pkg_version_minor!() as usize,
                            pkg_version_patch!() as usize,
                        ));
                        ui.separator();
                        ui.text(format!(
                            "Press the {} key to open/close the tool's\ninterface.\n\nYou can \
                             toggle flags/launch commands by\nclicking in the UI or by \
                             pressing\nthe hotkeys (in the parentheses).\n\nYou can configure \
                             your tool by editing\nthe jdsd_dsiii_practice_tool.toml file with\na \
                             text editor. If you break something,\njust download a fresh \
                             file!\n\nThank you for using my tool! <3\n",
                            self.config.settings.display
                        ));
                        ui.separator();
                        ui.text("-- johndisandonato");
                        ui.text("   https://twitch.tv/johndisandonato");
                        if ui.is_item_clicked() {
                            open::that("https://twitch.tv/johndisandonato").ok();
                        }
                        ui.separator();
                        if ui.button("Close") {
                            ui.close_current_popup();
                            self.pointers.cursor_show.set(false);
                        }
                        ui.same_line();
                        if ui.button("Submit issue") {
                            open::that(
                                "https://github.com/veeenu/darksoulsiii-practice-tool/issues/new",
                            )
                            .ok();
                        }
                    });

                if let Some(igt) = self.pointers.igt.read() {
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

                if flags.focused && !ui.io().want_capture_keyboard {
                    for w in self.widgets.iter_mut() {
                        w.interact(ui);
                    }
                }
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }

    fn render_hidden(&mut self, ui: &imgui::Ui, flags: &ImguiRenderLoopFlags) {
        if flags.focused && !ui.io().want_capture_keyboard {
            for w in self.widgets.iter_mut() {
                w.interact(ui);
            }
        }
    }

    fn render_logs(&mut self, ui: &imgui::Ui, _flags: &ImguiRenderLoopFlags) {
        let io = ui.io();

        let [dw, dh] = io.display_size;
        let [ww, wh] = [dw * 0.3, 14.0 * 6.];

        let stack_tokens = vec![
            ui.push_style_var(StyleVar::WindowRounding(0.)),
            ui.push_style_var(StyleVar::FrameBorderSize(0.)),
            ui.push_style_var(StyleVar::WindowBorderSize(0.)),
        ];

        ui.window("##logs")
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
            .build(|| {
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

    fn set_font<'a>(&mut self, ui: &'a imgui::Ui) -> imgui::FontStackToken<'a> {
        let width = ui.io().display_size[0];
        let font_id = self
            .fonts
            .as_mut()
            .map(|fonts| {
                if width > 2000. {
                    fonts.big
                } else if width > 1200. {
                    fonts.normal
                } else {
                    fonts.small
                }
            })
            .unwrap();

        ui.push_font(font_id)
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        let font_token = self.set_font(ui);

        if flags.focused && !ui.io().want_capture_keyboard && self.config.settings.display.keyup(ui) {
            let rshift = unsafe { GetAsyncKeyState(VK_RSHIFT.0 as _) < 0 };

            self.ui_state = match (&self.ui_state, rshift) {
                (UiState::Hidden, _) => UiState::Closed,
                (_, true) => UiState::Hidden,
                (UiState::MenuOpen, _) => UiState::Closed,
                (UiState::Closed, _) => UiState::MenuOpen,
            };

            match &self.ui_state {
                UiState::MenuOpen => {},
                UiState::Closed => self.pointers.cursor_show.set(false),
                UiState::Hidden => self.pointers.cursor_show.set(false),
            }
        }

        match &self.ui_state {
            UiState::MenuOpen => {
                self.pointers.cursor_show.set(true);
                self.render_visible(ui, flags);
            },
            UiState::Closed => {
                self.render_closed(ui, flags);
            },
            UiState::Hidden => {
                self.render_hidden(ui, flags);
            },
        }

        for w in &mut self.widgets {
            if let Some(logs) = w.log() {
                let now = Instant::now();
                self.log.extend(logs.into_iter().map(|l| (now, l)));
            }
            self.log.retain(|(tm, _)| tm.elapsed() < std::time::Duration::from_secs(5));
        }

        self.render_logs(ui, flags);
        drop(font_token);
    }

    fn initialize(&mut self, ctx: &mut imgui::Context) {
        let fonts = ctx.fonts();
        self.fonts = Some(FontIDs {
            small: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../../lib/data/ComicMono.ttf"),
                size_pixels: 11.,
                config: None,
            }]),
            normal: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../../lib/data/ComicMono.ttf"),
                size_pixels: 18.,
                config: None,
            }]),
            big: fonts.add_font(&[FontSource::TtfData {
                data: include_bytes!("../../lib/data/ComicMono.ttf"),
                size_pixels: 24.,
                config: None,
            }]),
        });
    }
}

hudhook::hudhook!(PracticeTool::new().into_hook::<ImguiDx11Hooks>());
