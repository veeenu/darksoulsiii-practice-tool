use hudhook::hooks::dx11::ImguiRenderLoop;
use imgui::{Condition, Window};
struct HookYou {}

impl ImguiRenderLoop for HookYou {
    fn render(&mut self, ui: &mut imgui::Ui) {
        Window::new("Hello world")
            .position([0., 0.], Condition::FirstUseEver)
            .size([320.0, 200.0], Condition::FirstUseEver)
            .build(&ui, || {
                ui.text("Hello world!");
                ui.text("こんにちは世界！");
                ui.text("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    }
}

hudhook::hudhook!(
    {
        println!("Initializing");
        hudhook::init::alloc_console();
        hudhook::init::simplelog();
    },
    [hudhook::hooks::dx11::hook_imgui(HookYou {})]
);

