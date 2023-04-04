#![feature(lazy_cell)]

use std::fmt::Write;

use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::hooks::{ImguiRenderLoop, ImguiRenderLoopFlags};
use imgui::*;
use libds3::prelude::*;

struct ParamTinkerer {
    pointers: PointerChains,
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
            pointers: PointerChains::new(),
        }
    }
}

impl ImguiRenderLoop for ParamTinkerer {
    fn render(&mut self, ui: &mut imgui::Ui, _: &ImguiRenderLoopFlags) {
        if ui.is_key_index_released(0x50) {
            // P key
            self.shown = !self.shown;
            self.pointers.cursor_show.set(self.shown);
        }

        if !self.shown {
            return;
        }

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
        const COLUMN1: f32 = 240.;
        const COLUMN2: f32 = 480.;
        const COLUMN3: f32 = 480.;

        ui.child_window("##param_child_wnd")
            .flags(WindowFlags::NO_SCROLLBAR)
            .size([COLUMN1 + COLUMN2 + COLUMN3 + 10., 405.])
            .build(|| {
                ui.columns(3, "##param_columns", false);
                ui.set_column_offset(0, 0.);
                ui.set_column_offset(1, COLUMN1 + 10.);
                ui.set_column_offset(1, COLUMN1 + COLUMN2 + 20.);

                let param_entries = {
                    ui.set_current_column_width(COLUMN1 + 10.);

                    let _ = ui.push_item_width(-1.);
                    ListBox::new("##param_names").size([COLUMN1, 400.]).build(ui, || {
                        for (idx, k) in params.keys().enumerate() {
                            if ui.selectable_config(k).selected(idx == self.selected_param).build() {
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
                    ui.set_current_column_width(COLUMN2 + 10.);

                    let mut buf = String::new();
                    let _ = ui.push_item_width(-1.);
                    ListBox::new("##param_ids").size([COLUMN2, 400.]).build(ui, || {
                        for (idx, id) in param_entries.enumerate() {
                            buf.clear();
                            if let Some(id_name) = PARAM_NAMES
                                .get(param_name)
                                .and_then(|param_id_names| param_id_names.get(&(id as usize)))
                            {
                                write!(buf, "{id} - {id_name}").ok();
                            } else {
                                write!(buf, "{id}").ok();
                            }

                            if ui.selectable_config(&buf)
                                .selected(idx == self.selected_param_id)
                                .build()
                            {
                                self.selected_param_id = idx;
                            }
                        }
                    });

                    (param_name, self.selected_param_id)
                });

                if let Some((param_name, param_idx)) = param_item {
                    struct ImguiParamVisitor<'a>(&'a imgui::Ui);

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
                            let mut i = *v;
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

                    ui.next_column();

                    ListBox::new("##param_detail").size([COLUMN3, 400.]).build(ui, || {
                        let _token = ui.push_item_width(120.);
                        params.visit_param_item(param_name, param_idx, &mut ImguiParamVisitor(ui));
                    });
                };
            });
    }
}

hudhook::hudhook!(ParamTinkerer::new().into_hook::<ImguiDx11Hooks>());
