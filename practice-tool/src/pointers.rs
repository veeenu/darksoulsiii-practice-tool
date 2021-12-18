use crate::memedit::*;

pub(crate) struct PointerChains {
    pub(crate) all_no_damage: Bitflag<u8>,
    pub(crate) no_death: Bitflag<u8>,
    pub(crate) one_shot: Bitflag<u8>,
    pub(crate) inf_stamina: Bitflag<u8>,
    pub(crate) inf_focus: Bitflag<u8>,
    pub(crate) inf_consumables: Bitflag<u8>,
    pub(crate) deathcam: Bitflag<u8>,
    pub(crate) evt_draw: Bitflag<u8>,
    pub(crate) evt_disable: Bitflag<u8>,
    pub(crate) ai_disable: Bitflag<u8>,
    pub(crate) rend_chr: Bitflag<u8>,
    pub(crate) rend_obj: Bitflag<u8>,
    pub(crate) rend_map: Bitflag<u8>,
    pub(crate) rend_mesh_hi: Bitflag<u8>,
    pub(crate) rend_mesh_lo: Bitflag<u8>,
    pub(crate) all_draw_hit: Bitflag<u8>,
    pub(crate) ik_foot_ray: Bitflag<u8>,
    pub(crate) debug_sphere_1: Bitflag<u8>,
    pub(crate) debug_sphere_2: Bitflag<u8>,
    pub(crate) gravity: Bitflag<u8>,
    pub(crate) speed: PointerChain<f32>,
    pub(crate) position: (PointerChain<f32>, PointerChain<[f32; 3]>),
    pub(crate) souls: PointerChain<u32>,
    pub(crate) quitout: PointerChain<u8>,
    pub(crate) mouse_enable: PointerChain<u8>,

    #[allow(unused)]
    pub(crate) world_chr_man: usize,
    pub(crate) map_item_man: u64,
    pub(crate) spawn_item_func_ptr: u64,
}

pub(crate) struct BaseAddresses {
    // offsets from base_b
    pub world_chr_man: usize,
    pub sprj_debug_event: usize,
    pub debug: usize,
    pub grend: usize,
    pub xa: u32,

    // hitbox things
    pub world_chr_man_dbg: u64,
    pub base_hbd: u64,

    pub offs_deathcam: u64,
    pub offs_no_goods_consume: u64,
    pub offs_speed: u64,

    // offsets from debug
    pub offs_player_exterminate: u64,
    pub offs_all_no_damage: u64,
    pub offs_no_update_ai: u64,

    pub mesh_lo: u64,
    pub mesh_hi: u64,
    pub instaqo: u64,
    pub base_souls: u64,
    pub mouse_enable: (u64, u64),

    // other static pointers

    pub map_item_man: u64,
    pub spawn_item_func_ptr: u64,

    #[allow(unused)]
    pub version_string_ptr: usize,
    #[allow(unused)]
    pub version: &'static str,
    #[allow(unused)]
    pub format_string: u64,
    #[allow(unused)]
    pub base_d: usize,
}

const VER104: BaseAddresses = BaseAddresses {
    world_chr_man: 0x1446c5dc8,
    base_d: 0x1446a09f8,
    sprj_debug_event: 0x144697d88,
    debug: 0x1446c5eb8,
    grend: 0x1444bb000,
    xa: 0x1f70,
    world_chr_man_dbg: 0x1446C5EE8,
    base_hbd: 0x1446C3AD0,

    offs_deathcam: 0x88,           // deathcam
    offs_no_goods_consume: 0x1ECA, // no goods consume
    offs_speed: 0xA38,             // speed
    offs_player_exterminate: 0x1,  // player exterminate
    offs_all_no_damage: 0x9,       // all no damage
    offs_no_update_ai: 0xD,        // no update ai

    mesh_lo: 0x1446C3BBC,            // mesh (low hit)
    mesh_hi: 0x1446C3BBD,            // mesh (high hit)
    instaqo: 0x1446A9280,            // insta qo
    version_string_ptr: 0x14288C422, // version string
    base_souls: 0x144704268,         // souls base ptr
    mouse_enable: (0x1446A9280, 0x54),
    version: "1.04",
    format_string: 0x142952940,
    spawn_item_func_ptr: 0x1407abc00,
    map_item_man: 0x1446af280,
};

const VER108: BaseAddresses = BaseAddresses {
    world_chr_man: 0x14472CF58,
    base_d: 0x144707b58,
    sprj_debug_event: 0x1446fee88,
    debug: 0x14472d049,
    grend: 0x14451b608,
    xa: 0x1f80,
    world_chr_man_dbg: 0x14472D078,
    base_hbd: 0x14472AC60,

    offs_deathcam: 0x88,             // deathcam
    offs_no_goods_consume: 0x1EDA,   // no goods consume
    offs_speed: 0xA38,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 0x9,         // all no damage
    offs_no_update_ai: 0xD,          // no update ai
    mesh_lo: 0x14472AD4C,            // mesh (low hit)
    mesh_hi: 0x14472AD4D,            // mesh (high hit)
    instaqo: 0x1447103D8,            // insta qo
    version_string_ptr: 0x1428D3F92, // version string
    base_souls: 0x1446FEE88,         // souls base ptr
    mouse_enable: (0x1447103D8, 0x54),
    version: "1.08",
    format_string: 0x142952940,
    spawn_item_func_ptr: 0x1407B6230,
    map_item_man: 0x1447163f0,
};

const VER112: BaseAddresses = BaseAddresses {
    world_chr_man: 0x144763518,
    base_d: 0x14473e120,
    sprj_debug_event: 0x144735418,
    debug: 0x144763608,
    grend: 0x144550cf0,
    xa: 0x1f88,
    world_chr_man_dbg: 0x144763638,
    base_hbd: 0x144761220,

    offs_deathcam: 0x90,             // deathcam
    offs_no_goods_consume: 0x1EE2,   // no goods consume
    offs_speed: 0xA58,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 0x9,         // all no damage
    offs_no_update_ai: 0xD,          // no update ai
    mesh_lo: 0x14476130C,            // mesh (low hit)
    mesh_hi: 0x14476130D,            // mesh (high hit)
    instaqo: 0x144746988,            // insta qo
    version_string_ptr: 0x1428FD262, // version string
    base_souls: 0x144704268,         // souls base ptr
    mouse_enable: (0x144746988, 0x54),
    version: "1.12",
    format_string: 0x142952940,
    spawn_item_func_ptr: 0x1407BB750,
    map_item_man: 0x14474c9a0,
};

const VER115: BaseAddresses = BaseAddresses {
    world_chr_man: 0x144768e78,
    base_d: 0x144743a80,
    sprj_debug_event: 0x14473ad78,
    debug: 0x144768f68,
    grend: 0x144555cf0,
    xa: 0x1f90,
    world_chr_man_dbg: 0x144768F98,
    base_hbd: 0x144766B80,

    offs_deathcam: 0x90,             // deathcam
    offs_no_goods_consume: 0x1EEA,   // no goods consume
    offs_speed: 0xA58,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 0x9,         // all no damage
    offs_no_update_ai: 0xD,          // no update ai
    mesh_lo: 0x144766C6C,            // mesh (low hit)
    mesh_hi: 0x144766C6D,            // mesh (high hit)
    instaqo: 0x14474C2E8,            // insta qo
    version_string_ptr: 0x142900782, // version string
    base_souls: 0x144704268,         // souls base ptr
    mouse_enable: (0x14474C2E8, 0x54),
    version: "1.15",
    format_string: 0x142952940,
    spawn_item_func_ptr: 0x1407BBA70,
    map_item_man: 0x144752300,
};

pub(crate) fn detect_version() -> Option<BaseAddresses> {
    use libds3::version::{detect_version, Version};
    unsafe { detect_version() }.map(|ver| match ver {
        Version::Ver104 => VER104,
        Version::Ver108 => VER108,
        Version::Ver112 => VER112,
        Version::Ver115 => VER115,
    })
}

impl From<BaseAddresses> for PointerChains {
    fn from(b: BaseAddresses) -> Self {
        let BaseAddresses {
            world_chr_man,
            sprj_debug_event,
            debug,
            grend,
            xa,
            world_chr_man_dbg,
            base_hbd,
            base_souls,
            offs_all_no_damage,
            offs_player_exterminate,
            offs_no_goods_consume,
            offs_deathcam,
            offs_no_update_ai,
            offs_speed,
            mesh_hi,
            mesh_lo,
            instaqo,
            mouse_enable,
            spawn_item_func_ptr,
            map_item_man,
            ..
        } = b;

        PointerChains {
            all_no_damage: bitflag!(0b1; debug + offs_all_no_damage as usize),
            no_death: bitflag!(0b100; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            one_shot: bitflag!(0b1; debug + offs_player_exterminate as usize),
            inf_stamina: bitflag!(0b10000; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            inf_focus: bitflag!(0b100000; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            inf_consumables: bitflag!(0b1000; world_chr_man, 0x80, offs_no_goods_consume as _),
            deathcam: bitflag!(0b1; world_chr_man, offs_deathcam as usize),
            evt_draw: bitflag!(0b1; sprj_debug_event, 0xa8),
            evt_disable: bitflag!(0b1; sprj_debug_event, 0xd4),
            ai_disable: bitflag!(0b1; debug + offs_no_update_ai as usize),
            rend_chr: bitflag!(0b1; grend + 2),
            rend_obj: bitflag!(0b1; grend + 1),
            rend_map: bitflag!(0b1; grend),
            rend_mesh_hi: bitflag!(0b1; mesh_hi as usize),
            rend_mesh_lo: bitflag!(0b1; mesh_lo as usize),
            all_draw_hit: bitflag!(0b1; world_chr_man_dbg as usize, 0x66),
            ik_foot_ray: bitflag!(0b1; world_chr_man_dbg as usize, 0x6B),
            debug_sphere_1: bitflag!(0b1; base_hbd as usize, 0x30),
            debug_sphere_2: bitflag!(0b1; base_hbd as usize, 0x31),
            gravity: bitflag!(0b1000000; world_chr_man, 0x80, 0x1a08),
            speed: pointer_chain!(world_chr_man, 0x80, xa as _, 0x28, offs_speed as _),
            position: (
                pointer_chain!(world_chr_man, 0x40, 0x28, 0x74),
                pointer_chain!(world_chr_man, 0x40, 0x28, 0x80),
            ),
            souls: pointer_chain!(base_souls as _, 0x3d0, 0x74),
            map_item_man,
            spawn_item_func_ptr,
            world_chr_man,
            mouse_enable: pointer_chain!(mouse_enable.0 as _, mouse_enable.1 as _),
            quitout: pointer_chain!(instaqo as _, 0x250),
        }
    }
}
