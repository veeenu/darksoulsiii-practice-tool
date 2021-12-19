#![feature(once_cell)]

use std::fmt::Write;

use hudhook::hooks::dx11::{ImguiRenderLoop, ImguiRenderLoopFlags};
use libds3::pointers::MOUSE_ENABLE;
use libds3::{ParamVisitor, PARAMS};

use imgui::*;

struct ParamTinkerer {
    shown: bool,
    selected_param: usize,
    selected_param_id: usize,
}

impl ParamTinkerer {
    fn new() -> Self {
        println!("Initializing");
        hudhook::utils::alloc_console();
        hudhook::utils::simplelog();

        ParamTinkerer {
            shown: false,
            selected_param: 0,
            selected_param_id: 0,
        }
    }
}

impl ImguiRenderLoop for ParamTinkerer {
    fn render(&mut self, ui: &mut imgui_dx11::imgui::Ui, _: &ImguiRenderLoopFlags) {
        if ui.is_key_index_released(0x50) {
            // P key
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

                self.render_params(ui);

                style_tokens.into_iter().rev().for_each(|t| t.pop());
            });
    }
}

impl ParamTinkerer {
    pub fn render_params(&mut self, ui: &imgui::Ui) {
        let params = PARAMS.write();

        ChildWindow::new("##param_child_wnd")
            .size([250. * 3., 250.])
            .build(ui, || {
                ui.columns(3, "##param_columns", false);

                let param_entries = {
                    ListBox::new("##param_names")
                        .size([240., 240.])
                        .build(ui, || {
                            for (idx, k) in params.keys().enumerate() {
                                if Selectable::new(k)
                                    .selected(idx == self.selected_param)
                                    .build(ui)
                                {
                                    self.selected_param = idx;
                                }
                            }
                        });

                    params
                        .keys()
                        .nth(self.selected_param)
                        .and_then(|k| unsafe { params.iter_param_ids(k) }.map(|v| (k, v)))
                };

                let param_item = param_entries.map(|(param_name, param_entries)| {
                    ui.next_column();
                    ui.set_current_column_width(130.);

                    let mut buf = String::new();
                    ListBox::new("##param_ids")
                        .size([120., 240.])
                        .build(ui, || {
                            for (idx, id) in param_entries.enumerate() {
                                write!(buf, "{}", id).ok();
                                if Selectable::new(&buf)
                                    .selected(idx == self.selected_param_id)
                                    .build(ui)
                                {
                                    self.selected_param_id = idx;
                                }
                                buf.clear();
                            }
                        });

                    (param_name, self.selected_param_id)
                });

                if let Some((param_name, param_idx)) = param_item {
                    ui.next_column();
                    ui.set_current_column_width(370.);

                    struct ImguiParamVisitor<'a>(&'a imgui::Ui<'a>);

                    impl<'a> ParamVisitor for ImguiParamVisitor<'a> {
                        fn visit_u8(&mut self, name: &str, v: &mut u8) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_u16(&mut self, name: &str, v: &mut u16) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_u32(&mut self, name: &str, v: &mut u32) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_i8(&mut self, name: &str, v: &mut i8) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_i16(&mut self, name: &str, v: &mut i16) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_i32(&mut self, name: &str, v: &mut i32) {
                            let mut i = *v as i32;
                            self.0.input_int(name, &mut i).build();
                            *v = i as _;
                        }

                        fn visit_f32(&mut self, name: &str, v: &mut f32) {
                            self.0.input_float(name, v).build();
                        }

                        fn visit_bool(&mut self, name: &str, v: &mut bool) {
                            self.0.checkbox(name, v);
                        }
                    }

                    ListBox::new("##param_detail")
                        .size([360., 240.])
                        .build(ui, || {
                            let token = ui.push_item_width(120.);
                            params.visit_param_item(
                                param_name,
                                param_idx,
                                &mut ImguiParamVisitor(ui),
                            );
                            drop(token);
                        });
                };
            });

    }
}

hudhook::hudhook!(|| { [hudhook::hooks::dx11::hook_imgui(ParamTinkerer::new())] });
