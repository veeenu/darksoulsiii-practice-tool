#![feature(once_cell)]

use std::path::Path;

use imgui::*;

use hudhook::hooks::dx12::{ImguiRenderLoop, ImguiRenderLoopFlags};

struct PracticeTool {}

impl PracticeTool {
    fn new() -> Self {
        use simplelog::*;

        hudhook::utils::alloc_console();
        log_panics::init();

        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed),
            WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                std::fs::File::create(Path::new("eldenring-practice-tool.log")).unwrap(),
            ),
        ])
        .ok();

        info!("Hello world");
        PracticeTool {}
    }
}

impl ImguiRenderLoop for PracticeTool {
    fn render(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        Window::new("##window").build(ui, || {
            ui.text("It works!");
        });
    }
}

hudhook::hudhook!(|| { hudhook::hooks::dx12::hook_imgui(PracticeTool::new()) });
