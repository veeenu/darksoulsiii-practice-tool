use super::*;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ActionButtonParam {
    pub(crate) region_type: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) dummy_poly1: i32,
    pub(crate) dummy_poly2: i32,
    pub(crate) radius: f32,
    pub(crate) angle: i32,
    pub(crate) depth: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) base_height_offset: f32,
    pub(crate) angle_check_type: u8,
    pub(crate) pad2: [u8; 3],
    pub(crate) allow_angle: i32,
    pub(crate) text_box_type: u8,
    pub(crate) pad3: [u8; 3],
    pub(crate) text_id: i32,
    pub(crate) invalid_flag: i32,
    pub(crate) grayout_flag: i32,
    pub(crate) priority: i32,
    pub(crate) exec_invalid_time: f32,
    pub(crate) exec_button_circle: u8,
    pub(crate) pad4: [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AiSoundParam {
    pub(crate) radius: f32,
    pub(crate) life_frame: f32,
    pub(crate) b_sp_effect_enable: u8,
    pub(crate) r#type: u8,
    pub(crate) fake_target_type: u8,
    pub(crate) interest_category: u8,
    pub(crate) use_hit_damage_team: u8,
    pub(crate) pad: [u8; 19],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AtkParam_Npc {
    pub(crate) hit0_radius: f32,
    pub(crate) hit1_radius: f32,
    pub(crate) hit2_radius: f32,
    pub(crate) hit3_radius: f32,
    pub(crate) knockback_dist: f32,
    pub(crate) hit_stop_time: f32,
    pub(crate) sp_effect0: i32,
    pub(crate) sp_effect1: i32,
    pub(crate) sp_effect2: i32,
    pub(crate) sp_effect3: i32,
    pub(crate) sp_effect4: i32,
    pub(crate) hit0_dmy_poly1: i16,
    pub(crate) hit1_dmy_poly1: i16,
    pub(crate) hit2_dmy_poly1: i16,
    pub(crate) hit3_dmy_poly1: i16,
    pub(crate) hit0_dmy_poly2: i16,
    pub(crate) hit1_dmy_poly2: i16,
    pub(crate) hit2_dmy_poly2: i16,
    pub(crate) hit3_dmy_poly2: i16,
    pub(crate) blowing: i16,
    pub(crate) atk_phys_correction: i16,
    pub(crate) atk_mag_correction: i16,
    pub(crate) atk_fire_correction: i16,
    pub(crate) atk_thun_correction: i16,
    pub(crate) atk_stam_correction: i16,
    pub(crate) guard_atk_rate_correction: i16,
    pub(crate) guard_break_correction: i16,
    pub(crate) atk_throw_escape_correction: i16,
    pub(crate) atk_super_armor_correction: i16,
    pub(crate) atk_phys: i16,
    pub(crate) atk_mag: i16,
    pub(crate) atk_fire: i16,
    pub(crate) atk_thun: i16,
    pub(crate) atk_stam: i16,
    pub(crate) guard_atk_rate: i16,
    pub(crate) guard_break_rate: i16,
    pub(crate) atk_super_armor: i16,
    pub(crate) atk_throw_escape: i16,
    pub(crate) atk_obj: i16,
    pub(crate) guard_stamina_cut_rate: i16,
    pub(crate) guard_rate: i16,
    pub(crate) throw_type_id: i16,
    pub(crate) hit0_hit_type: u8,
    pub(crate) hit1_hit_type: u8,
    pub(crate) hit2_hit_type: u8,
    pub(crate) hit3_hit_type: u8,
    pub(crate) hit0_priority: u8,
    pub(crate) hit1_priority: u8,
    pub(crate) hit2_priority: u8,
    pub(crate) hit3_priority: u8,
    pub(crate) damage_level: u8,
    pub(crate) map_hit_type: u8,
    pub(crate) guard_cut_cancel_rate: u8,
    pub(crate) atk_attribute: i8,
    pub(crate) sp_attribute: i8,
    pub(crate) atk_type: i8,
    pub(crate) atk_material: i8,
    pub(crate) atk_size: i8,
    pub(crate) def_material: i8,
    pub(crate) def_sfx_material: i8,
    pub(crate) hit_source_type: u8,
    pub(crate) throw_flag: u8,
    pub(crate) bitfield0: u8,
    pub(crate) atk_pow_for_sfx_se: u8,
    pub(crate) atk_dir_for_sfx_se: u8,
    pub(crate) bitfield1: u8,
    pub(crate) pad1: u8,
    pub(crate) regainable_slot_id: u8,
    pub(crate) death_cause_id: i32,
    pub(crate) decal_id1: i32,
    pub(crate) decal_id2: i32,
    pub(crate) spawn_ai_sound_id: i32,
    pub(crate) hit_ai_sound_id: i32,
    pub(crate) rumble_id0: i32,
    pub(crate) rumble_id1: i32,
    pub(crate) rumble_id2: i32,
    pub(crate) rumble_id3: i32,
    pub(crate) hit0_vfx_id: i32,
    pub(crate) hit0_dummy_poly_id0: i32,
    pub(crate) hit0_dummy_poly_id1: i32,
    pub(crate) hit1_vfx_id1: i32,
    pub(crate) hit1_dummy_poly_id0: i32,
    pub(crate) hit1_dummy_poly_id1: i32,
    pub(crate) hit2_vfx_id: i32,
    pub(crate) hit2_dummy_poly_id0: i32,
    pub(crate) hit2_dummy_poly_id1: i32,
    pub(crate) hit3_vfx_id: i32,
    pub(crate) hit3_dummy_poly_id0: i32,
    pub(crate) hit3_dummy_poly_id1: i32,
    pub(crate) hit4_vfx_id: i32,
    pub(crate) hit4_dummy_poly_id0: i32,
    pub(crate) hit4_dummy_poly_id1: i32,
    pub(crate) hit5_vfx_id: i32,
    pub(crate) hit5_dummy_poly_id0: i32,
    pub(crate) hit5_dummy_poly_id1: i32,
    pub(crate) hit6_vfx_id: i32,
    pub(crate) hit6_dummy_poly_id0: i32,
    pub(crate) hit6_dummy_poly_id1: i32,
    pub(crate) hit7_vfx_id: i32,
    pub(crate) hit7_dummy_poly_id0: i32,
    pub(crate) hit7_dummy_poly_id1: i32,
    pub(crate) hit4_radius: f32,
    pub(crate) hit5_radius: f32,
    pub(crate) hit6_radius: f32,
    pub(crate) hit7_radius: f32,
    pub(crate) hit8_radius: f32,
    pub(crate) hit9_radius: f32,
    pub(crate) hit10_radius: f32,
    pub(crate) hit11_radius: f32,
    pub(crate) hit12_radius: f32,
    pub(crate) hit13_radius: f32,
    pub(crate) hit14_radius: f32,
    pub(crate) hit15_radius: f32,
    pub(crate) hit4_dmy_poly1: i16,
    pub(crate) hit5_dmy_poly1: i16,
    pub(crate) hit6_dmy_poly1: i16,
    pub(crate) hit7_dmy_poly1: i16,
    pub(crate) hit8_dmy_poly1: i16,
    pub(crate) hit9_dmy_poly1: i16,
    pub(crate) hit10_dmy_poly1: i16,
    pub(crate) hit11_dmy_poly1: i16,
    pub(crate) hit12_dmy_poly1: i16,
    pub(crate) hit13_dmy_poly1: i16,
    pub(crate) hit14_dmy_poly1: i16,
    pub(crate) hit15_dmy_poly1: i16,
    pub(crate) hit4_dmy_poly2: i16,
    pub(crate) hit5_dmy_poly2: i16,
    pub(crate) hit6_dmy_poly2: i16,
    pub(crate) hit7_dmy_poly2: i16,
    pub(crate) hit8_dmy_poly2: i16,
    pub(crate) hit9_dmy_poly2: i16,
    pub(crate) hit10_dmy_poly2: i16,
    pub(crate) hit11_dmy_poly2: i16,
    pub(crate) hit12_dmy_poly2: i16,
    pub(crate) hit13_dmy_poly2: i16,
    pub(crate) hit14_dmy_poly2: i16,
    pub(crate) hit15_dmy_poly2: i16,
    pub(crate) hit4_hit_type: u8,
    pub(crate) hit5_hit_type: u8,
    pub(crate) hit6_hit_type: u8,
    pub(crate) hit7_hit_type: u8,
    pub(crate) hit8_hit_type: u8,
    pub(crate) hit9_hit_type: u8,
    pub(crate) hit10_hit_type: u8,
    pub(crate) hit11_hit_type: u8,
    pub(crate) hit12_hit_type: u8,
    pub(crate) hit13_hit_type: u8,
    pub(crate) hit14_hit_type: u8,
    pub(crate) hit15_hit_type: u8,
    pub(crate) field0x174: i32,
    pub(crate) field0x178: i32,
    pub(crate) field0x17_c: i32,
    pub(crate) def_material_val0: i16,
    pub(crate) def_material_val1: i16,
    pub(crate) def_material_val2: i16,
    pub(crate) atk_dark_correction: i16,
    pub(crate) atk_dark: i16,
    pub(crate) bitfield2: u8,
    pub(crate) field0x18_b: u8,
    pub(crate) phys_sp_correction: i16,
    pub(crate) mag_sp_correction: i16,
    pub(crate) fire_sp_correction: i16,
    pub(crate) thun_sp_correction: i16,
    pub(crate) damage_level_parameter: u8,
    pub(crate) field0x195: u8,
    pub(crate) dark_sp_correction: i16,
    pub(crate) atk_element_correct_id: i32,
    pub(crate) pad2: [u8; 12],
}

impl AtkParam_Npc { 
    #[allow(unused)]
    pub(crate) fn set_disable_guard(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_guard(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_stamina_attack(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_stamina_attack(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_hit_sp_effect(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_hit_sp_effect(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_ignore_notify_miss_swing_for_ai(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn ignore_notify_miss_swing_for_ai(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_repeat_hit_sfx(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn repeat_hit_sfx(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_arrow_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_arrow_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ghost_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ghost_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_disable_no_damage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_disable_no_damage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_oppose_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn oppose_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_friendly_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn friendly_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_self_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn self_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_share_hit_list(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_share_hit_list(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_check_obj_penetration(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_check_obj_penetration(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x81(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x81(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x81_0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x81_0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AtkParam_Pc {
    pub(crate) hit0_radius: f32,
    pub(crate) hit1_radius: f32,
    pub(crate) hit2_radius: f32,
    pub(crate) hit3_radius: f32,
    pub(crate) knockback_dist: f32,
    pub(crate) hit_stop_time: f32,
    pub(crate) sp_effect0: i32,
    pub(crate) sp_effect1: i32,
    pub(crate) sp_effect2: i32,
    pub(crate) sp_effect3: i32,
    pub(crate) sp_effect4: i32,
    pub(crate) hit0_dmy_poly1: i16,
    pub(crate) hit1_dmy_poly1: i16,
    pub(crate) hit2_dmy_poly1: i16,
    pub(crate) hit3_dmy_poly1: i16,
    pub(crate) hit0_dmy_poly2: i16,
    pub(crate) hit1_dmy_poly2: i16,
    pub(crate) hit2_dmy_poly2: i16,
    pub(crate) hit3_dmy_poly2: i16,
    pub(crate) blowing: i16,
    pub(crate) atk_phys_correction: i16,
    pub(crate) atk_mag_correction: i16,
    pub(crate) atk_fire_correction: i16,
    pub(crate) atk_thun_correction: i16,
    pub(crate) atk_stam_correction: i16,
    pub(crate) guard_atk_rate_correction: i16,
    pub(crate) guard_break_correction: i16,
    pub(crate) atk_throw_escape_correction: i16,
    pub(crate) atk_super_armor_correction: i16,
    pub(crate) atk_phys: i16,
    pub(crate) atk_mag: i16,
    pub(crate) atk_fire: i16,
    pub(crate) atk_thun: i16,
    pub(crate) atk_stam: i16,
    pub(crate) guard_atk_rate: i16,
    pub(crate) guard_break_rate: i16,
    pub(crate) atk_super_armor: i16,
    pub(crate) atk_throw_escape: i16,
    pub(crate) atk_obj: i16,
    pub(crate) guard_stamina_cut_rate: i16,
    pub(crate) guard_rate: i16,
    pub(crate) throw_type_id: i16,
    pub(crate) hit0_hit_type: u8,
    pub(crate) hit1_hit_type: u8,
    pub(crate) hit2_hit_type: u8,
    pub(crate) hit3_hit_type: u8,
    pub(crate) hit0_priority: u8,
    pub(crate) hit1_priority: u8,
    pub(crate) hit2_priority: u8,
    pub(crate) hit3_priority: u8,
    pub(crate) damage_level: u8,
    pub(crate) map_hit_type: u8,
    pub(crate) guard_cut_cancel_rate: u8,
    pub(crate) atk_attribute: i8,
    pub(crate) sp_attribute: i8,
    pub(crate) atk_type: i8,
    pub(crate) atk_material: i8,
    pub(crate) atk_size: i8,
    pub(crate) def_material: i8,
    pub(crate) def_sfx_material: i8,
    pub(crate) hit_source_type: u8,
    pub(crate) throw_flag: u8,
    pub(crate) bitfield0: u8,
    pub(crate) atk_pow_for_sfx_se: u8,
    pub(crate) atk_dir_for_sfx_se: u8,
    pub(crate) bitfield1: u8,
    pub(crate) pad1: u8,
    pub(crate) regainable_slot_id: u8,
    pub(crate) death_cause_id: i32,
    pub(crate) decal_id1: i32,
    pub(crate) decal_id2: i32,
    pub(crate) spawn_ai_sound_id: i32,
    pub(crate) hit_ai_sound_id: i32,
    pub(crate) rumble_id0: i32,
    pub(crate) rumble_id1: i32,
    pub(crate) rumble_id2: i32,
    pub(crate) rumble_id3: i32,
    pub(crate) hit0_vfx_id: i32,
    pub(crate) hit0_dummy_poly_id0: i32,
    pub(crate) hit0_dummy_poly_id1: i32,
    pub(crate) hit1_vfx_id1: i32,
    pub(crate) hit1_dummy_poly_id0: i32,
    pub(crate) hit1_dummy_poly_id1: i32,
    pub(crate) hit2_vfx_id: i32,
    pub(crate) hit2_dummy_poly_id0: i32,
    pub(crate) hit2_dummy_poly_id1: i32,
    pub(crate) hit3_vfx_id: i32,
    pub(crate) hit3_dummy_poly_id0: i32,
    pub(crate) hit3_dummy_poly_id1: i32,
    pub(crate) hit4_vfx_id: i32,
    pub(crate) hit4_dummy_poly_id0: i32,
    pub(crate) hit4_dummy_poly_id1: i32,
    pub(crate) hit5_vfx_id: i32,
    pub(crate) hit5_dummy_poly_id0: i32,
    pub(crate) hit5_dummy_poly_id1: i32,
    pub(crate) hit6_vfx_id: i32,
    pub(crate) hit6_dummy_poly_id0: i32,
    pub(crate) hit6_dummy_poly_id1: i32,
    pub(crate) hit7_vfx_id: i32,
    pub(crate) hit7_dummy_poly_id0: i32,
    pub(crate) hit7_dummy_poly_id1: i32,
    pub(crate) hit4_radius: f32,
    pub(crate) hit5_radius: f32,
    pub(crate) hit6_radius: f32,
    pub(crate) hit7_radius: f32,
    pub(crate) hit8_radius: f32,
    pub(crate) hit9_radius: f32,
    pub(crate) hit10_radius: f32,
    pub(crate) hit11_radius: f32,
    pub(crate) hit12_radius: f32,
    pub(crate) hit13_radius: f32,
    pub(crate) hit14_radius: f32,
    pub(crate) hit15_radius: f32,
    pub(crate) hit4_dmy_poly1: i16,
    pub(crate) hit5_dmy_poly1: i16,
    pub(crate) hit6_dmy_poly1: i16,
    pub(crate) hit7_dmy_poly1: i16,
    pub(crate) hit8_dmy_poly1: i16,
    pub(crate) hit9_dmy_poly1: i16,
    pub(crate) hit10_dmy_poly1: i16,
    pub(crate) hit11_dmy_poly1: i16,
    pub(crate) hit12_dmy_poly1: i16,
    pub(crate) hit13_dmy_poly1: i16,
    pub(crate) hit14_dmy_poly1: i16,
    pub(crate) hit15_dmy_poly1: i16,
    pub(crate) hit4_dmy_poly2: i16,
    pub(crate) hit5_dmy_poly2: i16,
    pub(crate) hit6_dmy_poly2: i16,
    pub(crate) hit7_dmy_poly2: i16,
    pub(crate) hit8_dmy_poly2: i16,
    pub(crate) hit9_dmy_poly2: i16,
    pub(crate) hit10_dmy_poly2: i16,
    pub(crate) hit11_dmy_poly2: i16,
    pub(crate) hit12_dmy_poly2: i16,
    pub(crate) hit13_dmy_poly2: i16,
    pub(crate) hit14_dmy_poly2: i16,
    pub(crate) hit15_dmy_poly2: i16,
    pub(crate) hit4_hit_type: u8,
    pub(crate) hit5_hit_type: u8,
    pub(crate) hit6_hit_type: u8,
    pub(crate) hit7_hit_type: u8,
    pub(crate) hit8_hit_type: u8,
    pub(crate) hit9_hit_type: u8,
    pub(crate) hit10_hit_type: u8,
    pub(crate) hit11_hit_type: u8,
    pub(crate) hit12_hit_type: u8,
    pub(crate) hit13_hit_type: u8,
    pub(crate) hit14_hit_type: u8,
    pub(crate) hit15_hit_type: u8,
    pub(crate) field0x174: i32,
    pub(crate) field0x178: i32,
    pub(crate) field0x17_c: i32,
    pub(crate) def_material_val0: i16,
    pub(crate) def_material_val1: i16,
    pub(crate) def_material_val2: i16,
    pub(crate) atk_dark_correction: i16,
    pub(crate) atk_dark: i16,
    pub(crate) bitfield2: u8,
    pub(crate) field0x18_b: u8,
    pub(crate) phys_sp_correction: i16,
    pub(crate) mag_sp_correction: i16,
    pub(crate) fire_sp_correction: i16,
    pub(crate) thun_sp_correction: i16,
    pub(crate) damage_level_parameter: u8,
    pub(crate) field0x195: u8,
    pub(crate) dark_sp_correction: i16,
    pub(crate) atk_element_correct_id: i32,
    pub(crate) pad2: [u8; 12],
}

impl AtkParam_Pc { 
    #[allow(unused)]
    pub(crate) fn set_disable_guard(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_guard(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_stamina_attack(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_stamina_attack(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_hit_sp_effect(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_hit_sp_effect(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_ignore_notify_miss_swing_for_ai(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn ignore_notify_miss_swing_for_ai(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_repeat_hit_sfx(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn repeat_hit_sfx(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_arrow_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_arrow_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ghost_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ghost_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_disable_no_damage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_disable_no_damage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_oppose_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn oppose_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_friendly_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn friendly_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_self_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn self_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_share_hit_list(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_share_hit_list(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_check_obj_penetration(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_check_obj_penetration(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x81(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x81(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x81_0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x81_0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_charge_atk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_charge_atk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x18_a_4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x18_a_4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AttackElementCorrectParam {
    pub(crate) bitfield0: u32,
    pub(crate) add_rate0: i16,
    pub(crate) add_rate1: i16,
    pub(crate) add_rate2: i16,
    pub(crate) add_rate3: i16,
    pub(crate) add_rate4: i16,
    pub(crate) add_rate5: i16,
    pub(crate) add_rate6: i16,
    pub(crate) add_rate7: i16,
    pub(crate) add_rate8: i16,
    pub(crate) add_rate9: i16,
    pub(crate) add_rate10: i16,
    pub(crate) add_rate11: i16,
    pub(crate) add_rate12: i16,
    pub(crate) add_rate13: i16,
    pub(crate) add_rate14: i16,
    pub(crate) add_rate15: i16,
    pub(crate) add_rate16: i16,
    pub(crate) add_rate17: i16,
    pub(crate) add_rate18: i16,
    pub(crate) add_rate19: i16,
    pub(crate) add_rate20: i16,
    pub(crate) add_rate21: i16,
    pub(crate) add_rate22: i16,
    pub(crate) add_rate23: i16,
    pub(crate) add_rate24: i16,
    pub(crate) corr_rate0: i16,
    pub(crate) corr_rate1: i16,
    pub(crate) corr_rate2: i16,
    pub(crate) corr_rate3: i16,
    pub(crate) corr_rate4: i16,
    pub(crate) corr_rate5: i16,
    pub(crate) corr_rate6: i16,
    pub(crate) corr_rate7: i16,
    pub(crate) corr_rate8: i16,
    pub(crate) corr_rate9: i16,
    pub(crate) corr_rate10: i16,
    pub(crate) corr_rate11: i16,
    pub(crate) corr_rate12: i16,
    pub(crate) corr_rate13: i16,
    pub(crate) corr_rate14: i16,
    pub(crate) corr_rate15: i16,
    pub(crate) corr_rate16: i16,
    pub(crate) corr_rate17: i16,
    pub(crate) corr_rate18: i16,
    pub(crate) corr_rate19: i16,
    pub(crate) corr_rate20: i16,
    pub(crate) corr_rate21: i16,
    pub(crate) corr_rate22: i16,
    pub(crate) corr_rate23: i16,
    pub(crate) corr_rate24: i16,
    pub(crate) pad1: [u8; 24],
}

impl AttackElementCorrectParam { 
    #[allow(unused)]
    pub(crate) fn set_field0x00(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_2(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_2(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_3(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_3(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_4(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_4(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_5(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_5(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_6(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_6(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_7(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 8;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_7(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 8;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_8(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 9;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_8(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 9;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_9(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 10;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_9(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 10;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_10(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 11;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_10(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 11;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_11(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 12;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_11(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 12;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_12(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 13;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_12(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 13;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_13(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 14;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_13(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 14;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_14(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 15;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_14(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 15;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_15(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 16;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_15(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 16;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_16(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 17;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_16(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 17;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_17(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 18;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_17(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 18;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_18(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 19;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_18(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 19;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_19(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 20;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_19(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 20;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_20(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 21;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_20(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 21;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_21(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 22;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_21(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 22;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_22(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 23;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_22(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 23;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_23(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 24;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_23(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 24;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_24(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 25;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_24(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 25;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_25(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 26;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_25(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 26;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_26(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 27;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_26(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 27;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_27(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 28;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_27(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 28;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_28(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 29;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_28(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 29;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_29(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 30;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_29(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 30;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x00_30(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 31;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x00_30(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 31;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct BehaviorParam {
    pub(crate) variation_id: i32,
    pub(crate) behavior_judge_id: i32,
    pub(crate) ez_state_behavior_type_old: u8,
    pub(crate) ref_type: u8,
    pub(crate) pad1: [u8; 2],
    pub(crate) ref_id: i32,
    pub(crate) sfx_variation_id: i32,
    pub(crate) stamina: i32,
    pub(crate) mp: i32,
    pub(crate) category: u8,
    pub(crate) hero_point: u8,
    pub(crate) pad2: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct BehaviorParam_PC {
    pub(crate) variation_id: i32,
    pub(crate) behavior_judge_id: i32,
    pub(crate) ez_state_behavior_type_old: u8,
    pub(crate) ref_type: u8,
    pub(crate) pad1: [u8; 2],
    pub(crate) ref_id: i32,
    pub(crate) sfx_variation_id: i32,
    pub(crate) stamina: i32,
    pub(crate) mp: i32,
    pub(crate) category: u8,
    pub(crate) hero_point: u8,
    pub(crate) pad2: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct BonfireWarpParam {
    pub(crate) location_event_id: i32,
    pub(crate) warp_event_id: i32,
    pub(crate) bonfire_name_id: i32,
    pub(crate) description_text_id: i32,
    pub(crate) picture_id: i32,
    pub(crate) list_id: u8,
    pub(crate) is_disable_quickwarp: u8,
    pub(crate) ceremony_id: i16,
    pub(crate) online_area_id: i32,
    pub(crate) pad1: [u8; 36],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct BudgetParam {
    pub(crate) memory_budget0: f32,
    pub(crate) memory_budget1: f32,
    pub(crate) memory_budget2: f32,
    pub(crate) memory_budget3: f32,
    pub(crate) memory_budget4: f32,
    pub(crate) memory_budget5: f32,
    pub(crate) memory_budget6: f32,
    pub(crate) memory_budget7: f32,
    pub(crate) memory_budget8: f32,
    pub(crate) memory_budget9: f32,
    pub(crate) memory_budget10: f32,
    pub(crate) memory_budget11: f32,
    pub(crate) memory_budget12: f32,
    pub(crate) memory_budget13: f32,
    pub(crate) memory_budget14: f32,
    pub(crate) memory_budget15: f32,
    pub(crate) memory_budget16: f32,
    pub(crate) memory_budget17: f32,
    pub(crate) memory_budget18: f32,
    pub(crate) memory_budget19: f32,
    pub(crate) memory_budget20: f32,
    pub(crate) memory_budget21: f32,
    pub(crate) memory_budget22: f32,
    pub(crate) memory_budget23: f32,
    pub(crate) memory_budget24: f32,
    pub(crate) memory_budget25: f32,
    pub(crate) pad1: [u8; 28],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Bullet {
    pub(crate) atk_bullet_id: i32,
    pub(crate) sfx_id_bullet: i32,
    pub(crate) sfx_id_hit: i32,
    pub(crate) sfx_id_flick: i32,
    pub(crate) life: f32,
    pub(crate) dist: f32,
    pub(crate) shoot_interval: f32,
    pub(crate) gravity_in_range: f32,
    pub(crate) gravity_out_range: f32,
    pub(crate) homing_stop_range: f32,
    pub(crate) init_vellocity: f32,
    pub(crate) accel_in_range: f32,
    pub(crate) accel_out_range: f32,
    pub(crate) max_vellocity: f32,
    pub(crate) min_vellocity: f32,
    pub(crate) accel_time: f32,
    pub(crate) homing_begin_dist: f32,
    pub(crate) hit_radius: f32,
    pub(crate) hit_radius_max: f32,
    pub(crate) spread_time: f32,
    pub(crate) exp_delay: f32,
    pub(crate) homing_offset_range: f32,
    pub(crate) dmg_hit_record_life_time: f32,
    pub(crate) external_force: f32,
    pub(crate) sp_effect_id_for_shooter: i32,
    pub(crate) auto_search_npc_think_id: i32,
    pub(crate) hit_bullet_id: i32,
    pub(crate) sp_effect_id0: i32,
    pub(crate) sp_effect_id1: i32,
    pub(crate) sp_effect_id2: i32,
    pub(crate) sp_effect_id3: i32,
    pub(crate) sp_effect_id4: i32,
    pub(crate) num_shoot: i16,
    pub(crate) homing_angle: i16,
    pub(crate) shoot_angle: i16,
    pub(crate) shoot_angle_interval: i16,
    pub(crate) shoot_angle_xinterval: i16,
    pub(crate) damage_damp: u8,
    pub(crate) magic_damage_damp: u8,
    pub(crate) fire_damage_damp: u8,
    pub(crate) thunder_damage_damp: u8,
    pub(crate) stamina_damp: u8,
    pub(crate) knockback_damp: u8,
    pub(crate) shoot_angle_xz: u8,
    pub(crate) lock_shoot_limit_ang: u8,
    pub(crate) is_penetrate: u8,
    pub(crate) prev_vellocity_dir_rate: u8,
    pub(crate) atk_attribute: u8,
    pub(crate) sp_attribute: u8,
    pub(crate) material_attack_type: u8,
    pub(crate) material_attack_material: u8,
    pub(crate) material_size: u8,
    pub(crate) launch_condition_type: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) dark_damage_damp: u8,
    pub(crate) bullet_hit_type0: u8,
    pub(crate) bullet_hit_type1: u8,
    pub(crate) sp_homing_yangle: f32,
    pub(crate) rand_shoot_angle_xz: f32,
    pub(crate) rand_shoot_angle_y: f32,
    pub(crate) bullet_emitter_bullet_id: i32,
    pub(crate) bullet_emitter_shoot_interval0: f32,
    pub(crate) bullet_emitter_init_interval: f32,
    pub(crate) sp_homing_yangle_for_enemy: f32,
    pub(crate) bullet_emitter_shoot_interval1: f32,
    pub(crate) launch_type: u8,
    pub(crate) limit_bullet_param_id: u8,
    pub(crate) counter_hit_type: u8,
    pub(crate) bitfield3: u8,
    pub(crate) emitte_pos_radius: f32,
    pub(crate) hit_object_id: i32,
    pub(crate) sub_life: f32,
    pub(crate) sub_homing_angle: i16,
    pub(crate) pad1: [u8; 2],
    pub(crate) lock_shoot_correction_ang: f32,
    pub(crate) pad2: [u8; 40],
}

impl Bullet { 
    #[allow(unused)]
    pub(crate) fn set_follow_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn follow_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_follow_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn follow_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_follow_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn follow_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_emitte_pos_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn emitte_pos_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_emitte_pos_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn emitte_pos_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_emitte_pos_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn emitte_pos_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_attack_sfx(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_attack_sfx(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_endless_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_endless_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_penetrate_map(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_penetrate_map(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_hit_both_team(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_hit_both_team(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_use_shard_hit_list(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_use_shard_hit_list(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_use_multi_dmy_poly_if_place(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_use_multi_dmy_poly_if_place(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_attach_effect_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn attach_effect_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_attach_effect_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn attach_effect_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_hit_force_magic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_hit_force_magic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ignore_sfx_if_hit_water(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ignore_sfx_if_hit_water(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ignore_move_state_if_hit_water(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ignore_move_state_if_hit_water(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_hit_dark_force_magic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_hit_dark_force_magic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_inherit_effect_to_child(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_inherit_effect_to_child(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_inherit_speed_to_child(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_inherit_speed_to_child(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_lock_module(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_lock_module(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_search_for_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_search_for_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_emitted_bullet(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_emitted_bullet(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_inherit_life_to_child(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_inherit_life_to_child(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_pos_hit_bullet(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_pos_hit_bullet(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_attach_attack_sfx(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_attach_attack_sfx(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_sp_correction0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_sp_correction0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_sp_correction1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_sp_correction1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_penetrate_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_penetrate_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_sp_correction2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_sp_correction2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_disable_int_scale(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_disable_int_scale(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_c3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_c3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct BulletCreateLimitParam {
    pub(crate) max_ammount: u8,
    pub(crate) pad1: [u8; 31],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CalcCorrectGraph {
    pub(crate) stage_max_val0: f32,
    pub(crate) stage_max_val1: f32,
    pub(crate) stage_max_val2: f32,
    pub(crate) stage_max_val3: f32,
    pub(crate) stage_max_val4: f32,
    pub(crate) stage_max_grow_val0: f32,
    pub(crate) stage_max_grow_val1: f32,
    pub(crate) stage_max_grow_val2: f32,
    pub(crate) stage_max_grow_val3: f32,
    pub(crate) stage_max_grow_val4: f32,
    pub(crate) adj_pt_max_grow_val0: f32,
    pub(crate) adj_pt_max_grow_val1: f32,
    pub(crate) adj_pt_max_grow_val2: f32,
    pub(crate) adj_pt_max_grow_val3: f32,
    pub(crate) adj_pt_max_grow_val4: f32,
    pub(crate) init_inclination_soul: f32,
    pub(crate) adjustment_value: f32,
    pub(crate) boundry_inclination_soul: f32,
    pub(crate) boundry_value: f32,
    pub(crate) pad1: [u8; 4],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Ceremony {
    pub(crate) event_layer_id: i32,
    pub(crate) map_studio_layer_id: i32,
    pub(crate) gparam_id: i16,
    pub(crate) gparam_id_0: i16,
    pub(crate) point: i32,
    pub(crate) g_i_texture_id: i32,
    pub(crate) light: i32,
    pub(crate) is_reload: u8,
    pub(crate) is_disable_online: u8,
    pub(crate) pad1: [u8; 10],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CharacterLoadParam {
    pub(crate) chr_bnd_type: u8,
    pub(crate) ani_bnd_type: u8,
    pub(crate) tex_bnd_type: u8,
    pub(crate) beh_bnd_type: u8,
    pub(crate) snd_chr_type: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CharaInitParam {
    pub(crate) base_rec_mp: f32,
    pub(crate) base_rec_sp: f32,
    pub(crate) red_falldam: f32,
    pub(crate) soul: i32,
    pub(crate) equip_wep_right: i32,
    pub(crate) equip_subwep_right: i32,
    pub(crate) equip_wep_left: i32,
    pub(crate) equip_subwep_left: i32,
    pub(crate) equip_helm: i32,
    pub(crate) equip_armor: i32,
    pub(crate) equip_gaunt: i32,
    pub(crate) equip_leg: i32,
    pub(crate) equip_arrow: i32,
    pub(crate) equip_bolt: i32,
    pub(crate) equip_sub_arrow: i32,
    pub(crate) equip_sub_bolt: i32,
    pub(crate) equip_accessory1: i32,
    pub(crate) equip_accessory2: i32,
    pub(crate) equip_accessory3: i32,
    pub(crate) equip_accessory4: i32,
    pub(crate) equip_accessory5: i32,
    pub(crate) equip_skill_01: i32,
    pub(crate) equip_skill_02: i32,
    pub(crate) equip_skill_03: i32,
    pub(crate) equip_spell_01: i32,
    pub(crate) equip_spell_02: i32,
    pub(crate) equip_spell_03: i32,
    pub(crate) equip_spell_04: i32,
    pub(crate) equip_spell_05: i32,
    pub(crate) equip_spell_06: i32,
    pub(crate) equip_spell_07: i32,
    pub(crate) item_01: i32,
    pub(crate) item_02: i32,
    pub(crate) item_03: i32,
    pub(crate) item_04: i32,
    pub(crate) item_05: i32,
    pub(crate) item_06: i32,
    pub(crate) item_07: i32,
    pub(crate) item_08: i32,
    pub(crate) item_09: i32,
    pub(crate) item_10: i32,
    pub(crate) npc_player_face_gen_id: i32,
    pub(crate) npc_player_think_id: i32,
    pub(crate) base_hp: i16,
    pub(crate) base_mp: i16,
    pub(crate) base_sp: i16,
    pub(crate) arrow_num: i16,
    pub(crate) bolt_num: i16,
    pub(crate) sub_arrow_num: i16,
    pub(crate) sub_bolt_num: i16,
    pub(crate) q_wc_sb: i16,
    pub(crate) q_wc_mw: i16,
    pub(crate) q_wc_cd: i16,
    pub(crate) soul_lvl: i16,
    pub(crate) base_vit: i8,
    pub(crate) base_wil: i8,
    pub(crate) base_end: i8,
    pub(crate) base_str: i8,
    pub(crate) base_dex: i8,
    pub(crate) base_mag: i8,
    pub(crate) base_fai: i8,
    pub(crate) base_luc: i8,
    pub(crate) base_hero_point: i8,
    pub(crate) base_durability: i8,
    pub(crate) item_num_01: u8,
    pub(crate) item_num_02: u8,
    pub(crate) item_num_03: u8,
    pub(crate) item_num_04: u8,
    pub(crate) item_num_05: u8,
    pub(crate) item_num_06: u8,
    pub(crate) item_num_07: u8,
    pub(crate) item_num_08: u8,
    pub(crate) item_num_09: u8,
    pub(crate) item_num_10: u8,
    pub(crate) body_scale_head: u8,
    pub(crate) body_scale_breast: u8,
    pub(crate) body_scale_abdomen: u8,
    pub(crate) body_scale_arm: u8,
    pub(crate) body_scale_leg: u8,
    pub(crate) gestureid0: u8,
    pub(crate) gestureid1: u8,
    pub(crate) gestureid2: u8,
    pub(crate) gestureid3: u8,
    pub(crate) gestureid4: u8,
    pub(crate) gestureid5: u8,
    pub(crate) gestureid6: u8,
    pub(crate) npc_player_type: u8,
    pub(crate) npc_player_draw_type: u8,
    pub(crate) npc_player_sex: u8,
    pub(crate) vow_type: u8,
    pub(crate) voice_type: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) equip_wep_right_gen_id: i32,
    pub(crate) equip_subwep_right_gen_id: i32,
    pub(crate) equip_wep_left_gen_id: i32,
    pub(crate) equip_subwep_left_gen_id: i32,
    pub(crate) equip_helm_gen_id: i32,
    pub(crate) equip_armor_gen_id: i32,
    pub(crate) equip_gaunt_gen_id: i32,
    pub(crate) equip_leg_gen_id: i32,
    pub(crate) equip_wep_body_gen_id: i32,
    pub(crate) secondary_item_01: i32,
    pub(crate) secondary_item_02: i32,
    pub(crate) secondary_item_03: i32,
    pub(crate) secondary_item_04: i32,
    pub(crate) secondary_item_05: i32,
    pub(crate) secondary_item_06: i32,
    pub(crate) secondary_item_07: i32,
    pub(crate) secondary_item_08: i32,
    pub(crate) secondary_item_num_01: u8,
    pub(crate) secondary_item_num_02: u8,
    pub(crate) secondary_item_num_03: u8,
    pub(crate) secondary_item_num_04: u8,
    pub(crate) secondary_item_num_05: u8,
    pub(crate) secondary_item_num_06: u8,
    pub(crate) secondary_item_num_07: u8,
    pub(crate) secondary_item_num_08: u8,
    pub(crate) pad2: [u8; 12],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CharMakeMenuListItemParam {
    pub(crate) value: i32,
    pub(crate) caption_id: i32,
    pub(crate) icon_id: u8,
    pub(crate) pad1: [u8; 7],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CharMakeMenuTopParam {
    pub(crate) command_id: i32,
    pub(crate) face_param_id: i32,
    pub(crate) table_id: i32,
    pub(crate) view_condition: i32,
    pub(crate) preview_mode: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) menu_type: i8,
    pub(crate) pad2: [u8; 11],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ClearCountCorrectParam {
    pub(crate) h_p: f32,
    pub(crate) mana: f32,
    pub(crate) stamina: f32,
    pub(crate) phys_dmg: f32,
    pub(crate) slash_dmg: f32,
    pub(crate) blow_dmg: f32,
    pub(crate) thrust_dmg: f32,
    pub(crate) neutral: f32,
    pub(crate) magic_dmg: f32,
    pub(crate) fire_dmg: f32,
    pub(crate) thunder_dmg: f32,
    pub(crate) dark_dmg: f32,
    pub(crate) phys_resist: f32,
    pub(crate) magic_resist: f32,
    pub(crate) fire_resist: f32,
    pub(crate) thunder_resist: f32,
    pub(crate) dark_resist: f32,
    pub(crate) stamina_dmg: f32,
    pub(crate) mp_recover: f32,
    pub(crate) poison_resist: f32,
    pub(crate) toxic_resist: f32,
    pub(crate) bleed_resist: f32,
    pub(crate) curse_resist: f32,
    pub(crate) frost_resist: f32,
    pub(crate) hp_recover: f32,
    pub(crate) sub_mp_recover: f32,
    pub(crate) sub_hp_recover: f32,
    pub(crate) pad1: [u8; 20],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CoolTimeParam {
    pub(crate) limitation_time_0: f32,
    pub(crate) observation_time_0: f32,
    pub(crate) limitation_time_1: f32,
    pub(crate) observation_time_1: f32,
    pub(crate) limitation_time_2: f32,
    pub(crate) observation_time_2: f32,
    pub(crate) limitation_time_3: f32,
    pub(crate) observation_time_3: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CultSettingParam {
    pub(crate) distance: f32,
    pub(crate) angle: f32,
    pub(crate) event_flag_id: i32,
    pub(crate) coefficient: i16,
    pub(crate) cult_state1: i8,
    pub(crate) cult_state2: i8,
    pub(crate) pad1: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct DecalParam {
    pub(crate) texture_id0: i32,
    pub(crate) dmy_poly_id: i32,
    pub(crate) pitch_angle: f32,
    pub(crate) yaw_angle: f32,
    pub(crate) near_distance: f32,
    pub(crate) far_distance: f32,
    pub(crate) near_size: f32,
    pub(crate) far_size: f32,
    pub(crate) mask_sp_effect_id: i32,
    pub(crate) bitfield0: u32,
    pub(crate) random_size_min: i16,
    pub(crate) random_size_max: i16,
    pub(crate) random_roll_min: f32,
    pub(crate) random_roll_max: f32,
    pub(crate) random_pitch_min: f32,
    pub(crate) random_pitch_max: f32,
    pub(crate) random_yaw_min: f32,
    pub(crate) random_yaw_max: f32,
    pub(crate) pom_height_scale: f32,
    pub(crate) pom_sample_min: u8,
    pub(crate) pom_sample_max: u8,
    pub(crate) contrast1: u8,
    pub(crate) texture_length_type: u8,
    pub(crate) texture_width_x: f32,
    pub(crate) texture_width_z: f32,
    pub(crate) texture_life_time: f32,
    pub(crate) field0x58: i32,
    pub(crate) field0x5_c: f32,
    pub(crate) field0x60: i32,
    pub(crate) field0x64: i32,
    pub(crate) field0x68: i32,
    pub(crate) field0x6_c: i32,
    pub(crate) texture_id1: i32,
    pub(crate) texture_id2: i32,
    pub(crate) texture_id3: i32,
    pub(crate) brightness: f32,
    pub(crate) texture_id4: i32,
    pub(crate) texture_id5: i32,
    pub(crate) texture_id6: i32,
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) unk_na_m: i8,
    pub(crate) r_0: u8,
    pub(crate) g_1: u8,
    pub(crate) b_2: u8,
    pub(crate) delayed_spawn_state: i8,
    pub(crate) contrast2: f32,
    pub(crate) field0x98: f32,
    pub(crate) field0x9_c: f32,
    pub(crate) field0x_a0: u8,
    pub(crate) field0x_a1: u8,
    pub(crate) field0x_a2: u8,
    pub(crate) field0x_a3: u8,
    pub(crate) min_val: f32,
    pub(crate) max_val: f32,
    pub(crate) field0x_ac: u8,
    pub(crate) field0x_ad: u8,
    pub(crate) decal_spawn_delay: u16,
    pub(crate) bitfield1: u32,
    pub(crate) texture_spawn_delay: f32,
    pub(crate) pad1: [u8; 8],
}

impl DecalParam { 
    #[allow(unused)]
    pub(crate) fn set_random_variation_num0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn random_variation_num0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_random_variation_num1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn random_variation_num1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_random_variation_num2(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn random_variation_num2(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_random_variation_num3(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn random_variation_num3(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_replace_texture_id_by_material(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn replace_texture_id_by_material(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_dmypoly_category0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn dmypoly_category0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_dmypoly_category1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn dmypoly_category1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_decal_shape_type0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn decal_shape_type0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_decal_shape_type1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 8;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn decal_shape_type1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 8;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_decal_shape_type2(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 9;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn decal_shape_type2(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 9;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_decal_shape_type3(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 10;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn decal_shape_type3(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 10;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_deferred_decal(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 11;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_deferred_decal(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 11;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_paint_decal(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 12;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_paint_decal(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 12;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_target_attack_chr(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 13;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn target_attack_chr(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 13;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_target_damage_chr(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 14;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn target_damage_chr(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 14;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_target_other_chr(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 15;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn target_other_chr(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 15;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_target_map_obj(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 16;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn target_map_obj(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 16;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_pom(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 17;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_pom(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 17;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 18;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 18;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26_0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 19;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26_0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 19;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26_1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 20;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26_1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 20;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26_2(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 21;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26_2(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 21;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26_3(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 22;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26_3(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 22;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x26_4(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 23;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x26_4(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 23;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 24;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 24;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_5(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 25;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_5(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 25;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_6(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 26;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_6(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 26;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_7(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 27;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_7(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 27;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_8(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 28;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_8(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 28;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_9(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 29;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_9(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 29;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_10(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 30;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_10(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 30;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x27_11(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 31;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x27_11(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 31;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_0(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_0(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_1(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_1(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_2(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_2(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_3(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_3(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_contact_player(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_contact_player(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_form_cube(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_form_cube(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_4(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_4(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_5(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 8;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_5(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 8;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_6(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 9;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_6(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 9;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_7(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 10;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_7(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 10;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_8(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 11;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_8(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 11;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_9(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 12;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_9(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 12;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_10(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 13;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_10(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 13;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_11(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 14;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_11(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 14;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_12(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 15;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_12(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 15;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_13(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 16;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_13(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 16;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_14(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 17;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_14(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 17;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_15(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 18;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_15(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 18;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_16(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 19;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_16(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 19;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_17(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 20;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_17(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 20;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_18(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 21;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_18(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 21;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_19(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 22;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_19(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 22;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_20(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 23;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_20(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 23;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_21(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 24;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_21(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 24;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_22(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 25;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_22(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 25;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_23(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 26;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_23(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 26;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_24(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 27;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_24(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 27;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_25(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 28;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_25(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 28;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_26(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 29;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_26(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 29;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_27(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 30;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_27(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 30;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x_b0_28(&mut self, state: bool) {
        const FIELD_INDEX: u32 = 1 << 31;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x_b0_28(&mut self) -> bool {
        const FIELD_INDEX: u32 = 1 << 31;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct DirectionCameraParam {
    pub(crate) rumble_state: u8,
    pub(crate) pad1: [u8; 15],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct EquipMtrlSetParam {
    pub(crate) material_id01: i32,
    pub(crate) material_id02: i32,
    pub(crate) material_id03: i32,
    pub(crate) material_id04: i32,
    pub(crate) material_id05: i32,
    pub(crate) item_num01: i8,
    pub(crate) item_num02: i8,
    pub(crate) item_num03: i8,
    pub(crate) item_num04: i8,
    pub(crate) item_num05: i8,
    pub(crate) pad1: [u8; 6],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct EquipParamAccessory {
    pub(crate) ref_id0: i32,
    pub(crate) sfx_variation_id: i32,
    pub(crate) weight: f32,
    pub(crate) behavior_id: i32,
    pub(crate) basic_price: i32,
    pub(crate) sell_value: i32,
    pub(crate) sort_id: i32,
    pub(crate) qwc_id: i32,
    pub(crate) equip_model_id: i16,
    pub(crate) icon_id: i16,
    pub(crate) shop_lv: i16,
    pub(crate) trophy_sgrade_id: i16,
    pub(crate) trophy_seq_id: i16,
    pub(crate) equip_model_category: u8,
    pub(crate) equip_model_gender: u8,
    pub(crate) accessory_category: u8,
    pub(crate) ref_category: u8,
    pub(crate) sp_effect_category: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) vagrant_item_lot_id: i32,
    pub(crate) vagrant_bonus_ene_drop_item_lot_id: i32,
    pub(crate) vagrant_item_ene_drop_item_lot_id: i32,
    pub(crate) bitfield0: u8,
    pub(crate) pad2: [u8; 3],
    pub(crate) costvalue: i32,
    pub(crate) ring_compatibility_id: i16,
    pub(crate) vow_id: u8,
    pub(crate) achievement_id: u8,
    pub(crate) ref_id1: i32,
    pub(crate) ref_id2: i32,
    pub(crate) ref_id3: i32,
    pub(crate) ref_id4: i32,
    pub(crate) pad3: [u8; 8],
}

impl EquipParamAccessory { 
    #[allow(unused)]
    pub(crate) fn set_is_deposit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_deposit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_equip_out_brake(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_equip_out_brake(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_multi_drop_share(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_multi_drop_share(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_discard(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_discard(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_bool(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_bool(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x3_c(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x3_c(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x3_c_0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x3_c_0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_field0x3_c_1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn field0x3_c_1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct EquipParamGoods {
    pub(crate) ref_id1: i32,
    pub(crate) sfx_variation_id: i32,
    pub(crate) weight: f32,
    pub(crate) fragment_num: i32,
    pub(crate) sell_value: i32,
    pub(crate) replace_item_id: i32,
    pub(crate) behavior_id: i32,
    pub(crate) sort_id: i32,
    pub(crate) qwc_id: i32,
    pub(crate) yes_no_dialog_message_id: i32,
    pub(crate) magic_id: i32,
    pub(crate) icon_id: i16,
    pub(crate) model_id: i16,
    pub(crate) shop_lv: i16,
    pub(crate) comp_trophy_sed_id: i16,
    pub(crate) trophy_seq_id: i16,
    pub(crate) max_num: i16,
    pub(crate) consume_hero_point: u8,
    pub(crate) over_dexterity: u8,
    pub(crate) goods_type: u8,
    pub(crate) ref_category: u8,
    pub(crate) sp_effect_category: u8,
    pub(crate) goods_category: u8,
    pub(crate) goods_use_anim: i8,
    pub(crate) opme_menu_type: u8,
    pub(crate) use_limit_category: u8,
    pub(crate) replace_category: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) bitfield4: u8,
    pub(crate) bitfield5: u8,
    pub(crate) ref_id2: i32,
    pub(crate) reinforce_param_weapon: i32,
    pub(crate) vagrant_item_lot_id: i32,
    pub(crate) vagrant_bonus_ene_drop_item_lot_id: i32,
    pub(crate) vagrant_itemene_drop_item_lot_id: i32,
    pub(crate) ref_virtual_wep_id: i32,
    pub(crate) replace_item_id_by_sp_effect: i32,
    pub(crate) replace_trigger_sp_effect_id: i32,
    pub(crate) bitfield6: u8,
    pub(crate) supple_item_type: u8,
    pub(crate) menu_adhoc_type: u8,
    pub(crate) drop: u8,
    pub(crate) max_rep_num: i16,
    pub(crate) invade_type: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) shop_id: i32,
    pub(crate) fp_consume: i16,
    pub(crate) use_limit_category2: i16,
    pub(crate) pad2: [u8; 8],
}

impl EquipParamGoods { 
    #[allow(unused)]
    pub(crate) fn set_vow_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type8(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type8(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type9(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type9(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_live(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_live(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_gray(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_gray(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_white(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_white(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_black(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_black(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_multi(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_multi(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_offline(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_offline(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_consume(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_consume(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_auto_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_auto_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_establishment(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_establishment(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_only_one(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_only_one(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_drop(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_drop(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_deposit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_deposit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_disable_hand(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_disable_hand(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_travel_item(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_travel_item(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_supple_item(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_supple_item(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_full_supple_item(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_full_supple_item(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enhance(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enhance(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_fix_item(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_fix_item(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_mutli_drop_share(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_mutli_drop_share(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_use_at_coliseum(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_use_at_coliseum(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_use_at_outof_coliseum(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_use_at_outof_coliseum(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_bullet_max_num(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_bullet_max_num(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_hp_cure_max_num(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_hp_cure_max_num(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_auto_replenish(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_auto_replenish(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_can_multi_use(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn can_multi_use(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_guest_drop(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_guest_drop(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enchant_left_hand(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enchant_left_hand(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_apply_special_effect(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_apply_special_effect(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_load_of_cinder(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_load_of_cinder(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_play_region1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_play_region1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ladder(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ladder(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_multi_play(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_multi_play(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_selected(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_selected(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_play_region2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_play_region2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_net_penalized(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_net_penalized(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct EquipParamProtector {
    pub(crate) sort_id: i32,
    pub(crate) wandering_equip_id: i32,
    pub(crate) vagrant_item_lot_id: i32,
    pub(crate) vagrant_bonusene_drop_item_lot_id: i32,
    pub(crate) vagrant_item_ene_drop_item_lot_id: i32,
    pub(crate) fix_price: i32,
    pub(crate) basic_price: i32,
    pub(crate) sell_value: i32,
    pub(crate) weight: f32,
    pub(crate) resident_sp_effect_id1: i32,
    pub(crate) resident_sp_effect_id2: i32,
    pub(crate) resident_sp_effect_id3: i32,
    pub(crate) material_set_id: i32,
    pub(crate) parts_damage_rate: f32,
    pub(crate) correct_sarecover: f32,
    pub(crate) origin_equip_pro1: i32,
    pub(crate) origin_equip_pro2: i32,
    pub(crate) origin_equip_pro3: i32,
    pub(crate) origin_equip_pro4: i32,
    pub(crate) origin_equip_pro5: i32,
    pub(crate) origin_equip_pro6: i32,
    pub(crate) origin_equip_pro7: i32,
    pub(crate) origin_equip_pro8: i32,
    pub(crate) origin_equip_pro9: i32,
    pub(crate) origin_equip_pro10: i32,
    pub(crate) origin_equip_pro11: i32,
    pub(crate) origin_equip_pro12: i32,
    pub(crate) origin_equip_pro13: i32,
    pub(crate) origin_equip_pro14: i32,
    pub(crate) origin_equip_pro15: i32,
    pub(crate) origin_equip_pro16: i32,
    pub(crate) face_scale_m_scale_x: f32,
    pub(crate) face_scale_m_scale_z: f32,
    pub(crate) face_scale_m_max_x: f32,
    pub(crate) face_scale_m_max_z: f32,
    pub(crate) face_scale_f_scale_x: f32,
    pub(crate) face_scale_f_scale_z: f32,
    pub(crate) face_scale_f_max_x: f32,
    pub(crate) face_scale_f_max_x_0: f32,
    pub(crate) qwc_id: i32,
    pub(crate) equip_model_id: i16,
    pub(crate) icon_id_m: i16,
    pub(crate) icon_id_f: i16,
    pub(crate) knockback: i16,
    pub(crate) knockback_bounce_rate: i16,
    pub(crate) durability: i16,
    pub(crate) durability_max: i16,
    pub(crate) sa_durability: i16,
    pub(crate) def_flick_power: i16,
    pub(crate) defense_phys: i16,
    pub(crate) defense_magic: i16,
    pub(crate) defense_fire: i16,
    pub(crate) defense_thunder: i16,
    pub(crate) defense_slash: i16,
    pub(crate) defense_blow: i16,
    pub(crate) defense_thrust: i16,
    pub(crate) resist_poison: i16,
    pub(crate) resist_toxic: i16,
    pub(crate) resist_blood: i16,
    pub(crate) resist_curse: i16,
    pub(crate) reinforce_type_id: i16,
    pub(crate) comp_trophy_sed_id: i16,
    pub(crate) shop_lv: i16,
    pub(crate) knockback_param_id: u8,
    pub(crate) flick_damage_cut_rate: u8,
    pub(crate) equip_model_category: u8,
    pub(crate) equip_model_gender: u8,
    pub(crate) protector_category: u8,
    pub(crate) defense_material: u8,
    pub(crate) defense_material_sfx: u8,
    pub(crate) parts_dmg_type: u8,
    pub(crate) defense_material_weak: u8,
    pub(crate) defense_material_sfx_weak: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) bitfield4: u8,
    pub(crate) bitfield5: u8,
    pub(crate) bitfield6: u8,
    pub(crate) bitfield7: u8,
    pub(crate) phys_damage_cut_rate: f32,
    pub(crate) slash_damage_cut_rate: f32,
    pub(crate) strike_damage_cut_rate: f32,
    pub(crate) thrust_damage_cut_rate: f32,
    pub(crate) magic_damage_cut_rate: f32,
    pub(crate) fire_damage_cut_rate: f32,
    pub(crate) thunder_damage_cut_rate: f32,
    pub(crate) material_id0: u16,
    pub(crate) material_id1: u16,
    pub(crate) material_id2: u16,
    pub(crate) material_id3: u16,
    pub(crate) material_id4: u16,
    pub(crate) material_id5: u16,
    pub(crate) material_id6: u16,
    pub(crate) material_id7: u16,
    pub(crate) protector_category_id: i32,
    pub(crate) poise: f32,
    pub(crate) pad1: [u8; 4],
    pub(crate) dark_damage_cut_rate: f32,
    pub(crate) defense_dark: i16,
    pub(crate) unk6: u8,
    pub(crate) unk7: u8,
    pub(crate) unk8: i32,
    pub(crate) upper_arm_id: i32,
    pub(crate) unk9: i32,
    pub(crate) resist_frost: i16,
    pub(crate) mask00: u8,
    pub(crate) mask01: u8,
    pub(crate) mask02: u8,
    pub(crate) mask03: u8,
    pub(crate) mask04: u8,
    pub(crate) mask05: u8,
    pub(crate) mask06: u8,
    pub(crate) mask07: u8,
    pub(crate) mask08: u8,
    pub(crate) mask09: u8,
    pub(crate) mask10: u8,
    pub(crate) mask11: u8,
    pub(crate) mask12: u8,
    pub(crate) mask13: u8,
    pub(crate) mask14: u8,
    pub(crate) mask15: u8,
    pub(crate) mask16: u8,
    pub(crate) mask17: u8,
    pub(crate) mask18: u8,
    pub(crate) mask19: u8,
    pub(crate) mask20: u8,
    pub(crate) mask21: u8,
    pub(crate) mask22: u8,
    pub(crate) mask23: u8,
    pub(crate) mask24: u8,
    pub(crate) mask25: u8,
    pub(crate) mask26: u8,
    pub(crate) mask27: u8,
    pub(crate) mask28: u8,
    pub(crate) mask29: u8,
    pub(crate) mask30: u8,
    pub(crate) mask31: u8,
    pub(crate) mask32: u8,
    pub(crate) mask33: u8,
    pub(crate) mask34: u8,
    pub(crate) mask35: u8,
    pub(crate) mask36: u8,
    pub(crate) mask37: u8,
    pub(crate) mask38: u8,
    pub(crate) mask39: u8,
    pub(crate) mask40: u8,
    pub(crate) mask41: u8,
    pub(crate) mask42: u8,
    pub(crate) mask43: u8,
    pub(crate) mask44: u8,
    pub(crate) mask45: u8,
    pub(crate) mask46: u8,
    pub(crate) mask47: u8,
    pub(crate) mask48: u8,
    pub(crate) mask49: u8,
    pub(crate) mask50: u8,
    pub(crate) mask51: u8,
    pub(crate) mask52: u8,
    pub(crate) mask53: u8,
    pub(crate) mask54: u8,
    pub(crate) mask55: u8,
    pub(crate) mask56: u8,
    pub(crate) mask57: u8,
    pub(crate) mask58: u8,
    pub(crate) mask59: u8,
    pub(crate) mask60: u8,
    pub(crate) mask61: u8,
    pub(crate) mask62: u8,
    pub(crate) mask63: u8,
    pub(crate) mask64: u8,
    pub(crate) mask65: u8,
    pub(crate) mask66: u8,
    pub(crate) mask67: u8,
    pub(crate) mask68: u8,
    pub(crate) mask69: u8,
    pub(crate) mask70: u8,
    pub(crate) mask71: u8,
    pub(crate) mask72: u8,
    pub(crate) mask73: u8,
    pub(crate) mask74: u8,
    pub(crate) mask75: u8,
    pub(crate) mask76: u8,
    pub(crate) mask77: u8,
    pub(crate) mask78: u8,
    pub(crate) mask79: u8,
    pub(crate) mask80: u8,
    pub(crate) mask81: u8,
    pub(crate) mask82: u8,
    pub(crate) mask83: u8,
    pub(crate) mask84: u8,
    pub(crate) mask85: u8,
    pub(crate) mask86: u8,
    pub(crate) mask87: u8,
    pub(crate) mask88: u8,
    pub(crate) mask89: u8,
    pub(crate) mask90: u8,
    pub(crate) mask91: u8,
    pub(crate) mask92: u8,
    pub(crate) mask93: u8,
    pub(crate) mask94: u8,
    pub(crate) mask95: u8,
    pub(crate) mask96: u8,
    pub(crate) mask97: u8,
}

impl EquipParamProtector { 
    #[allow(unused)]
    pub(crate) fn set_is_deposit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_deposit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_head_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn head_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_body_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn body_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_arm_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn arm_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_leg_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn leg_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_face_scale(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_face_scale(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag00(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag00(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag01(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag01(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag02(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag02(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag03(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag03(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag04(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag04(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag05(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag05(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag06(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag06(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag07(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag07(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag08(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag08(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag09(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag09(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag16(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag16(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag17(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag17(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag18(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag18(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag19(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag19(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag20(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag20(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag21(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag21(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag22(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag22(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag23(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag23(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag24(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag24(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag25(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag25(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag26(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag26(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag27(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag27(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag28(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag28(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag29(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag29(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag30(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag30(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag31(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag31(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag32(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag32(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag33(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag33(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag34(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag34(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag35(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag35(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag36(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag36(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag37(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag37(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag38(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag38(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag39(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag39(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag40(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag40(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag41(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag41(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag42(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag42(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag43(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag43(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag44(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag44(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag45(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag45(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag46(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag46(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_flag47(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_flag47(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_multi_drop_share(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_multi_drop_share(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_simple_model_for_dlc1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn simple_model_for_dlc1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_guest_drop0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_guest_drop0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_guest_drop1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_guest_drop1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_simple_model_for_dlc2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn simple_model_for_dlc2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct EquipParamWeapon {
    pub(crate) behavior_variation_id: i32,
    pub(crate) sort_id: i32,
    pub(crate) wandering_equip_id: i32,
    pub(crate) weight: f32,
    pub(crate) weapon_weight_rate: f32,
    pub(crate) fix_price: i32,
    pub(crate) basic_price: i32,
    pub(crate) sell_value: i32,
    pub(crate) correct_strength: f32,
    pub(crate) correct_agility: f32,
    pub(crate) corret_magic: f32,
    pub(crate) corret_faith: f32,
    pub(crate) phys_guard_cut_rate: f32,
    pub(crate) mag_guard_cut_rate: f32,
    pub(crate) fire_guard_cut_rate: f32,
    pub(crate) thun_guard_cut_rate: f32,
    pub(crate) sp_effect_behavior_id0: i32,
    pub(crate) sp_effect_behavior_id1: i32,
    pub(crate) sp_effect_behavior_id2: i32,
    pub(crate) resident_sp_effect_id0: i32,
    pub(crate) resident_sp_effect_id1: i32,
    pub(crate) resident_sp_effect_id2: i32,
    pub(crate) material_set_id: i32,
    pub(crate) origin_equip_wep0: i32,
    pub(crate) origin_equip_wep1: i32,
    pub(crate) origin_equip_wep2: i32,
    pub(crate) origin_equip_wep3: i32,
    pub(crate) origin_equip_wep4: i32,
    pub(crate) origin_equip_wep5: i32,
    pub(crate) origin_equip_wep6: i32,
    pub(crate) origin_equip_wep7: i32,
    pub(crate) origin_equip_wep8: i32,
    pub(crate) origin_equip_wep9: i32,
    pub(crate) origin_equip_wep10: i32,
    pub(crate) origin_equip_wep11: i32,
    pub(crate) origin_equip_wep12: i32,
    pub(crate) origin_equip_wep13: i32,
    pub(crate) origin_equip_wep14: i32,
    pub(crate) origin_equip_wep15: i32,
    pub(crate) anti_demon_damage_rate: f32,
    pub(crate) ant_undead_damage_rate: f32,
    pub(crate) ant_hollow_damage_rate: f32,
    pub(crate) ant_abyssal_damage_rate: f32,
    pub(crate) vagrant_item_lot_id: i32,
    pub(crate) vagrant_bonus_ene_drop_item_lot_id: i32,
    pub(crate) vagrant_item_ene_drop_item_lot_id: i32,
    pub(crate) equip_model_id: i16,
    pub(crate) icon_id: i16,
    pub(crate) durability: i16,
    pub(crate) duraility_max: i16,
    pub(crate) attack_throw_escape: i16,
    pub(crate) parry_damage_life: i16,
    pub(crate) atk_base_physics: i16,
    pub(crate) atk_base_magic: i16,
    pub(crate) atk_base_fire: i16,
    pub(crate) atk_base_thunder: i16,
    pub(crate) atk_base_stamina: i16,
    pub(crate) sa_weapon_damage: i16,
    pub(crate) sa_durability: i16,
    pub(crate) guard_angle: i16,
    pub(crate) stamina_guard_def: i16,
    pub(crate) reinforce_type_id: i16,
    pub(crate) trophy_sgrade_id: i16,
    pub(crate) trophy_seq_id: i16,
    pub(crate) throw_atk_rate: i16,
    pub(crate) bow_dist_rate: i16,
    pub(crate) equip_model_category: u8,
    pub(crate) equip_model_gender: u8,
    pub(crate) weapon_category: u8,
    pub(crate) wepmotion_category: u8,
    pub(crate) guardmotion_category: u8,
    pub(crate) atk_material: u8,
    pub(crate) def_material: u8,
    pub(crate) def_sfx_material: u8,
    pub(crate) correct_type: u8,
    pub(crate) sp_attribute: u8,
    pub(crate) sp_atk_category: i16,
    pub(crate) wepmotion_one_hand_id: u8,
    pub(crate) wepmotion_both_hand_id: u8,
    pub(crate) proper_strength: u8,
    pub(crate) proper_agility: u8,
    pub(crate) proper_magic: u8,
    pub(crate) proper_faith: u8,
    pub(crate) over_strength: u8,
    pub(crate) attack_base_parry: u8,
    pub(crate) defense_base_parry: u8,
    pub(crate) guard_base_repel: u8,
    pub(crate) attack_base_repel: u8,
    pub(crate) guard_cut_cancel_rate: u8,
    pub(crate) guard_level: u8,
    pub(crate) slash_guard_cut_rate: u8,
    pub(crate) blow_guard_cut_rate: u8,
    pub(crate) thrust_guard_cut_rate: u8,
    pub(crate) poison_guard_resist: u8,
    pub(crate) toxic_guard_resist: u8,
    pub(crate) blood_guard_resist: u8,
    pub(crate) curse_guard_resist: u8,
    pub(crate) is_durability_divergence: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) unk6: u8,
    pub(crate) unk7: u8,
    pub(crate) unk8: u8,
    pub(crate) unk9: u8,
    pub(crate) unk10: u8,
    pub(crate) unk11: u8,
    pub(crate) group0_atk_vfx_id: i32,
    pub(crate) group0_dummy_poly_id0: i32,
    pub(crate) group0_dummy_poly_id1: i32,
    pub(crate) group1_atk_vfx_id: i32,
    pub(crate) group1_dummy_poly_id0: i32,
    pub(crate) group1_dummy_poly_id1: i32,
    pub(crate) group2_atk_vfx_id: i32,
    pub(crate) group2_dummy_poly_id0: i32,
    pub(crate) group2_dummy_poly_id1: i32,
    pub(crate) group3_atk_vfx_id: i32,
    pub(crate) group3_dummy_poly_id0: i32,
    pub(crate) group3_dummy_poly_id1: i32,
    pub(crate) group4_atk_vfx_id: i32,
    pub(crate) group4_dummy_poly_id0: i32,
    pub(crate) group4_dummy_poly_id1: i32,
    pub(crate) group5_atk_vfx_id: i32,
    pub(crate) group5_dummy_poly_id0: i32,
    pub(crate) group5_dummy_poly_id1: i32,
    pub(crate) group6_atk_vfx_id: i32,
    pub(crate) group6_dummy_poly_id0: i32,
    pub(crate) group6_dummy_poly_id1: i32,
    pub(crate) group7_atk_vfx_id: i32,
    pub(crate) group7_dummy_poly_id0: i32,
    pub(crate) group7_dummy_poly_id1: i32,
    pub(crate) material_val0: i16,
    pub(crate) material_val1: i16,
    pub(crate) wep_absorp_pos_id: i32,
    pub(crate) unk12: f32,
    pub(crate) bitfield4: u8,
    pub(crate) unk21: u8,
    pub(crate) unk22: u8,
    pub(crate) unk23: u8,
    pub(crate) unk24: f32,
    pub(crate) unk25: f32,
    pub(crate) dark_guard_cut_rate: f32,
    pub(crate) atk_base_dark: i16,
    pub(crate) unk26: u8,
    pub(crate) atk_throw_escape_a: u8,
    pub(crate) sword_art_act_id: i32,
    pub(crate) atk_throw_escape_b: u8,
    pub(crate) unk30: u8,
    pub(crate) unk31: u8,
    pub(crate) menu_adhoc: u8,
    pub(crate) sword_art_id: i32,
    pub(crate) correct_luck: f32,
    pub(crate) reinforce_weapon_id: i32,
    pub(crate) unk34: i16,
    pub(crate) display_type_id: i16,
    pub(crate) calc_correct_val0: f32,
    pub(crate) calc_correct_val1: f32,
    pub(crate) calc_correct_val2: f32,
    pub(crate) calc_correct_val3: f32,
    pub(crate) calc_correct_val4: f32,
    pub(crate) calc_correct_val5: f32,
    pub(crate) calc_correct_val6: f32,
    pub(crate) calc_correct_val7: f32,
    pub(crate) calc_correct_val8: f32,
    pub(crate) calc_correct_val9: f32,
    pub(crate) calc_correct_val10: f32,
    pub(crate) weapon_vfx0: i32,
    pub(crate) weapon_vfx1: i32,
    pub(crate) weapon_vfx2: i32,
    pub(crate) weapon_vfx3: i32,
    pub(crate) weapon_vfx4: i32,
    pub(crate) weapon_vfx5: i32,
    pub(crate) weapon_vfx6: i32,
    pub(crate) weapon_vfx7: i32,
    pub(crate) stamina_consume_rate: f32,
    pub(crate) unk48: f32,
    pub(crate) unk49: f32,
    pub(crate) unk50: f32,
    pub(crate) unk51: f32,
    pub(crate) unk52: f32,
    pub(crate) unk53: f32,
    pub(crate) unk54: f32,
    pub(crate) unk55: f32,
    pub(crate) unk56: i32,
    pub(crate) unk57: i32,
    pub(crate) unk58: i32,
    pub(crate) unk59: i32,
    pub(crate) unk60: i32,
    pub(crate) attack_element_correct_id: i32,
    pub(crate) shop_price: i32,
    pub(crate) unk62: u8,
    pub(crate) max_num: u8,
    pub(crate) bitfield5: u8,
    pub(crate) unk65: u8,
    pub(crate) unk66: i32,
    pub(crate) sp_eff9600: i16,
    pub(crate) pad1: [u8; 38],
}

impl EquipParamWeapon { 
    #[allow(unused)]
    pub(crate) fn set_right_hand_equipable(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn right_hand_equipable(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_left_hand_equipable(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn left_hand_equipable(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_both_hand_equipable(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn both_hand_equipable(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_arrow_slot_equipable(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn arrow_slot_equipable(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bolt_slot_equipable(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bolt_slot_equipable(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_guard(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_guard(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_parry(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_parry(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_magic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_magic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_pyromancy(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_pyromancy(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_miracle(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_miracle(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_vow_magic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_vow_magic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_normal_attack_type(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_normal_attack_type(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_blow_attack_type(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_blow_attack_type(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_slash_attack_type(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_slash_attack_type(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_thrust_attack_type(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_thrust_attack_type(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enhance(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enhance(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_luck_correct(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_luck_correct(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_custom(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_custom(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_base_change_reset(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_base_change_reset(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_repair(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_repair(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_dark_hand(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_dark_hand(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_simple_model_for_dlc(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn simple_model_for_dlc(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_ubyte_lantern_wep(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn ubyte_lantern_wep(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_versus_ghost_wep(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_versus_ghost_wep(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_base_change_category(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn base_change_category(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_dragon_slayer(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_dragon_slayer(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_deposit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_deposit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_multi_drop_share(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_multi_drop_share(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_discard(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_discard(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_drop(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_drop(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bool3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bool3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bool4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bool4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bool5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bool5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bool6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bool6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_bool7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn bool7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_auto_equip(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_auto_equip(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk16(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk16(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk17(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk17(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk18(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk18(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk19(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk19(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk20(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk20(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_wep_sp_mask0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn wep_sp_mask0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_wep_sp_mask1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn wep_sp_mask1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_wep_sp_mask2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn wep_sp_mask2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_wep_sp_mask3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn wep_sp_mask3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_wep_sp_mask4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn wep_sp_mask4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct FaceGenParam {
    pub(crate) face_geo_data01: u8,
    pub(crate) face_geo_data02: u8,
    pub(crate) face_geo_data03: u8,
    pub(crate) face_geo_data04: u8,
    pub(crate) face_geo_data05: u8,
    pub(crate) face_geo_data06: u8,
    pub(crate) face_geo_data07: u8,
    pub(crate) face_geo_data08: u8,
    pub(crate) face_geo_data09: u8,
    pub(crate) face_geo_data10: u8,
    pub(crate) face_geo_data11: u8,
    pub(crate) face_geo_data12: u8,
    pub(crate) face_geo_data13: u8,
    pub(crate) face_geo_data14: u8,
    pub(crate) face_geo_data15: u8,
    pub(crate) face_geo_data16: u8,
    pub(crate) face_geo_data17: u8,
    pub(crate) face_geo_data18: u8,
    pub(crate) face_geo_data19: u8,
    pub(crate) face_geo_data20: u8,
    pub(crate) face_geo_data21: u8,
    pub(crate) face_geo_data22: u8,
    pub(crate) face_geo_data23: u8,
    pub(crate) face_geo_data24: u8,
    pub(crate) face_geo_data25: u8,
    pub(crate) face_geo_data26: u8,
    pub(crate) face_geo_data27: u8,
    pub(crate) face_geo_data28: u8,
    pub(crate) face_geo_data29: u8,
    pub(crate) face_geo_data30: u8,
    pub(crate) face_geo_data31: u8,
    pub(crate) face_geo_data32: u8,
    pub(crate) face_geo_data33: u8,
    pub(crate) face_geo_data34: u8,
    pub(crate) face_geo_data35: u8,
    pub(crate) face_geo_data36: u8,
    pub(crate) face_geo_data37: u8,
    pub(crate) face_geo_data38: u8,
    pub(crate) face_geo_data39: u8,
    pub(crate) face_geo_data40: u8,
    pub(crate) face_geo_data41: u8,
    pub(crate) face_geo_data42: u8,
    pub(crate) face_geo_data43: u8,
    pub(crate) face_geo_data44: u8,
    pub(crate) face_geo_data45: u8,
    pub(crate) face_geo_data46: u8,
    pub(crate) face_geo_data47: u8,
    pub(crate) face_geo_data48: u8,
    pub(crate) face_geo_data49: u8,
    pub(crate) face_tex_data00: u8,
    pub(crate) face_tex_data01: u8,
    pub(crate) face_tex_data02: u8,
    pub(crate) face_tex_data03: u8,
    pub(crate) face_tex_data04: u8,
    pub(crate) face_tex_data05: u8,
    pub(crate) face_tex_data06: u8,
    pub(crate) face_tex_data07: u8,
    pub(crate) face_tex_data08: u8,
    pub(crate) face_tex_data09: u8,
    pub(crate) face_tex_data10: u8,
    pub(crate) face_tex_data11: u8,
    pub(crate) face_tex_data12: u8,
    pub(crate) face_tex_data13: u8,
    pub(crate) face_tex_data14: u8,
    pub(crate) face_tex_data15: u8,
    pub(crate) face_tex_data16: u8,
    pub(crate) face_tex_data17: u8,
    pub(crate) face_tex_data18: u8,
    pub(crate) face_tex_data19: u8,
    pub(crate) face_tex_data20: u8,
    pub(crate) face_tex_data21: u8,
    pub(crate) face_tex_data22: u8,
    pub(crate) face_tex_data23: u8,
    pub(crate) face_tex_data24: u8,
    pub(crate) face_tex_data25: u8,
    pub(crate) face_tex_data26: u8,
    pub(crate) face_tex_data27: u8,
    pub(crate) face_tex_data28: u8,
    pub(crate) face_tex_data29: u8,
    pub(crate) face_tex_data30: u8,
    pub(crate) face_tex_data31: u8,
    pub(crate) face_tex_data32: u8,
    pub(crate) face_tex_data33: u8,
    pub(crate) face_tex_data34: u8,
    pub(crate) face_tex_data35: u8,
    pub(crate) face_tex_data36: u8,
    pub(crate) face_tex_data37: u8,
    pub(crate) face_tex_data38: u8,
    pub(crate) face_tex_data39: u8,
    pub(crate) face_tex_data40: u8,
    pub(crate) face_tex_data41: u8,
    pub(crate) face_tex_data42: u8,
    pub(crate) face_tex_data43: u8,
    pub(crate) face_tex_data44: u8,
    pub(crate) face_tex_data45: u8,
    pub(crate) face_tex_data46: u8,
    pub(crate) face_tex_data47: u8,
    pub(crate) face_tex_data48: u8,
    pub(crate) face_tex_data49: u8,
    pub(crate) face_parts_id: u8,
    pub(crate) skin_color_r: u8,
    pub(crate) skin_color_g: u8,
    pub(crate) skin_color_b: u8,
    pub(crate) hair_parts_id: u8,
    pub(crate) hair_color_r: u8,
    pub(crate) hair_color_g: u8,
    pub(crate) hair_color_b: u8,
    pub(crate) hair_color_g_0: u8,
    pub(crate) eye_lparts_id: u8,
    pub(crate) eye_lcolor_r: u8,
    pub(crate) eye_lcolor_g: u8,
    pub(crate) eye_lcolor_b: u8,
    pub(crate) eye_rparts_id: u8,
    pub(crate) eye_rcolor_r: u8,
    pub(crate) eye_rcolor_g: u8,
    pub(crate) eye_rcolor_b: u8,
    pub(crate) eye_brow_parts_id: u8,
    pub(crate) eye_brow_color_r: u8,
    pub(crate) eye_brow_color_g: u8,
    pub(crate) eye_brow_color_b: u8,
    pub(crate) beard_parts_id: u8,
    pub(crate) beard_color_r: u8,
    pub(crate) beard_color_g: u8,
    pub(crate) beard_color_b: u8,
    pub(crate) accessories_parts_id: u8,
    pub(crate) accessories_color_r: u8,
    pub(crate) accessories_color_g: u8,
    pub(crate) accessories_color_b: u8,
    pub(crate) decal_parts_id: u8,
    pub(crate) decal_color_r: u8,
    pub(crate) decal_color_g: u8,
    pub(crate) decal_color_b: u8,
    pub(crate) decal_pos_x: u8,
    pub(crate) decal_pos_y: u8,
    pub(crate) decal_angle: u8,
    pub(crate) decal_scale: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct FaceParam {
    pub(crate) face_parts_id: u8,
    pub(crate) skin_color_r: u8,
    pub(crate) skin_color_g: u8,
    pub(crate) skin_color_b: u8,
    pub(crate) hair_parts_id: u8,
    pub(crate) hair_color_r: u8,
    pub(crate) hair_color_g: u8,
    pub(crate) hair_color_b: u8,
    pub(crate) eye_lparts_id: u8,
    pub(crate) eye_lcolor_r: u8,
    pub(crate) eye_lcolor_g: u8,
    pub(crate) eye_lcolor_b: u8,
    pub(crate) eye_rparts_id: u8,
    pub(crate) eye_rcolor_r: u8,
    pub(crate) eye_rcolor_g: u8,
    pub(crate) eye_rcolor_b: u8,
    pub(crate) eye_brow_parts_id: u8,
    pub(crate) eye_brow_color_r: u8,
    pub(crate) eye_brow_color_g: u8,
    pub(crate) eye_brow_color_b: u8,
    pub(crate) beard_parts_id: u8,
    pub(crate) beard_color_r: u8,
    pub(crate) beard_color_g: u8,
    pub(crate) beard_color_b: u8,
    pub(crate) accessories_parts_id: u8,
    pub(crate) accessories_color_r: u8,
    pub(crate) accessories_color_g: u8,
    pub(crate) accessories_color_b: u8,
    pub(crate) decal_parts_id: u8,
    pub(crate) decal_color_r: u8,
    pub(crate) decal_color_g: u8,
    pub(crate) decal_color_b: u8,
    pub(crate) decal_pos_x: u8,
    pub(crate) decal_pos_y: u8,
    pub(crate) decal_angle: u8,
    pub(crate) decal_scale: u8,
    pub(crate) chr_body_scale_head: u8,
    pub(crate) chr_body_scale_breast: u8,
    pub(crate) chr_body_scale_abdomen: u8,
    pub(crate) chr_body_scale_arm: u8,
    pub(crate) chr_body_scale_leg: u8,
    pub(crate) age: u8,
    pub(crate) gender: u8,
    pub(crate) carricature_geometry: u8,
    pub(crate) carricature_texture: u8,
    pub(crate) face_geo_data00: u8,
    pub(crate) face_geo_data01: u8,
    pub(crate) face_geo_data02: u8,
    pub(crate) face_geo_data03: u8,
    pub(crate) face_geo_data04: u8,
    pub(crate) face_geo_data05: u8,
    pub(crate) face_geo_data06: u8,
    pub(crate) face_geo_data07: u8,
    pub(crate) face_geo_data08: u8,
    pub(crate) face_geo_data09: u8,
    pub(crate) face_geo_data10: u8,
    pub(crate) face_geo_data11: u8,
    pub(crate) face_geo_data12: u8,
    pub(crate) face_geo_data13: u8,
    pub(crate) face_geo_data14: u8,
    pub(crate) face_geo_data15: u8,
    pub(crate) face_geo_data16: u8,
    pub(crate) face_geo_data17: u8,
    pub(crate) face_geo_data18: u8,
    pub(crate) face_geo_data19: u8,
    pub(crate) face_geo_data20: u8,
    pub(crate) face_geo_data21: u8,
    pub(crate) face_geo_data22: u8,
    pub(crate) face_geo_data23: u8,
    pub(crate) face_geo_data24: u8,
    pub(crate) face_geo_data25: u8,
    pub(crate) face_geo_data26: u8,
    pub(crate) face_geo_data27: u8,
    pub(crate) face_geo_data28: u8,
    pub(crate) face_geo_data29: u8,
    pub(crate) face_geo_data30: u8,
    pub(crate) face_geo_data31: u8,
    pub(crate) face_geo_data32: u8,
    pub(crate) face_geo_data33: u8,
    pub(crate) face_geo_data34: u8,
    pub(crate) face_geo_data35: u8,
    pub(crate) face_geo_data36: u8,
    pub(crate) face_geo_data37: u8,
    pub(crate) face_geo_data38: u8,
    pub(crate) face_geo_data39: u8,
    pub(crate) face_geo_data40: u8,
    pub(crate) face_geo_data41: u8,
    pub(crate) face_geo_data42: u8,
    pub(crate) face_geo_data43: u8,
    pub(crate) face_geo_data44: u8,
    pub(crate) face_geo_data45: u8,
    pub(crate) face_geo_data46: u8,
    pub(crate) face_geo_data47: u8,
    pub(crate) face_geo_data48: u8,
    pub(crate) face_geo_data49: u8,
    pub(crate) face_geo_data50: u8,
    pub(crate) face_geo_data51: u8,
    pub(crate) face_geo_data52: u8,
    pub(crate) face_geo_data53: u8,
    pub(crate) face_geo_data54: u8,
    pub(crate) face_geo_data55: u8,
    pub(crate) face_geo_data56: u8,
    pub(crate) face_geo_data57: u8,
    pub(crate) face_geo_data58: u8,
    pub(crate) face_geo_data59: u8,
    pub(crate) face_geo_data60: u8,
    pub(crate) face_tex_data00: u8,
    pub(crate) face_tex_data01: u8,
    pub(crate) face_tex_data02: u8,
    pub(crate) face_tex_data03: u8,
    pub(crate) face_tex_data04: u8,
    pub(crate) face_tex_data05: u8,
    pub(crate) face_tex_data06: u8,
    pub(crate) face_tex_data07: u8,
    pub(crate) face_tex_data08: u8,
    pub(crate) face_tex_data09: u8,
    pub(crate) face_tex_data10: u8,
    pub(crate) face_tex_data11: u8,
    pub(crate) face_tex_data12: u8,
    pub(crate) face_tex_data13: u8,
    pub(crate) face_tex_data14: u8,
    pub(crate) face_tex_data15: u8,
    pub(crate) face_tex_data16: u8,
    pub(crate) face_tex_data17: u8,
    pub(crate) face_tex_data18: u8,
    pub(crate) face_tex_data19: u8,
    pub(crate) face_tex_data20: u8,
    pub(crate) face_tex_data21: u8,
    pub(crate) face_tex_data22: u8,
    pub(crate) face_tex_data23: u8,
    pub(crate) face_tex_data24: u8,
    pub(crate) face_tex_data25: u8,
    pub(crate) face_tex_data26: u8,
    pub(crate) face_tex_data27: u8,
    pub(crate) face_tex_data28: u8,
    pub(crate) face_tex_data29: u8,
    pub(crate) face_tex_data30: u8,
    pub(crate) face_tex_data31: u8,
    pub(crate) face_tex_data32: u8,
    pub(crate) face_tex_data33: u8,
    pub(crate) face_tex_data34: u8,
    pub(crate) face_tex_data35: u8,
    pub(crate) face_geo_asym_data00: u8,
    pub(crate) face_geo_asym_data01: u8,
    pub(crate) face_geo_asym_data02: u8,
    pub(crate) face_geo_asym_data03: u8,
    pub(crate) face_geo_asym_data04: u8,
    pub(crate) face_geo_asym_data05: u8,
    pub(crate) face_geo_asym_data06: u8,
    pub(crate) face_geo_asym_data07: u8,
    pub(crate) face_geo_asym_data08: u8,
    pub(crate) face_geo_asym_data09: u8,
    pub(crate) face_geo_asym_data10: u8,
    pub(crate) face_geo_asym_data11: u8,
    pub(crate) face_geo_asym_data12: u8,
    pub(crate) face_geo_asym_data13: u8,
    pub(crate) face_geo_asym_data14: u8,
    pub(crate) face_geo_asym_data15: u8,
    pub(crate) face_geo_asym_data16: u8,
    pub(crate) face_geo_asym_data17: u8,
    pub(crate) face_geo_asym_data18: u8,
    pub(crate) face_geo_asym_data19: u8,
    pub(crate) face_geo_asym_data20: u8,
    pub(crate) face_geo_asym_data21: u8,
    pub(crate) face_geo_asym_data22: u8,
    pub(crate) face_geo_asym_data23: u8,
    pub(crate) face_geo_asym_data24: u8,
    pub(crate) face_geo_asym_data25: u8,
    pub(crate) face_geo_asym_data26: u8,
    pub(crate) face_geo_asym_data27: u8,
    pub(crate) face_geo_asym_data28: u8,
    pub(crate) face_geo_asym_data29: u8,
    pub(crate) face_geo_asym_data30: u8,
    pub(crate) face_geo_asym_data31: u8,
    pub(crate) padding: [u8; 18],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct FaceRangeParam {
    pub(crate) unknown: [u8; 196],
    pub(crate) face_geo_data00: f32,
    pub(crate) face_geo_data01: f32,
    pub(crate) face_geo_data02: f32,
    pub(crate) face_geo_data03: f32,
    pub(crate) face_geo_data04: f32,
    pub(crate) face_geo_data05: f32,
    pub(crate) face_geo_data06: f32,
    pub(crate) face_geo_data07: f32,
    pub(crate) face_geo_data08: f32,
    pub(crate) face_geo_data09: f32,
    pub(crate) face_geo_data10: f32,
    pub(crate) face_geo_data11: f32,
    pub(crate) face_geo_data12: f32,
    pub(crate) face_geo_data13: f32,
    pub(crate) face_geo_data14: f32,
    pub(crate) face_geo_data15: f32,
    pub(crate) face_geo_data16: f32,
    pub(crate) face_geo_data17: f32,
    pub(crate) face_geo_data18: f32,
    pub(crate) face_geo_data19: f32,
    pub(crate) face_geo_data20: f32,
    pub(crate) face_geo_data21: f32,
    pub(crate) face_geo_data22: f32,
    pub(crate) face_geo_data23: f32,
    pub(crate) face_geo_data24: f32,
    pub(crate) face_geo_data25: f32,
    pub(crate) face_geo_data26: f32,
    pub(crate) face_geo_data27: f32,
    pub(crate) face_geo_data28: f32,
    pub(crate) face_geo_data29: f32,
    pub(crate) face_geo_data30: f32,
    pub(crate) face_geo_data31: f32,
    pub(crate) face_geo_data32: f32,
    pub(crate) face_geo_data33: f32,
    pub(crate) face_geo_data34: f32,
    pub(crate) face_geo_data35: f32,
    pub(crate) face_geo_data36: f32,
    pub(crate) face_geo_data37: f32,
    pub(crate) face_geo_data38: f32,
    pub(crate) face_geo_data39: f32,
    pub(crate) face_geo_data40: f32,
    pub(crate) face_geo_data41: f32,
    pub(crate) face_geo_data42: f32,
    pub(crate) face_geo_data43: f32,
    pub(crate) face_geo_data44: f32,
    pub(crate) face_geo_data45: f32,
    pub(crate) face_geo_data46: f32,
    pub(crate) face_geo_data47: f32,
    pub(crate) face_geo_data48: f32,
    pub(crate) face_geo_data49: f32,
    pub(crate) face_geo_data50: f32,
    pub(crate) face_geo_data51: f32,
    pub(crate) face_geo_data52: f32,
    pub(crate) face_geo_data53: f32,
    pub(crate) face_geo_data54: f32,
    pub(crate) face_geo_data55: f32,
    pub(crate) face_geo_data56: f32,
    pub(crate) face_geo_data57: f32,
    pub(crate) face_geo_data58: f32,
    pub(crate) face_geo_data59: f32,
    pub(crate) face_geo_data60: f32,
    pub(crate) face_tex_data00: f32,
    pub(crate) face_tex_data01: f32,
    pub(crate) face_tex_data02: f32,
    pub(crate) face_tex_data03: f32,
    pub(crate) face_tex_data04: f32,
    pub(crate) face_tex_data05: f32,
    pub(crate) face_tex_data06: f32,
    pub(crate) face_tex_data07: f32,
    pub(crate) face_tex_data08: f32,
    pub(crate) face_tex_data09: f32,
    pub(crate) face_tex_data10: f32,
    pub(crate) face_tex_data11: f32,
    pub(crate) face_tex_data12: f32,
    pub(crate) face_tex_data13: f32,
    pub(crate) face_tex_data14: f32,
    pub(crate) face_tex_data15: f32,
    pub(crate) face_tex_data16: f32,
    pub(crate) face_tex_data17: f32,
    pub(crate) face_tex_data18: f32,
    pub(crate) face_tex_data19: f32,
    pub(crate) face_tex_data20: f32,
    pub(crate) face_tex_data21: f32,
    pub(crate) face_tex_data22: f32,
    pub(crate) face_tex_data23: f32,
    pub(crate) face_tex_data24: f32,
    pub(crate) face_tex_data25: f32,
    pub(crate) face_tex_data26: f32,
    pub(crate) face_tex_data27: f32,
    pub(crate) face_tex_data28: f32,
    pub(crate) face_tex_data29: f32,
    pub(crate) face_tex_data30: f32,
    pub(crate) face_tex_data31: f32,
    pub(crate) face_tex_data32: f32,
    pub(crate) face_tex_data33: f32,
    pub(crate) face_tex_data34: f32,
    pub(crate) face_tex_data35: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct FootSfxParam {
    pub(crate) foot_sfx_id000: i32,
    pub(crate) foot_sfx_id001: i32,
    pub(crate) foot_sfx_id002: i32,
    pub(crate) foot_sfx_id003: i32,
    pub(crate) foot_sfx_id004: i32,
    pub(crate) foot_sfx_id005: i32,
    pub(crate) foot_sfx_id006: i32,
    pub(crate) foot_sfx_id007: i32,
    pub(crate) foot_sfx_id008: i32,
    pub(crate) foot_sfx_id009: i32,
    pub(crate) foot_sfx_id010: i32,
    pub(crate) foot_sfx_id011: i32,
    pub(crate) foot_sfx_id012: i32,
    pub(crate) foot_sfx_id013: i32,
    pub(crate) foot_sfx_id014: i32,
    pub(crate) foot_sfx_id015: i32,
    pub(crate) foot_sfx_id016: i32,
    pub(crate) foot_sfx_id017: i32,
    pub(crate) foot_sfx_id018: i32,
    pub(crate) foot_sfx_id019: i32,
    pub(crate) foot_sfx_id020: i32,
    pub(crate) foot_sfx_id021: i32,
    pub(crate) foot_sfx_id022: i32,
    pub(crate) foot_sfx_id023: i32,
    pub(crate) foot_sfx_id024: i32,
    pub(crate) foot_sfx_id025: i32,
    pub(crate) foot_sfx_id026: i32,
    pub(crate) foot_sfx_id027: i32,
    pub(crate) foot_sfx_id028: i32,
    pub(crate) foot_sfx_id029: i32,
    pub(crate) foot_sfx_id030: i32,
    pub(crate) foot_sfx_id031: i32,
    pub(crate) foot_sfx_id032: i32,
    pub(crate) foot_sfx_id033: i32,
    pub(crate) foot_sfx_id034: i32,
    pub(crate) foot_sfx_id035: i32,
    pub(crate) foot_sfx_id036: i32,
    pub(crate) foot_sfx_id037: i32,
    pub(crate) foot_sfx_id038: i32,
    pub(crate) foot_sfx_id039: i32,
    pub(crate) foot_sfx_id040: i32,
    pub(crate) foot_sfx_id041: i32,
    pub(crate) foot_sfx_id042: i32,
    pub(crate) foot_sfx_id043: i32,
    pub(crate) foot_sfx_id044: i32,
    pub(crate) foot_sfx_id045: i32,
    pub(crate) foot_sfx_id046: i32,
    pub(crate) foot_sfx_id047: i32,
    pub(crate) foot_sfx_id048: i32,
    pub(crate) foot_sfx_id049: i32,
    pub(crate) foot_sfx_id050: i32,
    pub(crate) foot_sfx_id051: i32,
    pub(crate) foot_sfx_id052: i32,
    pub(crate) foot_sfx_id053: i32,
    pub(crate) foot_sfx_id054: i32,
    pub(crate) foot_sfx_id055: i32,
    pub(crate) foot_sfx_id056: i32,
    pub(crate) foot_sfx_id057: i32,
    pub(crate) foot_sfx_id058: i32,
    pub(crate) foot_sfx_id059: i32,
    pub(crate) foot_sfx_id060: i32,
    pub(crate) foot_sfx_id061: i32,
    pub(crate) foot_sfx_id062: i32,
    pub(crate) foot_sfx_id063: i32,
    pub(crate) foot_sfx_id064: i32,
    pub(crate) foot_sfx_id065: i32,
    pub(crate) foot_sfx_id066: i32,
    pub(crate) foot_sfx_id067: i32,
    pub(crate) foot_sfx_id068: i32,
    pub(crate) foot_sfx_id069: i32,
    pub(crate) foot_sfx_id070: i32,
    pub(crate) foot_sfx_id071: i32,
    pub(crate) foot_sfx_id072: i32,
    pub(crate) foot_sfx_id073: i32,
    pub(crate) foot_sfx_id074: i32,
    pub(crate) foot_sfx_id075: i32,
    pub(crate) foot_sfx_id076: i32,
    pub(crate) foot_sfx_id077: i32,
    pub(crate) foot_sfx_id078: i32,
    pub(crate) foot_sfx_id079: i32,
    pub(crate) foot_sfx_id080: i32,
    pub(crate) foot_sfx_id081: i32,
    pub(crate) foot_sfx_id082: i32,
    pub(crate) foot_sfx_id083: i32,
    pub(crate) foot_sfx_id084: i32,
    pub(crate) foot_sfx_id085: i32,
    pub(crate) foot_sfx_id086: i32,
    pub(crate) foot_sfx_id087: i32,
    pub(crate) foot_sfx_id088: i32,
    pub(crate) foot_sfx_id089: i32,
    pub(crate) foot_sfx_id090: i32,
    pub(crate) foot_sfx_id091: i32,
    pub(crate) foot_sfx_id092: i32,
    pub(crate) foot_sfx_id093: i32,
    pub(crate) foot_sfx_id094: i32,
    pub(crate) foot_sfx_id095: i32,
    pub(crate) foot_sfx_id096: i32,
    pub(crate) foot_sfx_id097: i32,
    pub(crate) foot_sfx_id098: i32,
    pub(crate) foot_sfx_id099: i32,
    pub(crate) foot_sfx_id100: i32,
    pub(crate) foot_sfx_id101: i32,
    pub(crate) foot_sfx_id102: i32,
    pub(crate) foot_sfx_id103: i32,
    pub(crate) foot_sfx_id104: i32,
    pub(crate) foot_sfx_id105: i32,
    pub(crate) foot_sfx_id106: i32,
    pub(crate) foot_sfx_id107: i32,
    pub(crate) foot_sfx_id108: i32,
    pub(crate) foot_sfx_id109: i32,
    pub(crate) foot_sfx_id110: i32,
    pub(crate) foot_sfx_id111: i32,
    pub(crate) foot_sfx_id112: i32,
    pub(crate) foot_sfx_id113: i32,
    pub(crate) foot_sfx_id114: i32,
    pub(crate) foot_sfx_id115: i32,
    pub(crate) foot_sfx_id116: i32,
    pub(crate) foot_sfx_id117: i32,
    pub(crate) foot_sfx_id118: i32,
    pub(crate) foot_sfx_id119: i32,
    pub(crate) foot_sfx_id120: i32,
    pub(crate) foot_sfx_id121: i32,
    pub(crate) foot_sfx_id122: i32,
    pub(crate) foot_sfx_id123: i32,
    pub(crate) foot_sfx_id124: i32,
    pub(crate) foot_sfx_id125: i32,
    pub(crate) foot_sfx_id126: i32,
    pub(crate) foot_sfx_id127: i32,
    pub(crate) foot_sfx_id128: i32,
    pub(crate) foot_sfx_id129: i32,
    pub(crate) foot_sfx_id130: i32,
    pub(crate) foot_sfx_id131: i32,
    pub(crate) foot_sfx_id132: i32,
    pub(crate) foot_sfx_id133: i32,
    pub(crate) foot_sfx_id134: i32,
    pub(crate) foot_sfx_id135: i32,
    pub(crate) foot_sfx_id136: i32,
    pub(crate) foot_sfx_id137: i32,
    pub(crate) foot_sfx_id138: i32,
    pub(crate) foot_sfx_id139: i32,
    pub(crate) foot_sfx_id140: i32,
    pub(crate) foot_sfx_id141: i32,
    pub(crate) foot_sfx_id142: i32,
    pub(crate) foot_sfx_id143: i32,
    pub(crate) foot_sfx_id144: i32,
    pub(crate) foot_sfx_id145: i32,
    pub(crate) foot_sfx_id146: i32,
    pub(crate) foot_sfx_id147: i32,
    pub(crate) foot_sfx_id148: i32,
    pub(crate) foot_sfx_id149: i32,
    pub(crate) foot_sfx_id150: i32,
    pub(crate) foot_sfx_id151: i32,
    pub(crate) foot_sfx_id152: i32,
    pub(crate) foot_sfx_id153: i32,
    pub(crate) foot_sfx_id154: i32,
    pub(crate) foot_sfx_id155: i32,
    pub(crate) foot_sfx_id156: i32,
    pub(crate) foot_sfx_id157: i32,
    pub(crate) foot_sfx_id158: i32,
    pub(crate) foot_sfx_id159: i32,
    pub(crate) foot_sfx_id160: i32,
    pub(crate) foot_sfx_id161: i32,
    pub(crate) foot_sfx_id162: i32,
    pub(crate) foot_sfx_id163: i32,
    pub(crate) foot_sfx_id164: i32,
    pub(crate) foot_sfx_id165: i32,
    pub(crate) foot_sfx_id166: i32,
    pub(crate) foot_sfx_id167: i32,
    pub(crate) foot_sfx_id168: i32,
    pub(crate) foot_sfx_id169: i32,
    pub(crate) foot_sfx_id170: i32,
    pub(crate) foot_sfx_id171: i32,
    pub(crate) foot_sfx_id172: i32,
    pub(crate) foot_sfx_id173: i32,
    pub(crate) foot_sfx_id174: i32,
    pub(crate) foot_sfx_id175: i32,
    pub(crate) foot_sfx_id176: i32,
    pub(crate) foot_sfx_id177: i32,
    pub(crate) foot_sfx_id178: i32,
    pub(crate) foot_sfx_id179: i32,
    pub(crate) foot_sfx_id180: i32,
    pub(crate) foot_sfx_id181: i32,
    pub(crate) foot_sfx_id182: i32,
    pub(crate) foot_sfx_id183: i32,
    pub(crate) foot_sfx_id184: i32,
    pub(crate) foot_sfx_id185: i32,
    pub(crate) foot_sfx_id186: i32,
    pub(crate) foot_sfx_id187: i32,
    pub(crate) foot_sfx_id188: i32,
    pub(crate) foot_sfx_id189: i32,
    pub(crate) foot_sfx_id190: i32,
    pub(crate) foot_sfx_id191: i32,
    pub(crate) foot_sfx_id192: i32,
    pub(crate) foot_sfx_id193: i32,
    pub(crate) foot_sfx_id194: i32,
    pub(crate) foot_sfx_id195: i32,
    pub(crate) foot_sfx_id196: i32,
    pub(crate) foot_sfx_id197: i32,
    pub(crate) foot_sfx_id198: i32,
    pub(crate) foot_sfx_id199: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GameAreaParam {
    pub(crate) bonus_soul_single: i32,
    pub(crate) bonus_soul_multi: i32,
    pub(crate) humanity_point_count_flag_id_top: i32,
    pub(crate) humanity_drop_point1: i16,
    pub(crate) humanity_drop_point2: i16,
    pub(crate) humanity_drop_point3: i16,
    pub(crate) humanity_drop_point4: i16,
    pub(crate) humanity_drop_point5: i16,
    pub(crate) humanity_drop_point6: i16,
    pub(crate) humanity_drop_point7: i16,
    pub(crate) humanity_drop_point8: i16,
    pub(crate) humanity_drop_point9: i16,
    pub(crate) humanity_drop_point10: i16,
    pub(crate) sub_bonus_soul_single: i32,
    pub(crate) subbonus_soul_multi: i32,
    pub(crate) pad1: [u8; 8],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GameProgressParam {
    pub(crate) event_flag_id: i32,
    pub(crate) progress_id: u8,
    pub(crate) pad1: [u8; 11],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GemCategoryParam {
    pub(crate) sort_no: i32,
    pub(crate) manifest_rate: f32,
    pub(crate) directional_id: i32,
    pub(crate) cate_group_id: i32,
    pub(crate) exclude_group_id: i32,
    pub(crate) bitfield0: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) affinity_cate_id_0: i32,
    pub(crate) affinity_modify_rate_0: f32,
    pub(crate) affinity_cate_id_1: i32,
    pub(crate) affinity_modify_rate_1: f32,
    pub(crate) affinity_cate_id_2: i32,
    pub(crate) affinity_modify_rate_2: f32,
    pub(crate) affinity_cate_id_3: i32,
    pub(crate) affinity_modify_rate_3: f32,
}

impl GemCategoryParam { 
    #[allow(unused)]
    pub(crate) fn set_is_negative(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_negative(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_a(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_a(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_b(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_b(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_c(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_c(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_d(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_d(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_e(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_e(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_slot_type_f(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_slot_type_f(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_holygrail_type_group(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn holygrail_type_group(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GemDropDopingParam {
    pub(crate) rank_min: i32,
    pub(crate) rank_max: i32,
    pub(crate) normal_distribution_ave: i32,
    pub(crate) normal_distribution_sigma: i32,
    pub(crate) slot_type_a: f32,
    pub(crate) slot_type_b: f32,
    pub(crate) slot_type_c: f32,
    pub(crate) slot_type_d: f32,
    pub(crate) slot_type_e: f32,
    pub(crate) slot_type_f: f32,
    pub(crate) directional_id_rate_0: f32,
    pub(crate) directional_id_rate_1: f32,
    pub(crate) directional_id_rate_2: f32,
    pub(crate) directional_id_rate_3: f32,
    pub(crate) directional_id_rate_4: f32,
    pub(crate) directional_id_rate_5: f32,
    pub(crate) directional_id_rate_6: f32,
    pub(crate) directional_id_rate_7: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GemDropModifyParam {
    pub(crate) slot_type_rate_a: f32,
    pub(crate) slot_type_rate_b: f32,
    pub(crate) slot_type_rate_c: f32,
    pub(crate) slot_type_rate_d: f32,
    pub(crate) slot_type_rate_e: f32,
    pub(crate) slot_type_rate_f: f32,
    pub(crate) directional_id_rate_0: f32,
    pub(crate) directional_id_rate_1: f32,
    pub(crate) directional_id_rate_2: f32,
    pub(crate) directional_id_rate_3: f32,
    pub(crate) directional_id_rate_4: f32,
    pub(crate) directional_id_rate_5: f32,
    pub(crate) directional_id_rate_6: f32,
    pub(crate) directional_id_rate_7: f32,
    pub(crate) affinity_cate_id_0: i32,
    pub(crate) affinity_modify_rate_0: f32,
    pub(crate) affinity_cate_id_1: i32,
    pub(crate) affinity_modify_rate_1: f32,
    pub(crate) affinity_cate_id_2: i32,
    pub(crate) affinity_modify_rate_2: f32,
    pub(crate) affinity_cate_id_3: i32,
    pub(crate) affinity_modify_rate_3: f32,
    pub(crate) manifest_rate_0: f32,
    pub(crate) manifest_rate_1: f32,
    pub(crate) manifest_rate_2: f32,
    pub(crate) manifest_rate_3: f32,
    pub(crate) manifest_rate_4: f32,
    pub(crate) manifest_rate_5: f32,
    pub(crate) negativize_rate_0: f32,
    pub(crate) normal_distribution_ave: i32,
    pub(crate) normal_distribution_sigma: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GemeffectParam {
    pub(crate) sp_effect_id: i32,
    pub(crate) category_id: i32,
    pub(crate) effect_rank: i32,
    pub(crate) rank_min: i32,
    pub(crate) rank_max: i32,
    pub(crate) disposal_price: i32,
    pub(crate) gem_icon_id_offset: i16,
    pub(crate) pad1: [u8; 2],
    pub(crate) sp_effect_id_for_atk: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct GemGenParam {
    pub(crate) pad: [u8; 3],
    pub(crate) field0x04: i32,
    pub(crate) gem_name_id_offset: i32,
    pub(crate) disable_slot_rate_modify: i32,
    pub(crate) slot_type_rate_a: f32,
    pub(crate) slot_type_rate_b: f32,
    pub(crate) slot_type_rate_c: f32,
    pub(crate) slot_type_rate_d: f32,
    pub(crate) slot_type_rate_e: f32,
    pub(crate) slot_type_rate_f: f32,
    pub(crate) gem_rank_doping: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) gemeffect_gen_param_type_0: i32,
    pub(crate) gemeffect_gen_param_0: i32,
    pub(crate) manifest_rate_0: f32,
    pub(crate) negativize_rate_0: f32,
    pub(crate) gemeffect_gen_param_type_1: i32,
    pub(crate) gemeffect_gen_param_1: i32,
    pub(crate) manifest_rate_1: f32,
    pub(crate) negativize_rate_1: f32,
    pub(crate) gemeffect_gen_param_type_2: i32,
    pub(crate) gemeffect_gen_param_2: i32,
    pub(crate) manifest_rate_2: f32,
    pub(crate) negativize_rate_2: f32,
    pub(crate) gemeffect_gen_param_type_3: i32,
    pub(crate) gemeffect_gen_param_3: i32,
    pub(crate) manifest_rate_3: f32,
    pub(crate) negativize_rate_3: f32,
    pub(crate) gemeffect_gen_param_type_4: i32,
    pub(crate) gemeffect_gen_param_4: i32,
    pub(crate) manifest_rate_4: f32,
    pub(crate) negativize_rate_4: f32,
    pub(crate) gemeffect_gen_param_type_5: i32,
    pub(crate) gemeffect_gen_param_5: i32,
    pub(crate) manifest_rate_5: f32,
    pub(crate) negativize_rate_5: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct HitEffectSeParam {
    pub(crate) h00_hit_effect_se_id0: i32,
    pub(crate) h00_hit_effect_se_id1: i32,
    pub(crate) h00_hit_effect_se_id2: i32,
    pub(crate) h00_hit_effect_se_id3: i32,
    pub(crate) h00_hit_effect_se_id4: i32,
    pub(crate) h00_hit_effect_se_id5: i32,
    pub(crate) h00_hit_effect_se_id6: i32,
    pub(crate) h00_hit_effect_se_id7: i32,
    pub(crate) h01_hit_effect_se_id0: i32,
    pub(crate) h01_hit_effect_se_id1: i32,
    pub(crate) h01_hit_effect_se_id2: i32,
    pub(crate) h01_hit_effect_se_id3: i32,
    pub(crate) h01_hit_effect_se_id4: i32,
    pub(crate) h01_hit_effect_se_id5: i32,
    pub(crate) h01_hit_effect_se_id6: i32,
    pub(crate) h01_hit_effect_se_id7: i32,
    pub(crate) h02_hit_effect_se_id0: i32,
    pub(crate) h02_hit_effect_se_id1: i32,
    pub(crate) h02_hit_effect_se_id2: i32,
    pub(crate) h02_hit_effect_se_id3: i32,
    pub(crate) h02_hit_effect_se_id4: i32,
    pub(crate) h02_hit_effect_se_id5: i32,
    pub(crate) h02_hit_effect_se_id6: i32,
    pub(crate) h02_hit_effect_se_id7: i32,
    pub(crate) h03_hit_effect_se_id0: i32,
    pub(crate) h03_hit_effect_se_id1: i32,
    pub(crate) h03_hit_effect_se_id2: i32,
    pub(crate) h03_hit_effect_se_id3: i32,
    pub(crate) h03_hit_effect_se_id4: i32,
    pub(crate) h03_hit_effect_se_id5: i32,
    pub(crate) h03_hit_effect_se_id6: i32,
    pub(crate) h03_hit_effect_se_id7: i32,
    pub(crate) h04_hit_effect_se_id0: i32,
    pub(crate) h04_hit_effect_se_id1: i32,
    pub(crate) h04_hit_effect_se_id2: i32,
    pub(crate) h04_hit_effect_se_id3: i32,
    pub(crate) h04_hit_effect_se_id4: i32,
    pub(crate) h04_hit_effect_se_id5: i32,
    pub(crate) h04_hit_effect_se_id6: i32,
    pub(crate) h04_hit_effect_se_id7: i32,
    pub(crate) h05_hit_effect_se_id0: i32,
    pub(crate) h05_hit_effect_se_id1: i32,
    pub(crate) h05_hit_effect_se_id2: i32,
    pub(crate) h05_hit_effect_se_id3: i32,
    pub(crate) h05_hit_effect_se_id4: i32,
    pub(crate) h05_hit_effect_se_id5: i32,
    pub(crate) h05_hit_effect_se_id6: i32,
    pub(crate) h05_hit_effect_se_id7: i32,
    pub(crate) h06_hit_effect_se_id0: i32,
    pub(crate) h06_hit_effect_se_id1: i32,
    pub(crate) h06_hit_effect_se_id2: i32,
    pub(crate) h06_hit_effect_se_id3: i32,
    pub(crate) h06_hit_effect_se_id4: i32,
    pub(crate) h06_hit_effect_se_id5: i32,
    pub(crate) h06_hit_effect_se_id6: i32,
    pub(crate) h06_hit_effect_se_id7: i32,
    pub(crate) h07_hit_effect_se_id0: i32,
    pub(crate) h07_hit_effect_se_id1: i32,
    pub(crate) h07_hit_effect_se_id2: i32,
    pub(crate) h07_hit_effect_se_id3: i32,
    pub(crate) h07_hit_effect_se_id4: i32,
    pub(crate) h07_hit_effect_se_id5: i32,
    pub(crate) h07_hit_effect_se_id6: i32,
    pub(crate) h07_hit_effect_se_id7: i32,
    pub(crate) h08_hit_effect_se_id0: i32,
    pub(crate) h08_hit_effect_se_id1: i32,
    pub(crate) h08_hit_effect_se_id2: i32,
    pub(crate) h08_hit_effect_se_id3: i32,
    pub(crate) h08_hit_effect_se_id4: i32,
    pub(crate) h08_hit_effect_se_id5: i32,
    pub(crate) h08_hit_effect_se_id6: i32,
    pub(crate) h08_hit_effect_se_id7: i32,
    pub(crate) h09_hit_effect_se_id0: i32,
    pub(crate) h09_hit_effect_se_id1: i32,
    pub(crate) h09_hit_effect_se_id2: i32,
    pub(crate) h09_hit_effect_se_id3: i32,
    pub(crate) h09_hit_effect_se_id4: i32,
    pub(crate) h09_hit_effect_se_id5: i32,
    pub(crate) h09_hit_effect_se_id6: i32,
    pub(crate) h09_hit_effect_se_id7: i32,
    pub(crate) h10_hit_effect_se_id0: i32,
    pub(crate) h10_hit_effect_se_id1: i32,
    pub(crate) h10_hit_effect_se_id2: i32,
    pub(crate) h10_hit_effect_se_id3: i32,
    pub(crate) h10_hit_effect_se_id4: i32,
    pub(crate) h10_hit_effect_se_id5: i32,
    pub(crate) h10_hit_effect_se_id6: i32,
    pub(crate) h10_hit_effect_se_id7: i32,
    pub(crate) h11_hit_effect_se_id0: i32,
    pub(crate) h11_hit_effect_se_id1: i32,
    pub(crate) h11_hit_effect_se_id2: i32,
    pub(crate) h11_hit_effect_se_id3: i32,
    pub(crate) h11_hit_effect_se_id4: i32,
    pub(crate) h11_hit_effect_se_id5: i32,
    pub(crate) h11_hit_effect_se_id6: i32,
    pub(crate) h11_hit_effect_se_id7: i32,
    pub(crate) h12_hit_effect_se_id0: i32,
    pub(crate) h12_hit_effect_se_id1: i32,
    pub(crate) h12_hit_effect_se_id2: i32,
    pub(crate) h12_hit_effect_se_id3: i32,
    pub(crate) h12_hit_effect_se_id4: i32,
    pub(crate) h12_hit_effect_se_id5: i32,
    pub(crate) h12_hit_effect_se_id6: i32,
    pub(crate) h12_hit_effect_se_id7: i32,
    pub(crate) h13_hit_effect_se_id0: i32,
    pub(crate) h13_hit_effect_se_id1: i32,
    pub(crate) h13_hit_effect_se_id2: i32,
    pub(crate) h13_hit_effect_se_id3: i32,
    pub(crate) h13_hit_effect_se_id4: i32,
    pub(crate) h13_hit_effect_se_id5: i32,
    pub(crate) h13_hit_effect_se_id6: i32,
    pub(crate) h13_hit_effect_se_id7: i32,
    pub(crate) h14_hit_effect_se_id0: i32,
    pub(crate) h14_hit_effect_se_id1: i32,
    pub(crate) h14_hit_effect_se_id2: i32,
    pub(crate) h14_hit_effect_se_id3: i32,
    pub(crate) h14_hit_effect_se_id4: i32,
    pub(crate) h14_hit_effect_se_id5: i32,
    pub(crate) h14_hit_effect_se_id6: i32,
    pub(crate) h14_hit_effect_se_id7: i32,
    pub(crate) h15_hit_effect_se_id0: i32,
    pub(crate) h15_hit_effect_se_id1: i32,
    pub(crate) h15_hit_effect_se_id2: i32,
    pub(crate) h15_hit_effect_se_id3: i32,
    pub(crate) h15_hit_effect_se_id4: i32,
    pub(crate) h15_hit_effect_se_id5: i32,
    pub(crate) h15_hit_effect_se_id6: i32,
    pub(crate) h15_hit_effect_se_id7: i32,
    pub(crate) h16_hit_effect_se_id0: i32,
    pub(crate) h16_hit_effect_se_id1: i32,
    pub(crate) h16_hit_effect_se_id2: i32,
    pub(crate) h16_hit_effect_se_id3: i32,
    pub(crate) h16_hit_effect_se_id4: i32,
    pub(crate) h16_hit_effect_se_id5: i32,
    pub(crate) h16_hit_effect_se_id6: i32,
    pub(crate) h16_hit_effect_se_id7: i32,
    pub(crate) h17_hit_effect_se_id0: i32,
    pub(crate) h17_hit_effect_se_id1: i32,
    pub(crate) h17_hit_effect_se_id2: i32,
    pub(crate) h17_hit_effect_se_id3: i32,
    pub(crate) h17_hit_effect_se_id4: i32,
    pub(crate) h17_hit_effect_se_id5: i32,
    pub(crate) h17_hit_effect_se_id6: i32,
    pub(crate) h17_hit_effect_se_id7: i32,
    pub(crate) h18_hit_effect_se_id0: i32,
    pub(crate) h18_hit_effect_se_id1: i32,
    pub(crate) h18_hit_effect_se_id2: i32,
    pub(crate) h18_hit_effect_se_id3: i32,
    pub(crate) h18_hit_effect_se_id4: i32,
    pub(crate) h18_hit_effect_se_id5: i32,
    pub(crate) h18_hit_effect_se_id6: i32,
    pub(crate) h18_hit_effect_se_id7: i32,
    pub(crate) h19_hit_effect_se_id0: i32,
    pub(crate) h19_hit_effect_se_id1: i32,
    pub(crate) h19_hit_effect_se_id2: i32,
    pub(crate) h19_hit_effect_se_id3: i32,
    pub(crate) h19_hit_effect_se_id4: i32,
    pub(crate) h19_hit_effect_se_id5: i32,
    pub(crate) h19_hit_effect_se_id6: i32,
    pub(crate) h19_hit_effect_se_id7: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct HitEffectSfxConceptParam {
    pub(crate) sfx_concept_id0: i16,
    pub(crate) sfx_concept_id1: i16,
    pub(crate) sfx_concept_id2: i16,
    pub(crate) sfx_concept_id3: i16,
    pub(crate) sfx_concept_id4: i16,
    pub(crate) sfx_concept_id5: i16,
    pub(crate) sfx_concept_id6: i16,
    pub(crate) sfx_concept_id7: i16,
    pub(crate) sfx_concept_id8: i16,
    pub(crate) sfx_concept_id9: i16,
    pub(crate) sfx_concept_id10: i16,
    pub(crate) sfx_concept_id11: i16,
    pub(crate) sfx_concept_id12: i16,
    pub(crate) sfx_concept_id13: i16,
    pub(crate) sfx_concept_id14: i16,
    pub(crate) sfx_concept_id15: i16,
    pub(crate) sfx_concept_id16: i16,
    pub(crate) sfx_concept_id17: i16,
    pub(crate) sfx_concept_id18: i16,
    pub(crate) sfx_concept_id19: i16,
    pub(crate) sfx_concept_id20: i16,
    pub(crate) sfx_concept_id21: i16,
    pub(crate) sfx_concept_id22: i16,
    pub(crate) sfx_concept_id23: i16,
    pub(crate) sfx_concept_id24: i16,
    pub(crate) sfx_concept_id25: i16,
    pub(crate) sfx_concept_id26: i16,
    pub(crate) sfx_concept_id27: i16,
    pub(crate) sfx_concept_id28: i16,
    pub(crate) sfx_concept_id29: i16,
    pub(crate) pad1: [u8; 20],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct HitEffectSfxParam {
    pub(crate) hit_sfx_id0: i32,
    pub(crate) hit_sfx_id1: i32,
    pub(crate) hit_sfx_id2: i32,
    pub(crate) hit_sfx_id3: i32,
    pub(crate) hit_sfx_id4: i32,
    pub(crate) hit_sfx_id5: i32,
    pub(crate) hit_sfx_id6: i32,
    pub(crate) hit_sfx_id7: i32,
    pub(crate) hit_sfx_id8: i32,
    pub(crate) hit_sfx_id9: i32,
    pub(crate) hit_sfx_id10: i32,
    pub(crate) hit_sfx_id11: i32,
    pub(crate) hit_sfx_id12: i32,
    pub(crate) hit_sfx_id13: i32,
    pub(crate) hit_sfx_id14: i32,
    pub(crate) hit_sfx_id15: i32,
    pub(crate) pad1: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct HitMtrlParam {
    pub(crate) ai_volume_rate: f32,
    pub(crate) sp_effect_id0: i32,
    pub(crate) sp_effect_id1: i32,
    pub(crate) bitfield0: u8,
    pub(crate) hit_mtrl_type0: u8,
    pub(crate) hit_mtrl_type1: u8,
    pub(crate) hit_mtrl_type2: u8,
    pub(crate) sp_effect_id2: i32,
    pub(crate) sp_effect_id3: i32,
    pub(crate) sp_effect_id4: i32,
    pub(crate) sp_effect_id5: i32,
    pub(crate) sp_effect_id6: i32,
    pub(crate) sp_effect_id7: i32,
    pub(crate) sp_effect_id8: i32,
    pub(crate) sp_effect_id9: i32,
    pub(crate) sp_effect_id10: i32,
    pub(crate) sp_effect_id11: i32,
    pub(crate) sp_effect_id12: i32,
    pub(crate) sp_effect_id13: i32,
    pub(crate) sp_effect_id14: i32,
    pub(crate) sp_effect_id15: i32,
    pub(crate) sp_effect_id16: i32,
    pub(crate) pad1: [u8; 20],
}

impl HitMtrlParam { 
    #[allow(unused)]
    pub(crate) fn set_foot_effect_height_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn foot_effect_height_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_foot_effect_height_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn foot_effect_height_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_foot_effect_dir_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn foot_effect_dir_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_foot_effect_dir_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn foot_effect_dir_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_new_sp_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn new_sp_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_new_sp_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn new_sp_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct HPEstusFlaskRecoveryParam {
    pub(crate) recovery_count0: u8,
    pub(crate) recovery_count1: u8,
    pub(crate) recovery_count2: u8,
    pub(crate) recovery_count3: u8,
    pub(crate) recovery_count4: u8,
    pub(crate) recovery_count5: u8,
    pub(crate) recovery_count6: u8,
    pub(crate) recovery_count7: u8,
    pub(crate) recovery_count8: u8,
    pub(crate) recovery_count9: u8,
    pub(crate) recovery_count10: u8,
    pub(crate) recovery_count11: u8,
    pub(crate) recovery_count12: u8,
    pub(crate) recovery_count13: u8,
    pub(crate) recovery_count14: u8,
    pub(crate) recovery_count15: u8,
    pub(crate) recovery_count16: u8,
    pub(crate) recovery_count17: u8,
    pub(crate) recovery_count18: u8,
    pub(crate) recovery_count19: u8,
    pub(crate) recovery_count20: u8,
    pub(crate) recovery_count21: u8,
    pub(crate) recovery_count22: u8,
    pub(crate) recovery_count23: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ItemLotParam {
    pub(crate) item_lot_id1: i32,
    pub(crate) item_lot_id2: i32,
    pub(crate) item_lot_id3: i32,
    pub(crate) item_lot_id4: i32,
    pub(crate) item_lot_id5: i32,
    pub(crate) item_lot_id6: i32,
    pub(crate) item_lot_id7: i32,
    pub(crate) item_lot_id8: i32,
    pub(crate) lot_item_category01: u32,
    pub(crate) lot_item_category02: u32,
    pub(crate) lot_item_category03: u32,
    pub(crate) lot_item_category04: u32,
    pub(crate) lot_item_category05: u32,
    pub(crate) lot_item_category06: u32,
    pub(crate) lot_item_category07: u32,
    pub(crate) lot_item_category08: u32,
    pub(crate) lot_item_base_point01: i16,
    pub(crate) lot_item_base_point02: i16,
    pub(crate) lot_item_base_point03: i16,
    pub(crate) lot_item_base_point04: i16,
    pub(crate) lot_item_base_point05: i16,
    pub(crate) lot_item_base_point06: i16,
    pub(crate) lot_item_base_point07: i16,
    pub(crate) lot_item_base_point08: i16,
    pub(crate) cumulate_lot_point01: i16,
    pub(crate) cumulate_lot_point02: i16,
    pub(crate) cumulate_lot_point03: i16,
    pub(crate) cumulate_lot_point04: i16,
    pub(crate) cumulate_lot_point05: i16,
    pub(crate) cumulate_lot_point06: i16,
    pub(crate) cumulate_lot_point07: i16,
    pub(crate) cumulate_lot_point08: i16,
    pub(crate) get_item_flag_id01: i32,
    pub(crate) get_item_flag_id02: i32,
    pub(crate) get_item_flag_id03: i32,
    pub(crate) get_item_flag_id04: i32,
    pub(crate) get_item_flag_id05: i32,
    pub(crate) get_item_flag_id06: i32,
    pub(crate) get_item_flag_id07: i32,
    pub(crate) get_item_flag_id08: i32,
    pub(crate) get_item_flag_id: i32,
    pub(crate) cumulate_num_flag_id: i32,
    pub(crate) cumulate_num_max: u8,
    pub(crate) lot_item_rarity: u8,
    pub(crate) lot_item_num1: u8,
    pub(crate) lot_item_num2: u8,
    pub(crate) lot_item_num3: u8,
    pub(crate) lot_item_num4: u8,
    pub(crate) lot_item_num5: u8,
    pub(crate) lot_item_num6: u8,
    pub(crate) lot_item_num7: u8,
    pub(crate) lot_item_num8: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) clear_count: i8,
    pub(crate) pad1: [u8; 3],
}

impl ItemLotParam { 
    #[allow(unused)]
    pub(crate) fn set_enable_luck01(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck01(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck02(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck02(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck03(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck03(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck04(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck04(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck05(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck05(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck06(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck06(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck07(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck07(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_luck08(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_luck08(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset01(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset01(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset02(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset02(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset03(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset03(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset04(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset04(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset05(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset05(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset06(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset06(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset07(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset07(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cumulate_reset08(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cumulate_reset08(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct KnockBackParam {
    pub(crate) damage_min_cont_time: f32,
    pub(crate) damage_s_cont_time: f32,
    pub(crate) damage_m_cont_time: f32,
    pub(crate) damage_l_cont_time: f32,
    pub(crate) damage_blow_s_cont_time: f32,
    pub(crate) damage_blow_m_cont_time: f32,
    pub(crate) damage_strike_cont_time: f32,
    pub(crate) damage_uppercut_cont_time: f32,
    pub(crate) damage_push_cont_time: f32,
    pub(crate) damage_breath_cont_time: f32,
    pub(crate) damage_head_shot_cont_time: f32,
    pub(crate) guard_s_cont_time: f32,
    pub(crate) guard_l_cont_time: f32,
    pub(crate) guard_ll_cont_time: f32,
    pub(crate) guard_brake_cont_time: f32,
    pub(crate) damage_min_dec_time: f32,
    pub(crate) damage_s_dec_time: f32,
    pub(crate) damage_m_dec_time: f32,
    pub(crate) damage_l_dec_time: f32,
    pub(crate) damage_blow_s_dec_time: f32,
    pub(crate) damage_blow_m_dec_time: f32,
    pub(crate) damage_strike_dec_time: f32,
    pub(crate) damage_uppercut_dec_time: f32,
    pub(crate) damage_push_dec_time: f32,
    pub(crate) damage_breath_dec_time: f32,
    pub(crate) damage_head_shot_dec_time: f32,
    pub(crate) guard_s_dec_time: f32,
    pub(crate) guard_l_dec_time: f32,
    pub(crate) guard_ll_dec_time: f32,
    pub(crate) guard_brake_dec_time: f32,
    pub(crate) pad1: [u8; 8],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct KnowledgeLoadScreenItemParam {
    pub(crate) loadscreen_category_id: u32,
    pub(crate) knowledge_id: i32,
    pub(crate) pad1: [u8; 8],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LoadBalancerDrawDistScaleParam {
    pub(crate) lod_dist_draw_scale0: f32,
    pub(crate) lod_dist_draw_scale1: f32,
    pub(crate) lod_dist_draw_scale2: f32,
    pub(crate) lod_dist_draw_scale3: f32,
    pub(crate) lod_dist_draw_scale4: f32,
    pub(crate) lod_dist_draw_scale5: f32,
    pub(crate) lod_dist_draw_scale6: f32,
    pub(crate) lod_dist_draw_scale7: f32,
    pub(crate) lod_dist_draw_scale8: f32,
    pub(crate) lod_dist_draw_scale9: f32,
    pub(crate) lod_dist_draw_scale10: f32,
    pub(crate) lod_dist_draw_scale11: f32,
    pub(crate) lod_dist_draw_scale12: f32,
    pub(crate) lod_dist_draw_scale13: f32,
    pub(crate) lod_dist_draw_scale14: f32,
    pub(crate) lod_dist_draw_scale15: f32,
    pub(crate) lod_dist_draw_scale16: f32,
    pub(crate) lod_dist_draw_scale17: f32,
    pub(crate) lod_dist_draw_scale18: f32,
    pub(crate) lod_dist_draw_scale19: f32,
    pub(crate) lod_dist_draw_scale20: f32,
    pub(crate) pad1: [u8; 44],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LoadBalancerParam {
    pub(crate) unk1: f32,
    pub(crate) unk2: f32,
    pub(crate) unk3: i32,
    pub(crate) unk4: i32,
    pub(crate) unk5: i32,
    pub(crate) unk6: i32,
    pub(crate) load_balancer_val0: u8,
    pub(crate) load_balancer_val1: u8,
    pub(crate) load_balancer_val2: u8,
    pub(crate) load_balancer_val3: u8,
    pub(crate) load_balancer_val4: u8,
    pub(crate) load_balancer_val5: u8,
    pub(crate) load_balancer_val6: u8,
    pub(crate) load_balancer_val7: u8,
    pub(crate) load_balancer_val8: u8,
    pub(crate) load_balancer_val9: u8,
    pub(crate) load_balancer_val10: u8,
    pub(crate) load_balancer_val11: u8,
    pub(crate) load_balancer_val12: u8,
    pub(crate) load_balancer_val13: u8,
    pub(crate) load_balancer_val14: u8,
    pub(crate) load_balancer_val15: u8,
    pub(crate) load_balancer_val16: u8,
    pub(crate) load_balancer_val17: u8,
    pub(crate) pad1: [u8; 38],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LockCamParam {
    pub(crate) cam_dist_target: f32,
    pub(crate) rot_range_min_x: f32,
    pub(crate) lock_rot_xshift_ratio: f32,
    pub(crate) chr_org_offset_z: f32,
    pub(crate) chr_lock_range_max_radius: f32,
    pub(crate) fov_ychange: f32,
    pub(crate) chr_lock_range_max_radius_for_dark: f32,
    pub(crate) chr_lock_range_max_radius_for_pitch_dark: f32,
    pub(crate) melee_attack_capture_upper_limit_height: f32,
    pub(crate) attack_capture_lower_limit_height: f32,
    pub(crate) attack_auto_acquisition_angle_range_left: f32,
    pub(crate) melee_attack_auto_acquisition_character_range_maximum_radius: f32,
    pub(crate) melee_attack_auto_acquisition_dark_character_range_maximum_radius: f32,
    pub(crate) proximity_attack_auto_acquisition_character_range_for_pure_darkness_maximum_radius: f32,
    pub(crate) bullet_auto_capture_character_range_maximum_radius: f32,
    pub(crate) bullet_auto_capture_dark_character_range_maximum_radius: f32,
    pub(crate) bullet_automatic_capture_character_range_ror_pure_darkness_maximum_radius: f32,
    pub(crate) bullet_auto_capturing_angle_range_left_and_right: f32,
    pub(crate) pad1: [u8; 28],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LodParam {
    pub(crate) lv01_border_dist: f32,
    pub(crate) lv01_play_dist: f32,
    pub(crate) lv12_border_dist: f32,
    pub(crate) lv12_play_dist: f32,
    pub(crate) texture_lod: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) lv23_border_dist: f32,
    pub(crate) lv23_play_dist: f32,
    pub(crate) lv34_border_dist: f32,
    pub(crate) lv34_play_dist: f32,
    pub(crate) lv45_border_dist: f32,
    pub(crate) lv45_play_dist: f32,
    pub(crate) distance_scale_id: u8,
    pub(crate) pad2: [u8; 19],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LodParam_ps4 {
    pub(crate) lv01_border_dist: f32,
    pub(crate) lv01_play_dist: f32,
    pub(crate) lv12_border_dist: f32,
    pub(crate) lv12_play_dist: f32,
    pub(crate) texture_lod: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) lv23_border_dist: f32,
    pub(crate) lv23_play_dist: f32,
    pub(crate) lv34_border_dist: f32,
    pub(crate) lv34_play_dist: f32,
    pub(crate) lv45_border_dist: f32,
    pub(crate) lv45_play_dist: f32,
    pub(crate) distance_scale_id: u8,
    pub(crate) pad2: [u8; 19],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct LodParam_xb1 {
    pub(crate) lv01_border_dist: f32,
    pub(crate) lv01_play_dist: f32,
    pub(crate) lv12_border_dist: f32,
    pub(crate) lv12_play_dist: f32,
    pub(crate) texture_lod: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) lv23_border_dist: f32,
    pub(crate) lv23_play_dist: f32,
    pub(crate) lv34_border_dist: f32,
    pub(crate) lv34_play_dist: f32,
    pub(crate) lv45_border_dist: f32,
    pub(crate) lv45_play_dist: f32,
    pub(crate) distance_scale_id: u8,
    pub(crate) pad2: [u8; 19],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Magic {
    pub(crate) yes_no_dialog_message_id: i32,
    pub(crate) limit_cancel_sp_effect_id: i32,
    pub(crate) sort_id: i16,
    pub(crate) ref_id: i16,
    pub(crate) ref_id_fp_cost1: i16,
    pub(crate) ref_id_sp_cost1: i16,
    pub(crate) icon_id: i16,
    pub(crate) behavior_id: i16,
    pub(crate) mtrl_item_id: i16,
    pub(crate) replace_magic_id: i16,
    pub(crate) max_quantity: i16,
    pub(crate) hero_point: u8,
    pub(crate) over_dexterity: u8,
    pub(crate) sfx_variation_id: u8,
    pub(crate) slot_length: u8,
    pub(crate) requirement_intellect: u8,
    pub(crate) requirement_faith: u8,
    pub(crate) analog_dexterity_min: u8,
    pub(crate) analog_dexterity_max: u8,
    pub(crate) ez_state_behavior_type: u8,
    pub(crate) ref_category1: u8,
    pub(crate) sp_effect_category: u8,
    pub(crate) ref_type: u8,
    pub(crate) menu_type: u8,
    pub(crate) ref_category4: u8,
    pub(crate) has_sp_effect_type: i16,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) unk7: u8,
    pub(crate) unk8: u8,
    pub(crate) cast_sfx1: i32,
    pub(crate) cast_sfx2: i32,
    pub(crate) cast_sfx3: i32,
    pub(crate) unk9: i32,
    pub(crate) attribute_type: u8,
    pub(crate) attribute_val0: i8,
    pub(crate) attribute_val1: i8,
    pub(crate) attribute_val2: i8,
    pub(crate) attribute_val3: i8,
    pub(crate) ref_category2: u8,
    pub(crate) ref_id_sp_cost4: i16,
    pub(crate) magic_id0: i32,
    pub(crate) magic_id1: i32,
    pub(crate) magic_id2: i32,
    pub(crate) magic_id3: i32,
    pub(crate) ref_id_fp_cost2: i16,
    pub(crate) ref_id_sp_cost2: i16,
    pub(crate) unk10: u8,
    pub(crate) ref_category3: u8,
    pub(crate) ref_id_fp_cost4: i16,
    pub(crate) ref_id_fp_cost3: i16,
    pub(crate) ref_id_sp_cost3: i16,
    pub(crate) ref_id1: i32,
    pub(crate) ref_id2: i32,
    pub(crate) ref_id3: i32,
    pub(crate) ref_id4: i32,
    pub(crate) pad1: [u8; 12],
}

impl Magic { 
    #[allow(unused)]
    pub(crate) fn set_vow_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_multi(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_multi(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_mult_only(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_mult_only(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enchant(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enchant(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_shield_enchant(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_shield_enchant(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_live(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_live(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_gray(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_gray(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_white(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_white(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_black(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_black(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_offline(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_offline(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_cast_resonance_magic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn cast_resonance_magic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type8(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type8(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type9(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type9(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MapMimicryEstablishmentParam {
    pub(crate) randomizer_coefficient0: f32,
    pub(crate) randomizer_coefficient1: f32,
    pub(crate) randomizer_coefficient2: f32,
    pub(crate) transform_vfx_id0: i32,
    pub(crate) loop_vfx_id0: i32,
    pub(crate) destroy_vfx_id0: i32,
    pub(crate) transform_vfx_id1: i32,
    pub(crate) loop_vfx_id1: i32,
    pub(crate) destroy_vfx_id1: i32,
    pub(crate) transform_vfx_id2: i32,
    pub(crate) loop_vfx_id2: i32,
    pub(crate) destroy_vfx_id2: i32,
    pub(crate) pad1: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MenuOffscrRendParam {
    pub(crate) menu_content0: f32,
    pub(crate) menu_content0_0: f32,
    pub(crate) menu_content0_1: f32,
    pub(crate) menu_content1: f32,
    pub(crate) menu_content1_2: f32,
    pub(crate) menu_content1_3: f32,
    pub(crate) menu_content1_4: f32,
    pub(crate) pad1: [u8; 16],
    pub(crate) screen_rend_id: i32,
    pub(crate) pad2: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MenuPropertyLayoutParam {
    pub(crate) layout_path: [u8; 16],
    pub(crate) property_id: i32,
    pub(crate) caption_text_id: i32,
    pub(crate) help_text_id: i32,
    pub(crate) pad1: [u8; 4],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MenuPropertySpecParam {
    pub(crate) caption_text_id: i32,
    pub(crate) icon_id: i32,
    pub(crate) required_property_id: i32,
    pub(crate) compare_type: u8,
    pub(crate) required_property_format_id: u8,
    pub(crate) adhoc_caption: [u16; 9],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MenuValueTableParam {
    pub(crate) value: i32,
    pub(crate) text_id: i32,
    pub(crate) compare_type: u8,
    pub(crate) pad1: [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ModelSfxParam {
    pub(crate) vfx_id1: i32,
    pub(crate) dummy_poly_id1: i32,
    pub(crate) pad1: [u8; 8],
    pub(crate) vfx_id2: i32,
    pub(crate) dummy_poly_id2: i32,
    pub(crate) pad2: [u8; 8],
    pub(crate) vfx_id3: i32,
    pub(crate) dummy_poly_id3: i32,
    pub(crate) pad3: [u8; 0],
    pub(crate) vfx_id4: i32,
    pub(crate) dummy_poly_id4: i32,
    pub(crate) pad4: [u8; 0],
    pub(crate) vfx_id5: i32,
    pub(crate) dummy_poly_id5: i32,
    pub(crate) pad5: [u8; 0],
    pub(crate) vfx_id6: i32,
    pub(crate) dummy_poly_id6: i32,
    pub(crate) pad6: [u8; 0],
    pub(crate) vfx_id7: i32,
    pub(crate) dummy_poly_id7: i32,
    pub(crate) pad7: [u8; 8],
    pub(crate) vfx_id8: i32,
    pub(crate) dummy_poly_id8: i32,
    pub(crate) pad8: [u8; 8],
    pub(crate) vfx_id9: i32,
    pub(crate) dummy_poly_id9: i32,
    pub(crate) pad9: [u8; 8],
    pub(crate) vfx_id10: i32,
    pub(crate) dummy_poly_id10: i32,
    pub(crate) pad10: [u8; 8],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MoveParam {
    pub(crate) stay_id: i32,
    pub(crate) walk_f: i32,
    pub(crate) walk_b: i32,
    pub(crate) walk_l: i32,
    pub(crate) walk_r: i32,
    pub(crate) dash_f: i32,
    pub(crate) dash_b: i32,
    pub(crate) dash_l: i32,
    pub(crate) dash_r: i32,
    pub(crate) super_dash: i32,
    pub(crate) escape_f: i32,
    pub(crate) escape_b: i32,
    pub(crate) escape_l: i32,
    pub(crate) escape_r: i32,
    pub(crate) turn_l: i32,
    pub(crate) turn_r: i32,
    pub(crate) large_turn_l: i32,
    pub(crate) large_turn_r: i32,
    pub(crate) step_move: i32,
    pub(crate) fly_stay: i32,
    pub(crate) fly_walk_f: i32,
    pub(crate) fly_walk_fl: i32,
    pub(crate) fly_walk_fr: i32,
    pub(crate) fly_walk_fl2: i32,
    pub(crate) fly_walk_fr2: i32,
    pub(crate) fly_dash_f: i32,
    pub(crate) fly_dash_fl: i32,
    pub(crate) fly_dash_fr: i32,
    pub(crate) fly_dash_fl2: i32,
    pub(crate) fly_dash_fr2: i32,
    pub(crate) dash_escape_f: i32,
    pub(crate) dash_escape_b: i32,
    pub(crate) dash_escape_l: i32,
    pub(crate) dash_escape_r: i32,
    pub(crate) analog_move_param_id: i32,
    pub(crate) turn_no_anim_angle: u8,
    pub(crate) turn45_angle: u8,
    pub(crate) turn90_angle: u8,
    pub(crate) turn_wait_no_anim_angle: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MPEstusFlaskRecoveryParam {
    pub(crate) recovery_count0: u8,
    pub(crate) recovery_count1: u8,
    pub(crate) recovery_count2: u8,
    pub(crate) recovery_count3: u8,
    pub(crate) recovery_count4: u8,
    pub(crate) recovery_count5: u8,
    pub(crate) recovery_count6: u8,
    pub(crate) recovery_count7: u8,
    pub(crate) recovery_count8: u8,
    pub(crate) recovery_count9: u8,
    pub(crate) recovery_count10: u8,
    pub(crate) recovery_count11: u8,
    pub(crate) recovery_count12: u8,
    pub(crate) recovery_count13: u8,
    pub(crate) recovery_count14: u8,
    pub(crate) recovery_count15: u8,
    pub(crate) recovery_count16: u8,
    pub(crate) recovery_count17: u8,
    pub(crate) recovery_count18: u8,
    pub(crate) recovery_count19: u8,
    pub(crate) recovery_count20: u8,
    pub(crate) recovery_count21: u8,
    pub(crate) recovery_count22: u8,
    pub(crate) recovery_count23: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MultiHPEstusFlaskBonusParam {
    pub(crate) estus_flask_restore_count0: u8,
    pub(crate) estus_flask_restore_count1: u8,
    pub(crate) estus_flask_restore_count2: u8,
    pub(crate) estus_flask_restore_count3: u8,
    pub(crate) estus_flask_restore_count4: u8,
    pub(crate) estus_flask_restore_count5: u8,
    pub(crate) estus_flask_restore_count6: u8,
    pub(crate) estus_flask_restore_count7: u8,
    pub(crate) estus_flask_restore_count8: u8,
    pub(crate) estus_flask_restore_count9: u8,
    pub(crate) estus_flask_restore_count10: u8,
    pub(crate) estus_flask_restore_count11: u8,
    pub(crate) estus_flask_restore_count12: u8,
    pub(crate) estus_flask_restore_count13: u8,
    pub(crate) estus_flask_restore_count14: u8,
    pub(crate) estus_flask_restore_count15: u8,
    pub(crate) estus_flask_restore_count16: u8,
    pub(crate) estus_flask_restore_count17: u8,
    pub(crate) estus_flask_restore_count18: u8,
    pub(crate) estus_flask_restore_count19: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MultiMPEstusFlaskBonusParam {
    pub(crate) estus_flask_restore_count0: u8,
    pub(crate) estus_flask_restore_count1: u8,
    pub(crate) estus_flask_restore_count2: u8,
    pub(crate) estus_flask_restore_count3: u8,
    pub(crate) estus_flask_restore_count4: u8,
    pub(crate) estus_flask_restore_count5: u8,
    pub(crate) estus_flask_restore_count6: u8,
    pub(crate) estus_flask_restore_count7: u8,
    pub(crate) estus_flask_restore_count8: u8,
    pub(crate) estus_flask_restore_count9: u8,
    pub(crate) estus_flask_restore_count10: u8,
    pub(crate) estus_flask_restore_count11: u8,
    pub(crate) estus_flask_restore_count12: u8,
    pub(crate) estus_flask_restore_count13: u8,
    pub(crate) estus_flask_restore_count14: u8,
    pub(crate) estus_flask_restore_count15: u8,
    pub(crate) estus_flask_restore_count16: u8,
    pub(crate) estus_flask_restore_count17: u8,
    pub(crate) estus_flask_restore_count18: u8,
    pub(crate) estus_flask_restore_count19: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MultiPlayCorrectionParam {
    pub(crate) correction_val0: i32,
    pub(crate) correction_val1: i32,
    pub(crate) correction_val2: i32,
    pub(crate) correction_val3: i32,
    pub(crate) pad1: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct MultiSoulBonusRateParam {
    pub(crate) soul_multiplier_rate0: f32,
    pub(crate) soul_multiplier_rate1: f32,
    pub(crate) soul_multiplier_rate2: f32,
    pub(crate) soul_multiplier_rate3: f32,
    pub(crate) soul_multiplier_rate4: f32,
    pub(crate) soul_multiplier_rate5: f32,
    pub(crate) soul_multiplier_rate6: f32,
    pub(crate) soul_multiplier_rate7: f32,
    pub(crate) soul_multiplier_rate8: f32,
    pub(crate) soul_multiplier_rate9: f32,
    pub(crate) soul_multiplier_rate10: f32,
    pub(crate) soul_multiplier_rate11: f32,
    pub(crate) soul_multiplier_rate12: f32,
    pub(crate) soul_multiplier_rate13: f32,
    pub(crate) soul_multiplier_rate14: f32,
    pub(crate) soul_multiplier_rate15: f32,
    pub(crate) pad1: [u8; 4],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NetworkAreaParam {
    pub(crate) limitation_time0: f32,
    pub(crate) limitation_time1: f32,
    pub(crate) limitation_time2: f32,
    pub(crate) pad1: [u8; 12],
    pub(crate) bitfield0: u8,
    pub(crate) pad2: [u8; 3],
}

impl NetworkAreaParam { 
    #[allow(unused)]
    pub(crate) fn set_is_enable00(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable00(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enable01(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable01(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enable02(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable02(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unkb5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unkb5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NetworkMsgParam {
    pub(crate) msg_type0: u8,
    pub(crate) msg_type1: u8,
    pub(crate) msg_type2: u8,
    pub(crate) msg_type3: u8,
    pub(crate) msg_id0: i32,
    pub(crate) msg_id1: i32,
    pub(crate) msg_id2: i32,
    pub(crate) msg_id3: i32,
    pub(crate) msg_id4: i32,
    pub(crate) msg_id5: i32,
    pub(crate) msg_id6: i32,
    pub(crate) msg_id7: i32,
    pub(crate) msg_id8: i32,
    pub(crate) msg_id9: i32,
    pub(crate) msg_id10: i32,
    pub(crate) msg_id11: i32,
    pub(crate) msg_id12: i32,
    pub(crate) msg_id13: i32,
    pub(crate) msg_id14: i32,
    pub(crate) msg_id15: i32,
    pub(crate) msg_id16: i32,
    pub(crate) msg_id17: i32,
    pub(crate) msg_id18: i32,
    pub(crate) msg_id19: i32,
    pub(crate) msg_id20: i32,
    pub(crate) msg_id21: i32,
    pub(crate) msg_id22: i32,
    pub(crate) pad1: [u8; 48],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NetworkParam {
    pub(crate) network_data: [u8; 632],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NewMenuColorTableParam {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NpcAiActionParam {
    pub(crate) direction_movement_id: u8,
    pub(crate) act_id0: u8,
    pub(crate) act_id1: u8,
    pub(crate) act_id2: u8,
    pub(crate) is_disable_direction_movement: u8,
    pub(crate) is_disable_act0: u8,
    pub(crate) is_disable_act1: u8,
    pub(crate) is_disable_act2: u8,
    pub(crate) act_type: i32,
    pub(crate) is_disable_ai_check: u8,
    pub(crate) pad1: [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NpcParam {
    pub(crate) behavior_variation_id: i32,
    pub(crate) ai_think_id: i32,
    pub(crate) name_id: i32,
    pub(crate) turn_vellocity: f32,
    pub(crate) hit_height: f32,
    pub(crate) hit_radius: f32,
    pub(crate) weight: i32,
    pub(crate) hit_offset: f32,
    pub(crate) hp: i32,
    pub(crate) mp: i32,
    pub(crate) get_soul: i32,
    pub(crate) item_lot_id1: i32,
    pub(crate) item_lot_id2: i32,
    pub(crate) item_lot_id3: i32,
    pub(crate) item_lot_id4: i32,
    pub(crate) item_lot_id5: i32,
    pub(crate) item_lot_id6: i32,
    pub(crate) humanity_lot_id: i32,
    pub(crate) sp_effect_id0: i32,
    pub(crate) sp_effect_id1: i32,
    pub(crate) sp_effect_id2: i32,
    pub(crate) sp_effect_id3: i32,
    pub(crate) sp_effect_id4: i32,
    pub(crate) sp_effect_id5: i32,
    pub(crate) sp_effect_id6: i32,
    pub(crate) sp_effect_id7: i32,
    pub(crate) game_clear_sp_effect_id: i32,
    pub(crate) phys_guard_cut_rate: f32,
    pub(crate) mag_guard_cut_rate: f32,
    pub(crate) fire_guard_cut_rate: f32,
    pub(crate) thun_guard_cut_rate: f32,
    pub(crate) animid_offset: i32,
    pub(crate) move_anim_id: i32,
    pub(crate) sp_move_anim_id1: i32,
    pub(crate) sp_move_anim_id2: i32,
    pub(crate) network_warp_disp: f32,
    pub(crate) dbg_behavior_r1: i32,
    pub(crate) dbg_behavior_l1: i32,
    pub(crate) dbg_behavior_r2: i32,
    pub(crate) dbg_behavior_l2: i32,
    pub(crate) dbg_behavior_rl: i32,
    pub(crate) dbg_behavior_rr: i32,
    pub(crate) dbg_behavior_rd: i32,
    pub(crate) dbg_behavior_ru: i32,
    pub(crate) dbg_behavior_ll: i32,
    pub(crate) dbg_behavior_lr: i32,
    pub(crate) dbg_behavior_ld: i32,
    pub(crate) dbg_behavior_lu: i32,
    pub(crate) anim_id_offset: i32,
    pub(crate) parts_damage_rate1: f32,
    pub(crate) parts_damage_rate2: f32,
    pub(crate) parts_damage_rate3: f32,
    pub(crate) parts_damage_rate4: f32,
    pub(crate) parts_damage_rate5: f32,
    pub(crate) parts_damage_rate6: f32,
    pub(crate) parts_damage_rate7: f32,
    pub(crate) parts_damage_rate8: f32,
    pub(crate) weak_parts_damage_rate: f32,
    pub(crate) super_armor_recover_correction: f32,
    pub(crate) super_armor_brake_knockback_dist: f32,
    pub(crate) stamina: i16,
    pub(crate) stamina_recover_base_val: i16,
    pub(crate) def_phys: i16,
    pub(crate) def_slash: i16,
    pub(crate) def_blow: i16,
    pub(crate) def_thrust: i16,
    pub(crate) def_mag: i16,
    pub(crate) def_fire: i16,
    pub(crate) def_thunder: i16,
    pub(crate) def_flick_power: i16,
    pub(crate) resist_poison: i16,
    pub(crate) resist_toxic: i16,
    pub(crate) resist_blood: i16,
    pub(crate) resist_curse: i16,
    pub(crate) ghost_model_id: i16,
    pub(crate) normal_change_resource_id: i16,
    pub(crate) guard_angle: i16,
    pub(crate) slash_guard_cut_rate: i16,
    pub(crate) blow_guard_cut_rate: i16,
    pub(crate) thrust_guard_cut_rate: i16,
    pub(crate) super_armor_durability: i16,
    pub(crate) normal_change_tex_chr_id: i16,
    pub(crate) drop_type: i16,
    pub(crate) knockback_rate: u8,
    pub(crate) knockback_param_id: u8,
    pub(crate) fall_damage_damp: u8,
    pub(crate) stamina_guard_def: u8,
    pub(crate) pc_attr_b: u8,
    pub(crate) pc_attr_w: u8,
    pub(crate) pc_attr_l: u8,
    pub(crate) pc_attr_r: u8,
    pub(crate) area_attr_b: u8,
    pub(crate) area_attr_w: u8,
    pub(crate) area_attr_l: u8,
    pub(crate) area_attr_r: u8,
    pub(crate) mp_recover_base_val: u8,
    pub(crate) flick_damage_cut_rate: u8,
    pub(crate) default_lod_param_id: i8,
    pub(crate) draw_type: u8,
    pub(crate) npc_type: u8,
    pub(crate) team_type: u8,
    pub(crate) move_type: u8,
    pub(crate) lock_dist: u8,
    pub(crate) material: i16,
    pub(crate) material_sfx: i16,
    pub(crate) parts_damage_type: u8,
    pub(crate) max_unduration_ang: u8,
    pub(crate) guard_level: u8,
    pub(crate) burnsfx_type: u8,
    pub(crate) poison_guard_resist: u8,
    pub(crate) toxic_guard_resist: u8,
    pub(crate) blood_guard_resist: u8,
    pub(crate) curse_guard_resist: u8,
    pub(crate) parry_attack: u8,
    pub(crate) parry_defense: u8,
    pub(crate) sfx_size: u8,
    pub(crate) push_out_cam_region_radius: u8,
    pub(crate) hit_stop_type: u8,
    pub(crate) ladder_end_chk_offset_top: u8,
    pub(crate) ladder_end_chk_offset_low: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) bitfield4: u8,
    pub(crate) bitfield5: u8,
    pub(crate) bitfield6: u8,
    pub(crate) item_search_radius: f32,
    pub(crate) sub_hit_height: f32,
    pub(crate) sub_hit_radius: f32,
    pub(crate) turn_velocity_type: u8,
    pub(crate) bitfield7: u8,
    pub(crate) def_dark: i16,
    pub(crate) sub_turn_velocity: f32,
    pub(crate) foot_step_id: i32,
    pub(crate) sub_material: i16,
    pub(crate) sub_material_sfx: i16,
    pub(crate) material_weak: i16,
    pub(crate) material_sfx_weak: i16,
    pub(crate) sub_material_weak: i16,
    pub(crate) sub_material_sfx_weak: i16,
    pub(crate) sp_effect_id8: i32,
    pub(crate) sp_effect_id9: i32,
    pub(crate) sp_effect_id10: i32,
    pub(crate) sp_effect_id11: i32,
    pub(crate) sp_effect_id12: i32,
    pub(crate) sp_effect_id13: i32,
    pub(crate) sp_effect_id14: i32,
    pub(crate) sp_effect_id15: i32,
    pub(crate) tentative_player_id: i32,
    pub(crate) basic_toughness_value: i32,
    pub(crate) game_system_param_correction: f32,
    pub(crate) regain_rate_phys_neutral: f32,
    pub(crate) regain_rate_phys_slash: f32,
    pub(crate) regain_rate_phys_blow: f32,
    pub(crate) regain_rate_phys_thrust: f32,
    pub(crate) regain_rate_magic: f32,
    pub(crate) regain_rate_fire: f32,
    pub(crate) regain_rate_thunder: f32,
    pub(crate) regain_rate_dark: f32,
    pub(crate) max_ankle_angle: f32,
    pub(crate) cloth_update_offset: i8,
    pub(crate) sp_npc_type: u8,
    pub(crate) normal_change_model_id: i16,
    pub(crate) normal_change_anim_chr_id: i16,
    pub(crate) stamina_guard_def_val: i16,
    pub(crate) cult_setting_id: i32,
    pub(crate) phantom_param_id: i32,
    pub(crate) multi_play_correction_id: i32,
    pub(crate) foot_ankle: f32,
    pub(crate) resist_frost: i16,
    pub(crate) sub_npc_type: u8,
    pub(crate) sub_team_type: u8,
    pub(crate) lock_cam_param_id: i32,
    pub(crate) sp_effect_id16: i32,
    pub(crate) sp_effect_id17: i32,
    pub(crate) sp_effect_id18: i32,
    pub(crate) sp_effect_id19: i32,
    pub(crate) sp_effect_id20: i32,
    pub(crate) sp_effect_id21: i32,
    pub(crate) sp_effect_id22: i32,
    pub(crate) sp_effect_id23: i32,
    pub(crate) sp_effect_id24: i32,
    pub(crate) sp_effect_id25: i32,
    pub(crate) sp_effect_id26: i32,
    pub(crate) sp_effect_id27: i32,
    pub(crate) sp_effect_id28: i32,
    pub(crate) sp_effect_id29: i32,
    pub(crate) sp_effect_id30: i32,
    pub(crate) sp_effect_id31: i32,
    pub(crate) lock_correction: f32,
    pub(crate) sub_cloth_update_offset: i8,
    pub(crate) pad1: [u8; 1],
    pub(crate) estus_flask_param_id: i16,
    pub(crate) text_id: i32,
    pub(crate) h_p: i16,
    pub(crate) h_p_restore_id0: i16,
    pub(crate) m_p_restore_id0: i16,
    pub(crate) h_p_0: i16,
    pub(crate) h_p_restore_id1: i16,
    pub(crate) m_p_restore_id1: i16,
    pub(crate) sub_phantom_param_id: i32,
    pub(crate) activate_distance: i16,
    pub(crate) deactivate_distance: i16,
    pub(crate) pad2: [u8; 4],
}

impl NpcParam { 
    #[allow(unused)]
    pub(crate) fn set_use_ragdoll_cam_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_ragdoll_cam_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_cloth_rigid_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_cloth_rigid_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_ragdoll(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_ragdoll(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_demon(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_demon(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ghost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ghost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_no_damage_motion(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_no_damage_motion(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_unduration(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_unduration(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_change_wander_ghost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_change_wander_ghost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask8(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask8(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask9(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask9(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enable_neck_turn(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable_neck_turn(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_respawn(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_respawn(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_move_anim_wait(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_move_anim_wait(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_crowd(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_crowd(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_weak_saint(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_weak_saint(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_weak_a(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_weak_a(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_weak_b(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_weak_b(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_enable_drop_soul_capture(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable_drop_soul_capture(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_intiliaze_dead(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_intiliaze_dead(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_soul_get_by_boss(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_soul_get_by_boss(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_multilingual(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_multilingual(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask16(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask16(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask17(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask17(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask18(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask18(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask19(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask19(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask20(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask20(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask21(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask21(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask22(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask22(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask23(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask23(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask24(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask24(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask25(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask25(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask26(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask26(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask27(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask27(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask28(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask28(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask29(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask29(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask30(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask30(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_model_disp_mask31(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn model_disp_mask31(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_mult_mode_boss(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_mult_mode_boss(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_beh_mem_size(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn beh_mem_size(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_use_feet_data(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_use_feet_data(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct NpcThinkParam {
    pub(crate) logic_id: i32,
    pub(crate) battle_goal_id: i32,
    pub(crate) near_dist: f32,
    pub(crate) mid_dist: f32,
    pub(crate) far_dist: f32,
    pub(crate) out_dist: f32,
    pub(crate) back_home_life_on_hit_ene_wal: f32,
    pub(crate) goal_id_to_caution: f32,
    pub(crate) id_attack_cannot_move: i32,
    pub(crate) goal_id_to_find: f32,
    pub(crate) call_help_action_anim_id: i32,
    pub(crate) call_help_call_action_id: i32,
    pub(crate) eye_dist: i16,
    pub(crate) ear_dist: i16,
    pub(crate) ear_soundcut_dist: i16,
    pub(crate) nose_dist: i16,
    pub(crate) max_backhome_dist: i16,
    pub(crate) backhome_dist: i16,
    pub(crate) backhome_battle_dist: i16,
    pub(crate) non_battle_act_life: i16,
    pub(crate) back_home_look_target_time: i16,
    pub(crate) back_home_look_target_dist: i16,
    pub(crate) sight_target_forget_time: i16,
    pub(crate) sound_target_forget_time: i16,
    pub(crate) battle_start_dist: i16,
    pub(crate) call_help_my_peer_id: i16,
    pub(crate) call_help_call_peer_id: i16,
    pub(crate) target_sys_dmg_effect_rate: i16,
    pub(crate) team_attack_effectivity: u8,
    pub(crate) eye_ang_x: u8,
    pub(crate) eye_ang_y: u8,
    pub(crate) state0: u8,
    pub(crate) state1: u8,
    pub(crate) call_help_call_valid_min_dist_target: u8,
    pub(crate) call_help_call_valid_range: u8,
    pub(crate) call_help_forget_time_by_arrival: u8,
    pub(crate) call_help_min_wait_time: u8,
    pub(crate) call_help_max_wait_time: u8,
    pub(crate) goal_action_to_caution: u8,
    pub(crate) goal_action_to_find: u8,
    pub(crate) call_help_reply_behavior_type: u8,
    pub(crate) disable_path_move: u8,
    pub(crate) skip_arrival_visible_check: u8,
    pub(crate) think_attr_do_admirer: u8,
    pub(crate) bitfield0: u8,
    pub(crate) enable_navi_flag_reserve: [u8; 3],
    pub(crate) eye_dist_for_dark: i16,
    pub(crate) battle_start_dist_for_dark: i16,
    pub(crate) eye_dist_for_pitch_dark: i16,
    pub(crate) battle_start_dist_for_pitch_dark: i16,
    pub(crate) platoon_reply_time: f32,
    pub(crate) platoon_reply_add_random_time: f32,
    pub(crate) eye_back_offset_dist: i16,
    pub(crate) eye_begin_dist: i16,
    pub(crate) target_arrive_dist: f32,
    pub(crate) point_arrive_dist: f32,
    pub(crate) change_state_action_to_find: u8,
    pub(crate) change_state_action_to_caution: u8,
    pub(crate) change_state_action_to_battle: u8,
    pub(crate) goal_action_to_disappear: u8,
    pub(crate) disable_local_steering: u8,
    pub(crate) act_type_on_failed_path: u8,
    pub(crate) interest_category: u8,
    pub(crate) goal_action_to_interest: u8,
    pub(crate) unk1: i16,
    pub(crate) ear_ang_x: i16,
    pub(crate) ear_ang_y: i16,
    pub(crate) change_state_action_to_interest: u8,
    pub(crate) change_state_action_to_normal: u8,
    pub(crate) unk2: f32,
    pub(crate) unk3: f32,
    pub(crate) unk4: i16,
    pub(crate) unk5: u8,
    pub(crate) unk6: u8,
    pub(crate) unk7: f32,
    pub(crate) pad: [u8; 12],
}

impl NpcThinkParam { 
    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_edge(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_edge(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_large_space(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_large_space(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_ladder(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_ladder(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_hole(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_hole(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_door(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_door(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_in_side_wall(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_in_side_wall(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_edge_ordinary(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_edge_ordinary(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_navi_flg_reserve0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_navi_flg_reserve0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ObjActParam {
    pub(crate) action_enable_msg_id: i32,
    pub(crate) action_failed_msg_id: i32,
    pub(crate) sp_qualified_pass_event_flag: i32,
    pub(crate) player_anim_id: i32,
    pub(crate) chr_anim_id: i32,
    pub(crate) valid_dist: i16,
    pub(crate) sp_qualified_id: i16,
    pub(crate) sp_qualified_id2: i16,
    pub(crate) obj_dummy_id: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) obj_anim_id: i32,
    pub(crate) valid_player_angle: u8,
    pub(crate) sp_qualified_type: u8,
    pub(crate) sp_qualified_type2: u8,
    pub(crate) valid_obj_angle: u8,
    pub(crate) chr_sorb_type: u8,
    pub(crate) event_kick_timing: u8,
    pub(crate) pad2: [u8; 2],
    pub(crate) action_button_param_id: i32,
    pub(crate) action_success_msg_id: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ObjectMaterialSfxParam {
    pub(crate) mtrl_vfx_id0: i32,
    pub(crate) mtrl_vfx_id1: i32,
    pub(crate) mtrl_vfx_id2: i32,
    pub(crate) mtrl_vfx_id3: i32,
    pub(crate) mtrl_vfx_id4: i32,
    pub(crate) mtrl_vfx_id5: i32,
    pub(crate) mtrl_vfx_id6: i32,
    pub(crate) mtrl_vfx_id7: i32,
    pub(crate) mtrl_vfx_id8: i32,
    pub(crate) mtrl_vfx_id9: i32,
    pub(crate) mtrl_vfx_id10: i32,
    pub(crate) mtrl_vfx_id11: i32,
    pub(crate) mtrl_vfx_id12: i32,
    pub(crate) mtrl_vfx_id13: i32,
    pub(crate) mtrl_vfx_id14: i32,
    pub(crate) mtrl_vfx_id15: i32,
    pub(crate) mtrl_vfx_id16: i32,
    pub(crate) mtrl_vfx_id17: i32,
    pub(crate) mtrl_vfx_id18: i32,
    pub(crate) mtrl_vfx_id19: i32,
    pub(crate) mtrl_vfx_id20: i32,
    pub(crate) mtrl_vfx_id21: i32,
    pub(crate) mtrl_vfx_id22: i32,
    pub(crate) mtrl_vfx_id23: i32,
    pub(crate) mtrl_vfx_id24: i32,
    pub(crate) mtrl_vfx_id25: i32,
    pub(crate) mtrl_vfx_id26: i32,
    pub(crate) mtrl_vfx_id27: i32,
    pub(crate) mtrl_vfx_id28: i32,
    pub(crate) mtrl_vfx_id29: i32,
    pub(crate) mtrl_vfx_id30: i32,
    pub(crate) mtrl_vfx_id31: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ObjectParam {
    pub(crate) h_p: i16,
    pub(crate) defense: u16,
    pub(crate) ext_ref_tex_id: i16,
    pub(crate) material_id: i16,
    pub(crate) anim_break_id_max: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) default_lod_param_id: i8,
    pub(crate) break_sfx_id: i32,
    pub(crate) beh_param_id0: i32,
    pub(crate) beh_param_id1: i32,
    pub(crate) beh_param_id2: i32,
    pub(crate) unk2: u8,
    pub(crate) havok_system_wind: u8,
    pub(crate) havok_system_break_obj: u8,
    pub(crate) unk5: u8,
    pub(crate) wind_effect_rate: f32,
    pub(crate) unk7: f32,
    pub(crate) break_obj_rate: f32,
    pub(crate) burn_life_time: f32,
    pub(crate) break_update: f32,
    pub(crate) burn_vfx_id0: i32,
    pub(crate) burn_vfx_id1: i32,
    pub(crate) burn_vfx_id2: i32,
    pub(crate) burn_vfx_id3: i32,
    pub(crate) burn_behavior_id0: i32,
    pub(crate) burn_behavior_id1: i32,
    pub(crate) burn_behavior_id2: i32,
    pub(crate) burn_behavior_id3: i32,
    pub(crate) burn_interval: u16,
    pub(crate) unk20: u8,
    pub(crate) unk22: u8,
    pub(crate) burn_behavior_time: f32,
    pub(crate) burn_time_begin0: f32,
    pub(crate) burn_time_begin1: f32,
    pub(crate) burn_time_begin2: f32,
    pub(crate) burn_time_begin3: f32,
    pub(crate) burn_time_end0: f32,
    pub(crate) burn_time_end1: f32,
    pub(crate) burn_time_end2: f32,
    pub(crate) burn_time_end3: f32,
    pub(crate) ai_sound_param_id: i32,
    pub(crate) unk32: f32,
    pub(crate) unk33: f32,
    pub(crate) unk34: f32,
    pub(crate) unk35: f32,
    pub(crate) unk36: f32,
    pub(crate) unk37: f32,
    pub(crate) unk38: f32,
    pub(crate) unk39: f32,
    pub(crate) unk40: f32,
    pub(crate) unk41: i32,
    pub(crate) unk42: i16,
    pub(crate) spawn_param: i16,
    pub(crate) auto_destroy_timer: f32,
    pub(crate) unk44: f32,
    pub(crate) sound_id: i32,
    pub(crate) object_material_sfx_index: i32,
    pub(crate) pad1: [u8; 68],
}

impl ObjectParam { 
    #[allow(unused)]
    pub(crate) fn set_is_cam_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_cam_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_break_by_player_collide(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_break_by_player_collide(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_anim_break(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_anim_break(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_penetration_bullet_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_penetration_bullet_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_chr_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_chr_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_attack_backlash(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_attack_backlash(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_disable_break_for_first_appear(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_disable_break_for_first_appear(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_ladder(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_ladder(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_anim_pause_on_remo_play(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_anim_pause_on_remo_play(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_damage_no_hit(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_damage_no_hit(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_move_obj(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_move_obj(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk_bool1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk_bool1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk_bool2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk_bool2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk_bool3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk_bool3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_map_related(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn map_related(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_break_by_collide2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_break_by_collide2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct PhantomParam {
    pub(crate) alpha1: f32,
    pub(crate) alpha2: f32,
    pub(crate) alpha3: f32,
    pub(crate) alpha4: f32,
    pub(crate) alpha5: f32,
    pub(crate) r1: u8,
    pub(crate) g1: u8,
    pub(crate) b1: u8,
    pub(crate) r2: u8,
    pub(crate) g2: u8,
    pub(crate) b2: u8,
    pub(crate) r3: u8,
    pub(crate) g3: u8,
    pub(crate) b3: u8,
    pub(crate) r4: u8,
    pub(crate) g4: u8,
    pub(crate) b4: u8,
    pub(crate) r5: u8,
    pub(crate) g5: u8,
    pub(crate) b5: u8,
    pub(crate) unk1: u8,
    pub(crate) ghost_alpha1: f32,
    pub(crate) ghost_alpha2: f32,
    pub(crate) ghost_type: u8,
    pub(crate) pad1: [u8; 3],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct PlayRegionParam {
    pub(crate) play_region_sp_id: i32,
    pub(crate) event_flag_id0: i32,
    pub(crate) limitation_time: f32,
    pub(crate) event_flag_id1: i32,
    pub(crate) event_flag_id2: i32,
    pub(crate) disolved_event_flag: i16,
    pub(crate) load_of_cinder: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bonfire_event_id0: i32,
    pub(crate) bonfire_event_id1: i32,
    pub(crate) bonfire_event_id2: i32,
    pub(crate) bonfire_event_id3: i32,
    pub(crate) bonfire_event_id4: i32,
    pub(crate) bonfire_event_id5: i32,
    pub(crate) bonfire_event_id6: i32,
    pub(crate) bonfire_event_id7: i32,
    pub(crate) bonfire_event_id8: i32,
    pub(crate) bonfire_event_id9: i32,
    pub(crate) bitfield1: u8,
    pub(crate) pad1: [u8; 31],
}

impl PlayRegionParam { 
    #[allow(unused)]
    pub(crate) fn set_is_enable_event(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_enable_event(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk8(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk8(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk9(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk9(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ProtectorGenParam {
    pub(crate) pro_param_id: i32,
    pub(crate) gem_slot_type_0: u32,
    pub(crate) gem_gen_id_0: i32,
    pub(crate) gem_slot_type_1: u32,
    pub(crate) gem_gen_id_1: i32,
    pub(crate) gem_slot_type_2: u32,
    pub(crate) gem_gen_id_2: i32,
    pub(crate) gem_slot_type_3: u32,
    pub(crate) gem_gen_id_3: i32,
    pub(crate) gem_slot_type_4: u32,
    pub(crate) gem_gen_id_4: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct RagdollParam {
    pub(crate) hierarch_gain: f32,
    pub(crate) velocity_damping: f32,
    pub(crate) accel_gain: f32,
    pub(crate) velocity_gain: f32,
    pub(crate) position_gain: f32,
    pub(crate) max_liner_velocity: f32,
    pub(crate) max_angular_velocity: f32,
    pub(crate) snap_gain: f32,
    pub(crate) enable: u8,
    pub(crate) parts_hit_mask_no: i8,
    pub(crate) pad1: [u8; 14],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ReinforceParamProtector {
    pub(crate) physic_def_rate: f32,
    pub(crate) magic_def_rate: f32,
    pub(crate) fire_def_rate: f32,
    pub(crate) thunder_def_rate: f32,
    pub(crate) slash_def_rate: f32,
    pub(crate) blow_def_rate: f32,
    pub(crate) thrust_def_rate: f32,
    pub(crate) resist_poison_rate: f32,
    pub(crate) resist_toxic_rate: f32,
    pub(crate) resist_blood_rate: f32,
    pub(crate) resist_curse_rate: f32,
    pub(crate) resident_sp_effect_id1: u8,
    pub(crate) resident_sp_effect_id2: u8,
    pub(crate) resident_sp_effect_id3: u8,
    pub(crate) material_set_id: u8,
    pub(crate) dark_def_rate: f32,
    pub(crate) resist_frost: f32,
    pub(crate) pad1: [u8; 8],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ReinforceParamWeapon {
    pub(crate) physics_atk_rate: f32,
    pub(crate) magic_atk_rate: f32,
    pub(crate) fire_atk_rate: f32,
    pub(crate) thunder_atk_rate: f32,
    pub(crate) stamina_atk_rate: f32,
    pub(crate) sa_weapon_atk_rate: f32,
    pub(crate) sa_durability_rate: f32,
    pub(crate) correct_strength_rate: f32,
    pub(crate) correct_agility_rate: f32,
    pub(crate) correct_magic_rate: f32,
    pub(crate) correct_faith_rate: f32,
    pub(crate) physics_guard_cut_rate: f32,
    pub(crate) magic_guard_cut_rate: f32,
    pub(crate) fire_guard_cut_rate: f32,
    pub(crate) thunder_guard_cut_rate: f32,
    pub(crate) poison_guard_resist_rate: f32,
    pub(crate) toxic_guard_resist_rate: f32,
    pub(crate) bleed_guard_resist_rate: f32,
    pub(crate) curse_guard_resist_rate: f32,
    pub(crate) stamina_guard_resist_rate: f32,
    pub(crate) sp_effect_id1: u8,
    pub(crate) sp_effect_id2: u8,
    pub(crate) sp_effect_id3: u8,
    pub(crate) resident_sp_effect_id1: u8,
    pub(crate) resident_sp_effect_id2: u8,
    pub(crate) resident_sp_effect_id3: u8,
    pub(crate) material_set_id: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) dark_atk_rate: f32,
    pub(crate) dark_cut_rate: f32,
    pub(crate) stability_atk_rate: f32,
    pub(crate) stability_cut_rate: f32,
    pub(crate) frost_guard_resist_rate: f32,
    pub(crate) unk1: f32,
    pub(crate) pad2: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct RoleParam {
    pub(crate) team_type: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) phantom_param_id0: i32,
    pub(crate) sp_effect_id0: i32,
    pub(crate) sp_effect_id1: i32,
    pub(crate) sp_effect_id2: i32,
    pub(crate) sp_effect_id3: i32,
    pub(crate) sp_effect_id4: i32,
    pub(crate) sp_effect_id5: i32,
    pub(crate) sp_effect_id6: i32,
    pub(crate) sp_effect_id7: i32,
    pub(crate) sp_effect_id8: i32,
    pub(crate) sp_effect_id9: i32,
    pub(crate) sfx_id0: i32,
    pub(crate) sfx_id1: i32,
    pub(crate) stay_anim_id: i32,
    pub(crate) item_lot_id: i32,
    pub(crate) sp_effect_condition: u8,
    pub(crate) is_display_team_name: u8,
    pub(crate) pad2: [u8; 2],
    pub(crate) text_id: i32,
    pub(crate) sub_team_type: i32,
    pub(crate) phantom_param_id1: i32,
    pub(crate) phantom_param_id2: i32,
    pub(crate) phantom_param_id3: i32,
    pub(crate) sp_effect10: i32,
    pub(crate) sp_effect11: i32,
    pub(crate) sp_effect12: i32,
    pub(crate) sp_effect13: i32,
    pub(crate) phantom_param_id_for_debug: i32,
    pub(crate) pad3: [u8; 20],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SeMaterialConvertParam {
    pub(crate) material_id: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ShopLineupParam {
    pub(crate) equip_id: i32,
    pub(crate) value: i32,
    pub(crate) mtrl_id: i32,
    pub(crate) event_flag: i32,
    pub(crate) qwc_id: i32,
    pub(crate) sell_quantity: i16,
    pub(crate) shop_type: u8,
    pub(crate) equip_type: u8,
    pub(crate) value_san: i16,
    pub(crate) pad1: [u8; 6],
    pub(crate) price_rate: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SkeletonParam {
    pub(crate) neck_turn_gain: f32,
    pub(crate) original_ground_height_ms: i16,
    pub(crate) min_ankle_height_ms: i16,
    pub(crate) max_ankle_height_ms: i16,
    pub(crate) cosine_max_knee_angle: i16,
    pub(crate) cosine_min_knee_angle: i16,
    pub(crate) foot_planted_ankle_height_ms: i16,
    pub(crate) foot_raised_ankle_height_ms: i16,
    pub(crate) raycast_distance_up: i16,
    pub(crate) raycast_distance_down: i16,
    pub(crate) foot_end_ls_x: i16,
    pub(crate) foot_end_ls_y: i16,
    pub(crate) foot_end_ls_z: i16,
    pub(crate) on_off_gain: i16,
    pub(crate) ground_acsending_gain: i16,
    pub(crate) ground_descending_gain: i16,
    pub(crate) foot_raised_gain: i16,
    pub(crate) foot_planted_gain: i16,
    pub(crate) foot_unlock_gain: i16,
    pub(crate) knee_axis_type: u8,
    pub(crate) use_foot_locking: u8,
    pub(crate) foot_placement_on: u8,
    pub(crate) twist_knee_axis_type: u8,
    pub(crate) neck_turn_priority: u8,
    pub(crate) neck_turn_max_angle: u8,
    pub(crate) pad1: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SpEffectParam {
    pub(crate) icon_id: i32,
    pub(crate) condition_hp: f32,
    pub(crate) effect_endurance: f32,
    pub(crate) motion_interval: f32,
    pub(crate) max_hp_rate: f32,
    pub(crate) max_mp_rate: f32,
    pub(crate) max_stamina_cut_rate: f32,
    pub(crate) slash_damage_cut_rate: f32,
    pub(crate) blow_damage_cut_rate: f32,
    pub(crate) thrust_damage_cut_rate: f32,
    pub(crate) neutral_damage_cut_rate: f32,
    pub(crate) magic_damage_cut_rate: f32,
    pub(crate) fire_damage_cut_rate: f32,
    pub(crate) thunder_damage_cut_rate: f32,
    pub(crate) phys_atk_rate: f32,
    pub(crate) magic_atk_rate: f32,
    pub(crate) fire_atk_rate: f32,
    pub(crate) thunder_atk_rate: f32,
    pub(crate) phys_atk_power_rate: f32,
    pub(crate) magic_atk_power_rate: f32,
    pub(crate) fire_atk_power_rate: f32,
    pub(crate) thunder_atk_power_rate: f32,
    pub(crate) phys_atk_power: i32,
    pub(crate) magic_atk_power: i32,
    pub(crate) fire_atk_power: i32,
    pub(crate) thunder_atk_power: i32,
    pub(crate) phys_def_rate: f32,
    pub(crate) magic_def_rate: f32,
    pub(crate) fire_def_rate: f32,
    pub(crate) thunder_def_rate: f32,
    pub(crate) phys_def: i32,
    pub(crate) magic_def: i32,
    pub(crate) fire_def: i32,
    pub(crate) thunder_def: i32,
    pub(crate) no_guard_damage_rate: f32,
    pub(crate) vital_spot_change_rate: f32,
    pub(crate) normal_spot_change_rate: f32,
    pub(crate) max_hp_change_rate: f32,
    pub(crate) behavior_id: i32,
    pub(crate) change_hp_rate: f32,
    pub(crate) change_hp_point: i32,
    pub(crate) change_mp_rate: f32,
    pub(crate) change_mp_point: i32,
    pub(crate) mp_recover_change_speed: i32,
    pub(crate) change_stamina_rate: f32,
    pub(crate) change_stamina_point: i32,
    pub(crate) stamina_recover_change_speed: i32,
    pub(crate) magic_effect_time_change: f32,
    pub(crate) inside_durability: i32,
    pub(crate) max_durability: i32,
    pub(crate) stamina_attack_rate: f32,
    pub(crate) regist_poison: i32,
    pub(crate) regist_toxic: i32,
    pub(crate) regist_blood: i32,
    pub(crate) regist_curse: i32,
    pub(crate) fall_damage_rate: f32,
    pub(crate) soul_rate: f32,
    pub(crate) equip_weight_change_rate: f32,
    pub(crate) all_item_weight_change_rate: f32,
    pub(crate) soul: i32,
    pub(crate) anim_id_offset: i32,
    pub(crate) have_soul_rate: f32,
    pub(crate) target_priority: f32,
    pub(crate) sight_search_enemy_cut: i32,
    pub(crate) hearing_search_enemy_cut: f32,
    pub(crate) gravity_rate: f32,
    pub(crate) regist_poison_change_rate: f32,
    pub(crate) regist_toxic_change_rate: f32,
    pub(crate) regist_blood_change_rate: f32,
    pub(crate) regist_curse_change_rate: f32,
    pub(crate) soul_steal_rate: f32,
    pub(crate) life_reduction_rate: f32,
    pub(crate) hp_recover_rate: f32,
    pub(crate) replace_sp_effect_id: i32,
    pub(crate) cycle_occurence_sp_effect_id: i32,
    pub(crate) atk_occurence_sp_effect_id: i32,
    pub(crate) guard_def_flick_power_rate: f32,
    pub(crate) guard_stamina_cut_rate: f32,
    pub(crate) ray_cast_passed_time: i16,
    pub(crate) change_super_armor_point: i16,
    pub(crate) bow_dist_rate: i16,
    pub(crate) sp_category: i16,
    pub(crate) category_priority: i8,
    pub(crate) save_category: i8,
    pub(crate) change_magic_slot: u8,
    pub(crate) change_miracle_slot: u8,
    pub(crate) hero_point_damage: u8,
    pub(crate) def_flick_power: u8,
    pub(crate) flick_damage_cut_rate: u8,
    pub(crate) blood_damage_rate: u8,
    pub(crate) dmg_lv_none: u8,
    pub(crate) dmg_lv_s: u8,
    pub(crate) dmg_lv_m: u8,
    pub(crate) dmg_lv_l: u8,
    pub(crate) dmg_lv_blow_m: u8,
    pub(crate) dmg_lv_push: u8,
    pub(crate) dmg_lv_strike: u8,
    pub(crate) dmg_lv_blow_s: u8,
    pub(crate) dmg_lv_min: u8,
    pub(crate) dmg_lv_uppercut: u8,
    pub(crate) dmg_lv_blow_ll: u8,
    pub(crate) dmg_lv_breath: u8,
    pub(crate) atk_attribute: u8,
    pub(crate) sp_attribute: u8,
    pub(crate) state_info: i16,
    pub(crate) wep_param_change: u8,
    pub(crate) move_type: u8,
    pub(crate) life_reduction_type: i16,
    pub(crate) throw_condition: u8,
    pub(crate) add_behavior_judge_id_condition: i8,
    pub(crate) add_behavior_judge_id_add: i8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) bitfield2: u8,
    pub(crate) bitfield3: u8,
    pub(crate) bitfield4: u8,
    pub(crate) bitfield5: u8,
    pub(crate) bitfield6: u8,
    pub(crate) bitfield7: u8,
    pub(crate) damage_change_state: u8,
    pub(crate) unk1: f32,
    pub(crate) bitfield8: u8,
    pub(crate) unk7: i8,
    pub(crate) unk8: i16,
    pub(crate) effect_vfx0: i32,
    pub(crate) state_sp_effect0: i32,
    pub(crate) state_sp_effect1: i32,
    pub(crate) state_sp_effect2: i32,
    pub(crate) state_sp_effect3: i32,
    pub(crate) stability_val: i32,
    pub(crate) unk9: i32,
    pub(crate) unk10: i16,
    pub(crate) unk11: i16,
    pub(crate) effect_vfx1: i32,
    pub(crate) effect_vfx2: i32,
    pub(crate) effect_vfx3: i32,
    pub(crate) effect_vfx4: i32,
    pub(crate) effect_vfx5: i32,
    pub(crate) effect_vfx6: i32,
    pub(crate) effect_vfx7: i32,
    pub(crate) regist_frost: i32,
    pub(crate) unk12: i32,
    pub(crate) unk13: i16,
    pub(crate) unk14: u8,
    pub(crate) unk15: u8,
    pub(crate) poise_rate: f32,
    pub(crate) phys_rate: f32,
    pub(crate) magic_rate: f32,
    pub(crate) fire_rate: f32,
    pub(crate) thunder_rate: f32,
    pub(crate) dark_rate: f32,
    pub(crate) stamina_rate: f32,
    pub(crate) dark_damage_cut_rate: f32,
    pub(crate) dark_def_rate: f32,
    pub(crate) unk16: f32,
    pub(crate) unk17: f32,
    pub(crate) dark_attack_power_rate: f32,
    pub(crate) dark_atk_power: i32,
    pub(crate) unk18: f32,
    pub(crate) unk19: i32,
    pub(crate) condition_max_hp: f32,
    pub(crate) unk20: f32,
    pub(crate) drop_rate: f32,
    pub(crate) unk21: f32,
    pub(crate) poison_resist_change: f32,
    pub(crate) toxic_resist_change: f32,
    pub(crate) bleed_resist_change: f32,
    pub(crate) curse_resist_change: f32,
    pub(crate) frost_resist_change: f32,
    pub(crate) unk_damage_rate0: f32,
    pub(crate) unk_damage_rate1: f32,
    pub(crate) unk_damage_rate2: f32,
    pub(crate) unk_damage_rate3: f32,
    pub(crate) poison_atk_rate: f32,
    pub(crate) toxic_atk_rate: f32,
    pub(crate) bleed_atk_rate: f32,
    pub(crate) toxic_atk_rate_0: f32,
    pub(crate) unk22: f32,
    pub(crate) slash_sp_attack_rate: f32,
    pub(crate) blow_sp_attack_rate: f32,
    pub(crate) thrust_sp_attack_rate: f32,
    pub(crate) phys_sp_attack_rate: f32,
    pub(crate) magic_sp_attack_rate: f32,
    pub(crate) fire_sp_attack_rate: f32,
    pub(crate) thunder_sp_attack_rate: f32,
    pub(crate) dark_sp_attack_rate: f32,
    pub(crate) player_base_val0: u8,
    pub(crate) player_base_val1: u8,
    pub(crate) player_base_val2: u8,
    pub(crate) player_base_val3: u8,
    pub(crate) unk23: u8,
    pub(crate) bitfield9: u8,
    pub(crate) unk28: u8,
    pub(crate) unk29: u8,
    pub(crate) unk30: f32,
    pub(crate) phys_def_cut_rate_mp: f32,
    pub(crate) magic_def_cut_rate_mp: f32,
    pub(crate) fire_def_cut_rate_mp: f32,
    pub(crate) thunder_def_cut_rate_mp: f32,
    pub(crate) dark_def_cut_rate_mp: f32,
    pub(crate) phys_def_cut_rate2: f32,
    pub(crate) magic_def_cut_rate2: f32,
    pub(crate) fire_def_cut_rate2: f32,
    pub(crate) thunder_def_cut_rate2: f32,
    pub(crate) dark_def_cut_rate2: f32,
    pub(crate) unk36: f32,
    pub(crate) phys_unk_rate: f32,
    pub(crate) magic_unk_rate: f32,
    pub(crate) fire_unk_rate: f32,
    pub(crate) thunder_unk_rate: f32,
    pub(crate) dark_unk_rate: f32,
    pub(crate) phys_damage_rate2: f32,
    pub(crate) magic_damage_rate2: f32,
    pub(crate) fire_damage_rate2: f32,
    pub(crate) thunder_damage_rate2: f32,
    pub(crate) dark_damage_rate2: f32,
    pub(crate) regist_frost_change_rate: f32,
    pub(crate) condition_hp_change0: i16,
    pub(crate) condition_hp_change1: i16,
    pub(crate) condition_hp_change2: i16,
    pub(crate) unk37: u8,
    pub(crate) fake_dexterity_cast_speed: u8,
    pub(crate) unk38: f32,
    pub(crate) sword_art_mp_change_rate: f32,
    pub(crate) magic_mp_change_rate: f32,
    pub(crate) pyro_mp_change_rate: f32,
    pub(crate) miracle_mp_change_rate: f32,
    pub(crate) sub_change_hp_rate: f32,
    pub(crate) sub_hp_point: i32,
    pub(crate) sub_change_mp_rate: f32,
    pub(crate) sub_mp_point: i32,
    pub(crate) sub_hp_restore_rate: f32,
    pub(crate) sub_mp_restore_rate: f32,
    pub(crate) death_occurence_sp_effect_id: i32,
    pub(crate) effect_endurance_rate: f32,
    pub(crate) unk39: f32,
    pub(crate) unk40: u8,
    pub(crate) unk41: u8,
    pub(crate) sub_sp_category: i16,
    pub(crate) unk42: f32,
    pub(crate) vigor: i8,
    pub(crate) attunement: i8,
    pub(crate) endurance: i8,
    pub(crate) vitality: i8,
    pub(crate) strength: i8,
    pub(crate) dexterity: i8,
    pub(crate) intelligence: i8,
    pub(crate) faith: i8,
    pub(crate) luck: i8,
    pub(crate) human_point: i8,
    pub(crate) pad: [u8; 14],
}

impl SpEffectParam { 
    #[allow(unused)]
    pub(crate) fn set_effect_target_self(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_self(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_friend(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_friend(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_enemy(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_enemy(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_player(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_player(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_ai(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_ai(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_live(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_live(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_ghost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_ghost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_white_ghost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_white_ghost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_black_ghost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_black_ghost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_attacker(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_attacker(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disp_icon_nonactive(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disp_icon_nonactive(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_sp_effect_effect(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_sp_effect_effect(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_b_adjust_magic_ability(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn b_adjust_magic_ability(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_b_adjust_faith_ability(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn b_adjust_faith_ability(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_b_game_clear_bonus(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn b_game_clear_bonus(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_mag_param_change(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn mag_param_change(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_miracle_param_change(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn miracle_param_change(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_clear_soul(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn clear_soul(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_sos(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_sos(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_black_sos(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_black_sos(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_force_join_black_sos(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_force_join_black_sos(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_kick_session(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_kick_session(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_leave_session(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_leave_session(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_npc_invade(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield2;
        self.bitfield2 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_npc_invade(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield2 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_no_dead(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn no_dead(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_b_curr_hpindepende_max_hp(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn b_curr_hpindepende_max_hp(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_corrosion_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn corrosion_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_sight_search_cut_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn sight_search_cut_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_hearing_search_cut_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn hearing_search_cut_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_anti_magic_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn anti_magic_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_fake_target_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn fake_target_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_fake_target_ignore_undead(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield3;
        self.bitfield3 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn fake_target_ignore_undead(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield3 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_fake_target_ignore_animal(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn fake_target_ignore_animal(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_gravity_ignore(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn gravity_ignore(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_poison(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_poison(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_toxic(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_toxic(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_blood(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_blood(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_curse(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_curse(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_charm(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_charm(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_life_time(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield4;
        self.bitfield4 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_life_time(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield4 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_has_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn has_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_fire_damage_cancel(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_fire_damage_cancel(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_extend_sp_effect_life(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_extend_sp_effect_life(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_request_leave_coliseum_session(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn request_leave_coliseum_session(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_frost(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_frost(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_charge_attack_param_change(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn charge_attack_param_change(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_throw_attack_param_change(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn throw_attack_param_change(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_enable_equip_slot_check(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield5;
        self.bitfield5 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn enable_equip_slot_check(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield5 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type0(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type0(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type7(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield6;
        self.bitfield6 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type7(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield6 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type8(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type8(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type9(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type9(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type10(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type10(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type11(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type11(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type12(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type12(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type13(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type13(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type14(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type14(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_vow_type15(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield7;
        self.bitfield7 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn vow_type15(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield7 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_oppose_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_oppose_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_friendly_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_friendly_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_effect_target_self_target(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn effect_target_self_target(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk6(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield8;
        self.bitfield8 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk6(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield8 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk24(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk24(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_sp_val_correction(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_sp_val_correction(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_different_val(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_different_val(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_insta_death(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn insta_death(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk25(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk25(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk26(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk26(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk27(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk27(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_disable_multi_play_use(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield9;
        self.bitfield9 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn disable_multi_play_use(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield9 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SpEffectVfxParam {
    pub(crate) midst_sfx_id: i32,
    pub(crate) midst_se_id: i32,
    pub(crate) init_sfx_id: i32,
    pub(crate) init_se_id: i32,
    pub(crate) finish_sfx_id: i32,
    pub(crate) finish_se_id: i32,
    pub(crate) camouflage_begin_dist: f32,
    pub(crate) camouflage_end_dist: f32,
    pub(crate) transform_protector_id: i32,
    pub(crate) midst_dmy_id: i16,
    pub(crate) init_dmy_id: i16,
    pub(crate) finish_dmy_id: i16,
    pub(crate) effect_type: u8,
    pub(crate) soul_param_id_for_wep_enchant1: u8,
    pub(crate) play_category: u8,
    pub(crate) play_priority: u8,
    pub(crate) bitfield0: u8,
    pub(crate) bitfield1: u8,
    pub(crate) decal_id0: i32,
    pub(crate) decal_id1: i32,
    pub(crate) soul_param_id_for_wep_enchant2: u8,
    pub(crate) sp_effect_sp0: u8,
    pub(crate) sp_effect_sp1: u8,
    pub(crate) body_protector_type: u8,
    pub(crate) sp_sfx_id0: i32,
    pub(crate) sp_sfx_id1: i32,
    pub(crate) sp_sfx_id2: i32,
    pub(crate) sp_sfx_id3: i32,
    pub(crate) sp_sfx_id4: i32,
    pub(crate) sp_sfx_id5: i32,
    pub(crate) sp_sfx_id6: i32,
    pub(crate) sp_sfx_id7: i32,
    pub(crate) sp_sfx_id8: i32,
    pub(crate) sp_sfx_id9: i32,
    pub(crate) sp_sfx_id10: i32,
    pub(crate) sp_sfx_id11: i32,
    pub(crate) sp_sfx_id12: i32,
    pub(crate) sp_sfx_id13: i32,
    pub(crate) sp_sfx_id14: i32,
    pub(crate) sp_sfx_id15: i32,
    pub(crate) unk10: u8,
    pub(crate) phantom_type: u8,
    pub(crate) camouflage_ghost: u8,
    pub(crate) unk12: u8,
    pub(crate) phantom_param_id: i32,
    pub(crate) emissive_intensity_scale: f32,
    pub(crate) unk13: f32,
    pub(crate) body_protector_val: i16,
    pub(crate) unk14: i16,
    pub(crate) cinder_intensity_scale: f32,
    pub(crate) pad1: [u8; 8],
}

impl SpEffectVfxParam { 
    #[allow(unused)]
    pub(crate) fn set_exist_effect_for_large(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn exist_effect_for_large(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_exist_effect_for_soul(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn exist_effect_for_soul(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_exist_invisible_at_camouflage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn exist_invisible_at_camouflage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_use_camouflage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn use_camouflage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_invisible_at_friend_camouflage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn invisible_at_friend_camouflage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_add_map_area_block_offset(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn add_map_area_block_offset(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_half_camouflage(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn half_camouflage(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_full_body_protector_id(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_full_body_protector_id(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_invisible_weapon(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_invisible_weapon(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_silence(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_silence(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_gauntlet_protector_id(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_gauntlet_protector_id(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield1;
        self.bitfield1 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield1 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct SwordArtsParam {
    pub(crate) action_id: u8,
    pub(crate) action_correction: u8,
    pub(crate) reserve_art_point_type: u8,
    pub(crate) unused_field: u8,
    pub(crate) reserve_arts_point0: i8,
    pub(crate) reserve_arts_point1: i8,
    pub(crate) reserve_arts_point2: i8,
    pub(crate) reserve_arts_point3: i8,
    pub(crate) debug_text_id: i32,
    pub(crate) sub_fpcost: i16,
    pub(crate) f_pcost: i16,
    pub(crate) f_pcost_light: i16,
    pub(crate) fp_cost_strong: i16,
    pub(crate) shield_category: u8,
    pub(crate) pad1: [u8; 11],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct TalkParam {
    pub(crate) pc_gender_female1: i32,
    pub(crate) pc_gender_male1: i32,
    pub(crate) sp_effect_id0: i32,
    pub(crate) animation_id0: i32,
    pub(crate) sp_effect_id1: i32,
    pub(crate) animation_id1: i32,
    pub(crate) sp_effect_id2: i32,
    pub(crate) animation_id2: i32,
    pub(crate) event_flag_id: i32,
    pub(crate) event_id_female: i32,
    pub(crate) event_id_male: i32,
    pub(crate) talk_time: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ThrowDirectionSfxParam {
    pub(crate) sfx_id1: i32,
    pub(crate) sfx_id2: i32,
    pub(crate) sfx_id3: i32,
    pub(crate) sfx_id4: i32,
    pub(crate) sfx_id5: i32,
    pub(crate) sfx_id6: i32,
    pub(crate) sfx_id7: i32,
    pub(crate) sfx_id8: i32,
    pub(crate) sfx_id9: i32,
    pub(crate) sfx_id10: i32,
    pub(crate) sfx_id11: i32,
    pub(crate) sfx_id12: i32,
    pub(crate) sfx_id13: i32,
    pub(crate) sfx_id14: i32,
    pub(crate) sfx_id15: i32,
    pub(crate) sfx_id16: i32,
    pub(crate) sfx_id17: i32,
    pub(crate) sfx_id18: i32,
    pub(crate) sfx_id19: i32,
    pub(crate) sfx_id20: i32,
    pub(crate) sfx_id21: i32,
    pub(crate) sfx_id22: i32,
    pub(crate) sfx_id23: i32,
    pub(crate) sfx_id24: i32,
    pub(crate) sfx_id25: i32,
    pub(crate) sfx_id26: i32,
    pub(crate) sfx_id27: i32,
    pub(crate) sfx_id28: i32,
    pub(crate) sfx_id29: i32,
    pub(crate) sfx_id30: i32,
    pub(crate) pad1: [u8; 24],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ThrowParam {
    pub(crate) atk_chr_id: i32,
    pub(crate) def_chr_id: i32,
    pub(crate) dist: f32,
    pub(crate) diff_ang_min: f32,
    pub(crate) diff_ang_max: f32,
    pub(crate) upper_yrange: f32,
    pub(crate) lower_yrange: f32,
    pub(crate) diff_ang_my_to_def: f32,
    pub(crate) throw_type_id: i32,
    pub(crate) atk_animid: i32,
    pub(crate) def_anim_id: i32,
    pub(crate) esc_hp: i16,
    pub(crate) self_esc_cycle_time: i16,
    pub(crate) sphere_cast_radius_rate_top: i16,
    pub(crate) sphere_cast_radius_rate_low: i16,
    pub(crate) pad_type: u8,
    pub(crate) atk_enable_state: u8,
    pub(crate) atk_sorb_dmy_id: u8,
    pub(crate) def_sorb_dmy_id: u8,
    pub(crate) throw_type: u8,
    pub(crate) self_esc_cycle_int: u8,
    pub(crate) dmy_has_chr_dir_type: u8,
    pub(crate) bitfield0: u8,
    pub(crate) sub_atk_sorb_dmy_id: i16,
    pub(crate) sub_def_sorb_dmy_id: i16,
    pub(crate) dist2: f32,
    pub(crate) diff_ang_min2: f32,
    pub(crate) diff_ang_max2: f32,
    pub(crate) upper_yrange2: f32,
    pub(crate) lower_yrange2: f32,
    pub(crate) diff_ang_my_to_def2: f32,
    pub(crate) perform_dmy_id0: i32,
    pub(crate) perform_dmy_id1: i32,
    pub(crate) pad1: [u8; 32],
}

impl ThrowParam { 
    #[allow(unused)]
    pub(crate) fn set_is_turn_atker(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 0;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_turn_atker(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 0;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_skip_wep_cate(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 1;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_skip_wep_cate(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 1;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_is_skip_sphere_cast(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 2;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn is_skip_sphere_cast(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 2;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk1(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 3;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk1(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 3;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk2(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 4;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk2(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 4;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk3(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 5;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk3(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 5;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk4(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 6;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk4(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 6;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

    #[allow(unused)]
    pub(crate) fn set_unk5(&mut self, state: bool) {
        const FIELD_INDEX: u8 = 1 << 7;
        let val = self.bitfield0;
        self.bitfield0 = if state {
            val | FIELD_INDEX
        } else {
            val & !FIELD_INDEX
        };
    }

    #[allow(unused)]
    pub(crate) fn unk5(&mut self) -> bool {
        const FIELD_INDEX: u8 = 1 << 7;
        (self.bitfield0 & FIELD_INDEX) != 0
    }

}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct ToughnessParam {
    pub(crate) toughness: f32,
    pub(crate) damage_lvl_threshold: i16,
    pub(crate) is_toughness_effective: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) sp_effect_id: i32,
    pub(crate) pad2: [u8; 20],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct UpperArmParam {
    pub(crate) arm_z0: f32,
    pub(crate) arm_xy0: f32,
    pub(crate) arm_z1: f32,
    pub(crate) arm_xy1: f32,
    pub(crate) arm_z2: f32,
    pub(crate) arm_xy2: f32,
    pub(crate) arm_z3: f32,
    pub(crate) arm_xy3: f32,
    pub(crate) arm_z4: f32,
    pub(crate) arm_xy4: f32,
    pub(crate) arm_z5: f32,
    pub(crate) arm_xy5: f32,
    pub(crate) arm_z6: f32,
    pub(crate) arm_xy6: f32,
    pub(crate) arm_z7: f32,
    pub(crate) arm_xy7: f32,
    pub(crate) arm_z8: f32,
    pub(crate) arm_xy8: f32,
    pub(crate) arm_z9: f32,
    pub(crate) arm_xy9: f32,
    pub(crate) pad1: [u8; 48],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct WeaponGenParam {
    pub(crate) wep_param_id: i32,
    pub(crate) gem_slot_type_0: i32,
    pub(crate) gem_gen_id0: i32,
    pub(crate) gem_slot_type_1: i32,
    pub(crate) gem_gen_id1: i32,
    pub(crate) gem_slot_type_2: i32,
    pub(crate) gem_gen_id2: i32,
    pub(crate) gem_slot_type_3: i32,
    pub(crate) gem_gen_id3: i32,
    pub(crate) gem_slot_type_4: i32,
    pub(crate) gem_gen_id4: i32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct WepAbsorpPosParam {
    pub(crate) shealth_time: u8,
    pub(crate) pad1: [u8; 3],
    pub(crate) one_hand_damipoly_id0: u16,
    pub(crate) one_hand_damipoly_id1: u16,
    pub(crate) both_hand_damipoly_id0: u16,
    pub(crate) shealth_damipoly_id0: u16,
    pub(crate) shealth_damipoly_id1: u16,
    pub(crate) one_hand_damipoly_id2: u16,
    pub(crate) one_hand_damipoly_id3: u16,
    pub(crate) both_hand_damipoly_id1: u16,
    pub(crate) shealth_damipoly_id2: u16,
    pub(crate) shealth_damipoly_id3: u16,
    pub(crate) one_hand_damipoly_id4: u16,
    pub(crate) one_hand_damipoly_id5: u16,
    pub(crate) both_hand_damipoly_id2: u16,
    pub(crate) shealth_damipoly_id4: u16,
    pub(crate) shealth_damipoly_id5: u16,
    pub(crate) one_hand_damipoly_id6: u16,
    pub(crate) one_hand_damipoly_id7: u16,
    pub(crate) both_hand_damipoly_id3: u16,
    pub(crate) shealth_damipoly_id6: u16,
    pub(crate) shealth_damipoly_id7: u16,
    pub(crate) unk22: u8,
    pub(crate) unk23: u8,
    pub(crate) unk24: u8,
    pub(crate) unk25: u8,
    pub(crate) both_hand_damipoly_id4: i16,
    pub(crate) both_hand_damipoly_id5: i16,
    pub(crate) both_hand_damipoly_id6: i16,
    pub(crate) both_hand_damipoly_id7: i16,
    pub(crate) unk30: u8,
    pub(crate) unk31: u8,
    pub(crate) unk32: u8,
    pub(crate) unk33: u8,
    pub(crate) unk34: u8,
    pub(crate) unk35: u8,
    pub(crate) unk36: u8,
    pub(crate) unk37: u8,
    pub(crate) unk38: u8,
    pub(crate) unk39: u8,
    pub(crate) unk40: u8,
    pub(crate) unk41: u8,
    pub(crate) unk42: u8,
    pub(crate) unk43: u8,
    pub(crate) unk44: u8,
    pub(crate) unk45: u8,
    pub(crate) unk46: u8,
    pub(crate) unk47: u8,
    pub(crate) unk48: u8,
    pub(crate) unk49: u8,
    pub(crate) unk50: u8,
    pub(crate) unk51: u8,
    pub(crate) unk52: u8,
    pub(crate) unk53: u8,
    pub(crate) pad2: [u8; 16],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct WetAspectParam {
    pub(crate) r1: u8,
    pub(crate) g1: u8,
    pub(crate) b1: u8,
    pub(crate) pad1: [u8; 1],
    pub(crate) alpha1: f32,
    pub(crate) r2: u8,
    pub(crate) g2: u8,
    pub(crate) b2: u8,
    pub(crate) pad2: [u8; 1],
    pub(crate) alpha2: f32,
    pub(crate) wet_rate: f32,
    pub(crate) wet_correction: u8,
    pub(crate) pad3: [u8; 11],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct WhiteSignCoolTimeParam {
    pub(crate) time_limit0: f32,
    pub(crate) time_limit1: f32,
    pub(crate) time_limit2: f32,
    pub(crate) time_limit3: f32,
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct Wind {
    pub(crate) common_capsule_begin_dmy_id: i16,
    pub(crate) common_capsule_end_dmy_id: i16,
    pub(crate) common_capsule_radius: f32,
    pub(crate) pad1: [u8; 120],
    pub(crate) pad2: [u8; 3],
    pub(crate) sfx_dir_pitch_min: f32,
    pub(crate) sfx_dir_pitch_max: f32,
    pub(crate) sfx_dir_yaw_min: f32,
    pub(crate) sfx_dir_yaw_max: f32,
    pub(crate) sfx_cycle_min: f32,
    pub(crate) sfx_cycle_max: f32,
    pub(crate) sfx_speed_min: f32,
    pub(crate) sfx_speed_max: f32,
    pub(crate) sfx_maximum_drag: f32,
    pub(crate) pad3: [u8; 88],
    pub(crate) pad4: [u8; 3],
    pub(crate) cloth_dir_pitch_min: f32,
    pub(crate) cloth_dir_pitch_max: f32,
    pub(crate) cloth_dir_yaw_min: f32,
    pub(crate) cloth_dir_yaw_max: f32,
    pub(crate) cloth_cycle_min: f32,
    pub(crate) cloth_cycle_max: f32,
    pub(crate) cloth_speed_min: f32,
    pub(crate) cloth_speed_max: f32,
    pub(crate) cloth_maximum_drag: f32,
    pub(crate) pad5: [u8; 88],
}

impl Params {
    #[allow(unused)]
    pub(crate) unsafe fn get_action_button_param(&self) -> Option<impl Iterator<Item = Param<ActionButtonParam>>> {
        self.iter_param::<ActionButtonParam>("ActionButtonParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_ai_sound_param(&self) -> Option<impl Iterator<Item = Param<AiSoundParam>>> {
        self.iter_param::<AiSoundParam>("AiSoundParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_atk_param_npc(&self) -> Option<impl Iterator<Item = Param<AtkParam_Npc>>> {
        self.iter_param::<AtkParam_Npc>("AtkParam_Npc")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_atk_param_pc(&self) -> Option<impl Iterator<Item = Param<AtkParam_Pc>>> {
        self.iter_param::<AtkParam_Pc>("AtkParam_Pc")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_attack_element_correct_param(&self) -> Option<impl Iterator<Item = Param<AttackElementCorrectParam>>> {
        self.iter_param::<AttackElementCorrectParam>("AttackElementCorrectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_behavior_param(&self) -> Option<impl Iterator<Item = Param<BehaviorParam>>> {
        self.iter_param::<BehaviorParam>("BehaviorParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_behavior_param_pc(&self) -> Option<impl Iterator<Item = Param<BehaviorParam_PC>>> {
        self.iter_param::<BehaviorParam_PC>("BehaviorParam_PC")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_bonfire_warp_param(&self) -> Option<impl Iterator<Item = Param<BonfireWarpParam>>> {
        self.iter_param::<BonfireWarpParam>("BonfireWarpParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_budget_param(&self) -> Option<impl Iterator<Item = Param<BudgetParam>>> {
        self.iter_param::<BudgetParam>("BudgetParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_bullet(&self) -> Option<impl Iterator<Item = Param<Bullet>>> {
        self.iter_param::<Bullet>("Bullet")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_bullet_create_limit_param(&self) -> Option<impl Iterator<Item = Param<BulletCreateLimitParam>>> {
        self.iter_param::<BulletCreateLimitParam>("BulletCreateLimitParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_calc_correct_graph(&self) -> Option<impl Iterator<Item = Param<CalcCorrectGraph>>> {
        self.iter_param::<CalcCorrectGraph>("CalcCorrectGraph")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_ceremony(&self) -> Option<impl Iterator<Item = Param<Ceremony>>> {
        self.iter_param::<Ceremony>("Ceremony")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_character_load_param(&self) -> Option<impl Iterator<Item = Param<CharacterLoadParam>>> {
        self.iter_param::<CharacterLoadParam>("CharacterLoadParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_chara_init_param(&self) -> Option<impl Iterator<Item = Param<CharaInitParam>>> {
        self.iter_param::<CharaInitParam>("CharaInitParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_char_make_menu_list_item_param(&self) -> Option<impl Iterator<Item = Param<CharMakeMenuListItemParam>>> {
        self.iter_param::<CharMakeMenuListItemParam>("CharMakeMenuListItemParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_char_make_menu_top_param(&self) -> Option<impl Iterator<Item = Param<CharMakeMenuTopParam>>> {
        self.iter_param::<CharMakeMenuTopParam>("CharMakeMenuTopParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_clear_count_correct_param(&self) -> Option<impl Iterator<Item = Param<ClearCountCorrectParam>>> {
        self.iter_param::<ClearCountCorrectParam>("ClearCountCorrectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_cool_time_param(&self) -> Option<impl Iterator<Item = Param<CoolTimeParam>>> {
        self.iter_param::<CoolTimeParam>("CoolTimeParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_cult_setting_param(&self) -> Option<impl Iterator<Item = Param<CultSettingParam>>> {
        self.iter_param::<CultSettingParam>("CultSettingParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_decal_param(&self) -> Option<impl Iterator<Item = Param<DecalParam>>> {
        self.iter_param::<DecalParam>("DecalParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_direction_camera_param(&self) -> Option<impl Iterator<Item = Param<DirectionCameraParam>>> {
        self.iter_param::<DirectionCameraParam>("DirectionCameraParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_equip_mtrl_set_param(&self) -> Option<impl Iterator<Item = Param<EquipMtrlSetParam>>> {
        self.iter_param::<EquipMtrlSetParam>("EquipMtrlSetParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_equip_param_accessory(&self) -> Option<impl Iterator<Item = Param<EquipParamAccessory>>> {
        self.iter_param::<EquipParamAccessory>("EquipParamAccessory")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_equip_param_goods(&self) -> Option<impl Iterator<Item = Param<EquipParamGoods>>> {
        self.iter_param::<EquipParamGoods>("EquipParamGoods")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_equip_param_protector(&self) -> Option<impl Iterator<Item = Param<EquipParamProtector>>> {
        self.iter_param::<EquipParamProtector>("EquipParamProtector")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_equip_param_weapon(&self) -> Option<impl Iterator<Item = Param<EquipParamWeapon>>> {
        self.iter_param::<EquipParamWeapon>("EquipParamWeapon")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_face_gen_param(&self) -> Option<impl Iterator<Item = Param<FaceGenParam>>> {
        self.iter_param::<FaceGenParam>("FaceGenParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_face_param(&self) -> Option<impl Iterator<Item = Param<FaceParam>>> {
        self.iter_param::<FaceParam>("FaceParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_face_range_param(&self) -> Option<impl Iterator<Item = Param<FaceRangeParam>>> {
        self.iter_param::<FaceRangeParam>("FaceRangeParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_foot_sfx_param(&self) -> Option<impl Iterator<Item = Param<FootSfxParam>>> {
        self.iter_param::<FootSfxParam>("FootSfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_game_area_param(&self) -> Option<impl Iterator<Item = Param<GameAreaParam>>> {
        self.iter_param::<GameAreaParam>("GameAreaParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_game_progress_param(&self) -> Option<impl Iterator<Item = Param<GameProgressParam>>> {
        self.iter_param::<GameProgressParam>("GameProgressParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_gem_category_param(&self) -> Option<impl Iterator<Item = Param<GemCategoryParam>>> {
        self.iter_param::<GemCategoryParam>("GemCategoryParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_gem_drop_doping_param(&self) -> Option<impl Iterator<Item = Param<GemDropDopingParam>>> {
        self.iter_param::<GemDropDopingParam>("GemDropDopingParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_gem_drop_modify_param(&self) -> Option<impl Iterator<Item = Param<GemDropModifyParam>>> {
        self.iter_param::<GemDropModifyParam>("GemDropModifyParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_gemeffect_param(&self) -> Option<impl Iterator<Item = Param<GemeffectParam>>> {
        self.iter_param::<GemeffectParam>("GemeffectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_gem_gen_param(&self) -> Option<impl Iterator<Item = Param<GemGenParam>>> {
        self.iter_param::<GemGenParam>("GemGenParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_hit_effect_se_param(&self) -> Option<impl Iterator<Item = Param<HitEffectSeParam>>> {
        self.iter_param::<HitEffectSeParam>("HitEffectSeParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_hit_effect_sfx_concept_param(&self) -> Option<impl Iterator<Item = Param<HitEffectSfxConceptParam>>> {
        self.iter_param::<HitEffectSfxConceptParam>("HitEffectSfxConceptParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_hit_effect_sfx_param(&self) -> Option<impl Iterator<Item = Param<HitEffectSfxParam>>> {
        self.iter_param::<HitEffectSfxParam>("HitEffectSfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_hit_mtrl_param(&self) -> Option<impl Iterator<Item = Param<HitMtrlParam>>> {
        self.iter_param::<HitMtrlParam>("HitMtrlParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_h_pestus_flask_recovery_param(&self) -> Option<impl Iterator<Item = Param<HPEstusFlaskRecoveryParam>>> {
        self.iter_param::<HPEstusFlaskRecoveryParam>("HPEstusFlaskRecoveryParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_item_lot_param(&self) -> Option<impl Iterator<Item = Param<ItemLotParam>>> {
        self.iter_param::<ItemLotParam>("ItemLotParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_knock_back_param(&self) -> Option<impl Iterator<Item = Param<KnockBackParam>>> {
        self.iter_param::<KnockBackParam>("KnockBackParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_knowledge_load_screen_item_param(&self) -> Option<impl Iterator<Item = Param<KnowledgeLoadScreenItemParam>>> {
        self.iter_param::<KnowledgeLoadScreenItemParam>("KnowledgeLoadScreenItemParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_load_balancer_draw_dist_scale_param(&self) -> Option<impl Iterator<Item = Param<LoadBalancerDrawDistScaleParam>>> {
        self.iter_param::<LoadBalancerDrawDistScaleParam>("LoadBalancerDrawDistScaleParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_load_balancer_param(&self) -> Option<impl Iterator<Item = Param<LoadBalancerParam>>> {
        self.iter_param::<LoadBalancerParam>("LoadBalancerParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_lock_cam_param(&self) -> Option<impl Iterator<Item = Param<LockCamParam>>> {
        self.iter_param::<LockCamParam>("LockCamParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_lod_param(&self) -> Option<impl Iterator<Item = Param<LodParam>>> {
        self.iter_param::<LodParam>("LodParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_lod_param_ps4(&self) -> Option<impl Iterator<Item = Param<LodParam_ps4>>> {
        self.iter_param::<LodParam_ps4>("LodParam_ps4")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_lod_param_xb1(&self) -> Option<impl Iterator<Item = Param<LodParam_xb1>>> {
        self.iter_param::<LodParam_xb1>("LodParam_xb1")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_magic(&self) -> Option<impl Iterator<Item = Param<Magic>>> {
        self.iter_param::<Magic>("Magic")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_map_mimicry_establishment_param(&self) -> Option<impl Iterator<Item = Param<MapMimicryEstablishmentParam>>> {
        self.iter_param::<MapMimicryEstablishmentParam>("MapMimicryEstablishmentParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_menu_offscr_rend_param(&self) -> Option<impl Iterator<Item = Param<MenuOffscrRendParam>>> {
        self.iter_param::<MenuOffscrRendParam>("MenuOffscrRendParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_menu_property_layout_param(&self) -> Option<impl Iterator<Item = Param<MenuPropertyLayoutParam>>> {
        self.iter_param::<MenuPropertyLayoutParam>("MenuPropertyLayoutParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_menu_property_spec_param(&self) -> Option<impl Iterator<Item = Param<MenuPropertySpecParam>>> {
        self.iter_param::<MenuPropertySpecParam>("MenuPropertySpecParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_menu_value_table_param(&self) -> Option<impl Iterator<Item = Param<MenuValueTableParam>>> {
        self.iter_param::<MenuValueTableParam>("MenuValueTableParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_model_sfx_param(&self) -> Option<impl Iterator<Item = Param<ModelSfxParam>>> {
        self.iter_param::<ModelSfxParam>("ModelSfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_move_param(&self) -> Option<impl Iterator<Item = Param<MoveParam>>> {
        self.iter_param::<MoveParam>("MoveParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_m_pestus_flask_recovery_param(&self) -> Option<impl Iterator<Item = Param<MPEstusFlaskRecoveryParam>>> {
        self.iter_param::<MPEstusFlaskRecoveryParam>("MPEstusFlaskRecoveryParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_multi_hpestus_flask_bonus_param(&self) -> Option<impl Iterator<Item = Param<MultiHPEstusFlaskBonusParam>>> {
        self.iter_param::<MultiHPEstusFlaskBonusParam>("MultiHPEstusFlaskBonusParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_multi_mpestus_flask_bonus_param(&self) -> Option<impl Iterator<Item = Param<MultiMPEstusFlaskBonusParam>>> {
        self.iter_param::<MultiMPEstusFlaskBonusParam>("MultiMPEstusFlaskBonusParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_multi_play_correction_param(&self) -> Option<impl Iterator<Item = Param<MultiPlayCorrectionParam>>> {
        self.iter_param::<MultiPlayCorrectionParam>("MultiPlayCorrectionParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_multi_soul_bonus_rate_param(&self) -> Option<impl Iterator<Item = Param<MultiSoulBonusRateParam>>> {
        self.iter_param::<MultiSoulBonusRateParam>("MultiSoulBonusRateParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_network_area_param(&self) -> Option<impl Iterator<Item = Param<NetworkAreaParam>>> {
        self.iter_param::<NetworkAreaParam>("NetworkAreaParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_network_msg_param(&self) -> Option<impl Iterator<Item = Param<NetworkMsgParam>>> {
        self.iter_param::<NetworkMsgParam>("NetworkMsgParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_network_param(&self) -> Option<impl Iterator<Item = Param<NetworkParam>>> {
        self.iter_param::<NetworkParam>("NetworkParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_new_menu_color_table_param(&self) -> Option<impl Iterator<Item = Param<NewMenuColorTableParam>>> {
        self.iter_param::<NewMenuColorTableParam>("NewMenuColorTableParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_npc_ai_action_param(&self) -> Option<impl Iterator<Item = Param<NpcAiActionParam>>> {
        self.iter_param::<NpcAiActionParam>("NpcAiActionParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_npc_param(&self) -> Option<impl Iterator<Item = Param<NpcParam>>> {
        self.iter_param::<NpcParam>("NpcParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_npc_think_param(&self) -> Option<impl Iterator<Item = Param<NpcThinkParam>>> {
        self.iter_param::<NpcThinkParam>("NpcThinkParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_obj_act_param(&self) -> Option<impl Iterator<Item = Param<ObjActParam>>> {
        self.iter_param::<ObjActParam>("ObjActParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_object_material_sfx_param(&self) -> Option<impl Iterator<Item = Param<ObjectMaterialSfxParam>>> {
        self.iter_param::<ObjectMaterialSfxParam>("ObjectMaterialSfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_object_param(&self) -> Option<impl Iterator<Item = Param<ObjectParam>>> {
        self.iter_param::<ObjectParam>("ObjectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_phantom_param(&self) -> Option<impl Iterator<Item = Param<PhantomParam>>> {
        self.iter_param::<PhantomParam>("PhantomParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_play_region_param(&self) -> Option<impl Iterator<Item = Param<PlayRegionParam>>> {
        self.iter_param::<PlayRegionParam>("PlayRegionParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_protector_gen_param(&self) -> Option<impl Iterator<Item = Param<ProtectorGenParam>>> {
        self.iter_param::<ProtectorGenParam>("ProtectorGenParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_ragdoll_param(&self) -> Option<impl Iterator<Item = Param<RagdollParam>>> {
        self.iter_param::<RagdollParam>("RagdollParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_reinforce_param_protector(&self) -> Option<impl Iterator<Item = Param<ReinforceParamProtector>>> {
        self.iter_param::<ReinforceParamProtector>("ReinforceParamProtector")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_reinforce_param_weapon(&self) -> Option<impl Iterator<Item = Param<ReinforceParamWeapon>>> {
        self.iter_param::<ReinforceParamWeapon>("ReinforceParamWeapon")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_role_param(&self) -> Option<impl Iterator<Item = Param<RoleParam>>> {
        self.iter_param::<RoleParam>("RoleParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_se_material_convert_param(&self) -> Option<impl Iterator<Item = Param<SeMaterialConvertParam>>> {
        self.iter_param::<SeMaterialConvertParam>("SeMaterialConvertParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_shop_lineup_param(&self) -> Option<impl Iterator<Item = Param<ShopLineupParam>>> {
        self.iter_param::<ShopLineupParam>("ShopLineupParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_skeleton_param(&self) -> Option<impl Iterator<Item = Param<SkeletonParam>>> {
        self.iter_param::<SkeletonParam>("SkeletonParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_sp_effect_param(&self) -> Option<impl Iterator<Item = Param<SpEffectParam>>> {
        self.iter_param::<SpEffectParam>("SpEffectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_sp_effect_vfx_param(&self) -> Option<impl Iterator<Item = Param<SpEffectVfxParam>>> {
        self.iter_param::<SpEffectVfxParam>("SpEffectVfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_sword_arts_param(&self) -> Option<impl Iterator<Item = Param<SwordArtsParam>>> {
        self.iter_param::<SwordArtsParam>("SwordArtsParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_talk_param(&self) -> Option<impl Iterator<Item = Param<TalkParam>>> {
        self.iter_param::<TalkParam>("TalkParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_throw_direction_sfx_param(&self) -> Option<impl Iterator<Item = Param<ThrowDirectionSfxParam>>> {
        self.iter_param::<ThrowDirectionSfxParam>("ThrowDirectionSfxParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_throw_param(&self) -> Option<impl Iterator<Item = Param<ThrowParam>>> {
        self.iter_param::<ThrowParam>("ThrowParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_toughness_param(&self) -> Option<impl Iterator<Item = Param<ToughnessParam>>> {
        self.iter_param::<ToughnessParam>("ToughnessParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_upper_arm_param(&self) -> Option<impl Iterator<Item = Param<UpperArmParam>>> {
        self.iter_param::<UpperArmParam>("UpperArmParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_weapon_gen_param(&self) -> Option<impl Iterator<Item = Param<WeaponGenParam>>> {
        self.iter_param::<WeaponGenParam>("WeaponGenParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_wep_absorp_pos_param(&self) -> Option<impl Iterator<Item = Param<WepAbsorpPosParam>>> {
        self.iter_param::<WepAbsorpPosParam>("WepAbsorpPosParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_wet_aspect_param(&self) -> Option<impl Iterator<Item = Param<WetAspectParam>>> {
        self.iter_param::<WetAspectParam>("WetAspectParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_white_sign_cool_time_param(&self) -> Option<impl Iterator<Item = Param<WhiteSignCoolTimeParam>>> {
        self.iter_param::<WhiteSignCoolTimeParam>("WhiteSignCoolTimeParam")
    }

    #[allow(unused)]
    pub(crate) unsafe fn get_wind(&self) -> Option<impl Iterator<Item = Param<Wind>>> {
        self.iter_param::<Wind>("Wind")
    }
}
