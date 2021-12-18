#![feature(once_cell)]
use imgui::{Condition, Window, WindowFlags};
use libds3::{PARAMS, pointers::MOUSE_ENABLE};

use hudhook::hooks::dx11::{ImguiRenderLoop, ImguiRenderLoopFlags};
struct ParamTinkerer {
    shown: bool,
}

impl ParamTinkerer {
    fn new() -> Self {
        println!("Initializing");
        hudhook::utils::alloc_console();
        hudhook::utils::simplelog();

        ParamTinkerer { shown: false }
    }
}

impl ImguiRenderLoop for ParamTinkerer {
    fn render(&mut self, ui: &mut imgui_dx11::imgui::Ui, _: &ImguiRenderLoopFlags) {
        if ui.is_key_index_released(0x30) {
            self.shown = !self.shown;
            MOUSE_ENABLE.write(if self.shown { 1 } else { 0 });
        }

        if !self.shown {
            return;
        }

        Window::new("##tool_window")
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
                let style_tokens =
                    [ui.push_style_color(imgui::StyleColor::ModalWindowDimBg, [0., 0., 0., 0.])];
                 
                PARAMS.render_imgui(ui);

                style_tokens.into_iter().rev().for_each(|t| t.pop());
            });
    }
}

hudhook::hudhook!(|| { [hudhook::hooks::dx11::hook_imgui(ParamTinkerer::new())] });
