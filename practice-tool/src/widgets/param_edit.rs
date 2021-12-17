use imgui::{Condition, PopupModal, WindowFlags};
use libds3::Params;

use super::Widget;

pub(crate) struct ParamEdit<'a> {
    params: &'a Params,
}

impl<'a> std::fmt::Debug for ParamEdit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParamEdit {{}}")
    }
}

impl<'a> ParamEdit<'a> {
    pub(crate) fn new(params: &'a Params) -> Self {
        ParamEdit { params }
    }
}

impl<'a> Widget for ParamEdit<'a> {
    fn render(&mut self, ui: &imgui::Ui) {
        if ui.button_with_size("Param edit", [super::BUTTON_WIDTH, super::BUTTON_HEIGHT]) {
            ui.open_popup("##param_edit_popup");
        }
        let [cx, cy] = ui.cursor_pos();
        let [wx, wy] = ui.window_pos();
        let [x, y] = [cx + wx, cy + wy - super::BUTTON_HEIGHT];
        unsafe {
            imgui_sys::igSetNextWindowPos(
                imgui_sys::ImVec2 { x, y },
                Condition::Always as _,
                imgui_sys::ImVec2 { x: 0., y: 0. },
            )
        };

        let style_tokens =
            [ui.push_style_color(imgui::StyleColor::ModalWindowDimBg, [0., 0., 0., 0.])];

        if let Some(_token) = PopupModal::new("##param_edit_popup")
            .flags(
                WindowFlags::NO_TITLE_BAR
                    | WindowFlags::NO_RESIZE
                    | WindowFlags::NO_MOVE
                    | WindowFlags::NO_SCROLLBAR
                    | WindowFlags::ALWAYS_AUTO_RESIZE,
            )
            .begin_popup(ui)
        {
            self.params.render_imgui(ui);
        }

        style_tokens.into_iter().rev().for_each(|t| t.pop());
    }
}
