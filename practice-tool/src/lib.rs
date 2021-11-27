#![feature(once_cell)]

mod memedit;
mod pointers;
mod style;
mod util;
mod widgets;

use std::lazy::SyncLazy;

use hudhook::hooks::dx11::ImguiRenderLoop;

use imgui::*;
use pointers::PointerChains;
use util::get_key_code;

struct PracticeTool {
    // pointers: SyncLazy<pointers::PointerChains>,
    widgets: SyncLazy<widgets::Flag>,
}

impl PracticeTool {
    fn new() -> Self {
        PracticeTool {
            widgets: SyncLazy::new(|| {
                let pointers: PointerChains = pointers::detect_version()
                    .expect("Couldn't detect version!")
                    .into();

                widgets::flag::Flag::new(pointers.deathcam, get_key_code("P").unwrap())
            }),
            // pointers: SyncLazy::new(|| {
            //     pointers::detect_version()
            //         .expect("Couldn't detect version!")
            //         .into()
            // }),
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

                self.widgets.render(ui);
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

fn init() {
    use log_panics;
    use simplelog::*;
    hudhook::init::alloc_console();

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
                    LevelFilter::Debug, // config.settings.log_level.to_level_filter(),
                    Config::default(),
                    TerminalMode::Mixed,
                ),
                WriteLogger::new(
                    LevelFilter::Debug, // config.settings.log_level.to_level_filter(),
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

    log_panics::init();
}

hudhook::hudhook!(
    {
        init();
    },
    [hudhook::hooks::dx11::hook_imgui(PracticeTool::new())]
);
