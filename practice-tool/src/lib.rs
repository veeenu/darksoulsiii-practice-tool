use hudhook::hooks::dx11::ImguiRenderLoop;
use imgui::*;
struct HookYou {}

impl ImguiRenderLoop for HookYou {
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
                ui.text(
                    "johndisandonato's Dark Souls III Practice Tool is active"
                );
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
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
