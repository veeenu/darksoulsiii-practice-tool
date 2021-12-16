#![feature(once_cell)]

mod params;

use std::time::Instant;

use imgui::*;

use hudhook::hooks::dx11::{ImguiRenderLoop, ImguiRenderLoopFlags};

struct CarthusCurvedSwordMod {
    note: String,
    shortsword: &'static mut params::EquipParamWeapon,
    sid: i16,
}
unsafe impl Send for CarthusCurvedSwordMod {}
unsafe impl Sync for CarthusCurvedSwordMod {}

impl CarthusCurvedSwordMod {
    fn new() -> Self {
        use simplelog::*;
        log_panics::init();
        hudhook::utils::alloc_console();

        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Debug, // config.settings.log_level.to_level_filter(),
            Config::default(),
            TerminalMode::Mixed,
        )])
        .ok();

        let params = unsafe { params::Params::new() }.unwrap();

        info!("Params");

        unsafe { params.get_equip_param_goods() }
            .and_then(|equip_param_goods| {
                equip_param_goods
                    .filter_map(|i| {
                        if i.id >= 150 && i.id <= 171 {
                            i.param
                        } else {
                            None
                        }
                    })
                    .for_each(|mut estus| {
                        println!("{:?}", estus);
                        estus.icon_id = 117;
                        estus.set_is_supple_item(true);
                        estus.set_is_full_supple_item(false);
                    });

                Some(())
            })
            .unwrap();

        info!("Estus");

        let shortsword = unsafe { params.get_equip_param_weapon() }
            .and_then(|mut equip_param_weapon| equip_param_weapon.find(|i| i.id == 2000000))
            .and_then(|param| param.param)
            .and_then(|mut shortsword| {
                shortsword.atk_base_physics = i16::MAX;
                shortsword.atk_base_fire = i16::MAX;
                shortsword.atk_base_thunder = i16::MAX;
                shortsword.atk_base_magic = i16::MAX;
                shortsword.atk_base_stamina = 10;

                Some(shortsword)
            })
            .unwrap();

        info!("Shortsword");

        unsafe { params.get_item_lot_param() }
            .and_then(|mut item_lot_param| item_lot_param.find(|i| i.id == 11700000))
            .and_then(|param| param.param)
            .and_then(|ccs_drop| {
                ccs_drop.lot_item_base_point01 = 0;
                ccs_drop.lot_item_base_point02 = 1000;
                Some(())
            });

        let note = format!("");

        CarthusCurvedSwordMod {
            note,
            shortsword,
            sid: 6,
        }
    }

    fn render_closed(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        if ui.is_key_index_released(32) {
            self.sid += 1
        }

        self.shortsword.icon_id = self.sid;

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
                ui.text("johndisandonato's CCS gadget is active");
                // for k in ui.io().keys_down.chunks(16) {
                //     ui.text(format!("{:?}", k));
                // }
                ui.text(format!("{}", self.sid));
            });

        for st in stack_tokens.into_iter().rev() {
            st.pop();
        }
    }
}

impl ImguiRenderLoop for CarthusCurvedSwordMod {
    fn render(&mut self, ui: &mut imgui::Ui, flags: &ImguiRenderLoopFlags) {
        self.render_closed(ui, flags);
    }
}

hudhook::hudhook!(|| {
    [hudhook::hooks::dx11::hook_imgui(
        CarthusCurvedSwordMod::new(),
    )]
});
