use hudhook::memory::*;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use log::*;

pub(crate) struct PointerChains {
    pub(crate) all_no_damage: (PointerChain<u8>, u8),
    pub(crate) no_death: (PointerChain<u8>, u8),
    pub(crate) one_shot: (PointerChain<u8>, u8),
    pub(crate) inf_stamina: (PointerChain<u8>, u8),
    pub(crate) inf_focus: (PointerChain<u8>, u8),
    pub(crate) inf_consumables: (PointerChain<u8>, u8),
    pub(crate) deathcam: (PointerChain<u8>, u8),
    pub(crate) evt_draw: (PointerChain<u8>, u8),
    pub(crate) evt_disable: (PointerChain<u8>, u8),
    pub(crate) ai_disable: (PointerChain<u8>, u8),
    pub(crate) rend_chr: (PointerChain<u8>, u8),
    pub(crate) rend_obj: (PointerChain<u8>, u8),
    pub(crate) rend_map: (PointerChain<u8>, u8),
    pub(crate) rend_mesh_hi: (PointerChain<u8>, u8),
    pub(crate) rend_mesh_lo: (PointerChain<u8>, u8),
    pub(crate) all_draw_hit: PointerChain<u8>,
    pub(crate) ik_foot_ray: PointerChain<u8>,
    pub(crate) debug_sphere_1: PointerChain<u8>,
    pub(crate) debug_sphere_2: PointerChain<u8>,
    pub(crate) gravity: (PointerChain<u8>, u8),
    pub(crate) speed: PointerChain<f32>,
    pub(crate) position: (
        PointerChain<f32>,
        PointerChain<f32>,
        PointerChain<f32>,
        PointerChain<f32>,
    ),
    pub(crate) souls: PointerChain<u32>,
    pub(crate) item_spawn: (u64, u64, u64),
    pub(crate) quitout: PointerChain<u8>,
    pub(crate) mouse_enable: PointerChain<u8>,
    pub(crate) format_string: PointerChain<[u16; 90]>,
}

pub(crate) struct BaseAddresses {
    // offsets from base_b
    pub base_b: usize,
    pub base_d: usize,
    pub base_f: usize,
    pub debug: usize,
    pub grend: usize,
    pub xa: u32,

    // hitbox things
    pub base_z: u64,
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
    pub version_string_ptr: u64,
    pub base_souls: u64,
    pub item_spawn: (u64, u64, u64),
    pub mouse_enable: (u64, u64),

    pub version: &'static str,

    // other static pointers
    pub format_string: u64,
}

const VER104: BaseAddresses = BaseAddresses {
    // base_b: 0x1404BC5FA,             // base b
    // base_d: 0x1404C1DC0,             // base d
    // base_f: 0x1404C527D,             // base f
    // debug: 0x1408C3388,              // debug
    // grend: 0x140620B1B,              // game rend
    // xa: 0x140830AF1,                 // xa
    base_b: 0x1446c5dc8,
    base_d: 0x1446a09f8,
    base_f: 0x144697d88,
    debug: 0x1446c5eb8,
    grend: 0x1444bb000,
    xa: 0x1f70,
    base_z: 0x1446C5EE8,
    base_hbd: 0x1446C3AD0,

    offs_deathcam: 0x88,           // deathcam
    offs_no_goods_consume: 0x1ECA, // no goods consume
    offs_speed: 0xA38,             // speed
    offs_player_exterminate: 1,    // player exterminate
    offs_all_no_damage: 9,         // all no damage
    offs_no_update_ai: 9 + 4,      // no update ai

    mesh_lo: 0x1446C3BBC,            // mesh (low hit)
    mesh_hi: 0x1446C3BBD,            // mesh (high hit)
    instaqo: 0x1446A9280,            // insta qo
    version_string_ptr: 0x14288C422, // version string
    base_souls: 0x144704268,         // souls base ptr
    item_spawn: (0x1407abc00, 0x1446af280, 0),
    mouse_enable: (0x1446A9280, 0x54),
    version: "1.04",
    format_string: 0x142952940,
};

const VER108: BaseAddresses = BaseAddresses {
    // base_b: 0x1404C0DDA, // base b
    // base_d: 0x1404C6580, // base d
    // base_f: 0x1404C9A4D, // base f
    // debug: 0x1408D06F8,  // debug
    // grend: 0x1406287AB,  // game rend
    // xa: 0x14083BA91,     // xa
    base_b: 0x14472CF58,
    base_d: 0x144707b58,
    base_f: 0x1446fee88,
    debug: 0x14472d049,
    grend: 0x14451b608,
    xa: 0x1f80,
    base_z: 0x14472D078,
    base_hbd: 0x14472AC60,

    offs_deathcam: 0x88,             // deathcam
    offs_no_goods_consume: 0x1EDA,   // no goods consume
    offs_speed: 0xA38,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 9,           // all no damage
    offs_no_update_ai: 9 + 3,        // no update ai
    mesh_lo: 0x14472AD4C,            // mesh (low hit)
    mesh_hi: 0x14472AD4D,            // mesh (high hit)
    instaqo: 0x1447103D8,            // insta qo
    version_string_ptr: 0x1428D3F92, // version string
    base_souls: 0x1446FEE88,         // souls base ptr
    item_spawn: (0x1407B6230, 0x1447163f0, 0x14472cf58),
    mouse_enable: (0x1447103D8, 0x54),
    version: "1.08",
    format_string: 0x142952940,
};

const VER112: BaseAddresses = BaseAddresses {
    // base_b: 0x1404C191A, // base b
    // base_d: 0x1404C7120, // base d
    // base_f: 0x1404CA5ED, // base f
    // debug: 0x1408D7C88,  // debug
    // grend: 0x14062C45B,  // game rend
    // xa: 0x140841875,     // xa
    base_b: 0x144763518,
    base_d: 0x14473e120,
    base_f: 0x144735418,
    debug: 0x144763608,
    grend: 0x144550cf0,
    xa: 0x1f88,
    base_z: 0x144763638,
    base_hbd: 0x144761220,

    offs_deathcam: 0x90,             // deathcam
    offs_no_goods_consume: 0x1EE2,   // no goods consume
    offs_speed: 0xA58,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 9,           // all no damage
    offs_no_update_ai: 9 + 4,        // no update ai
    mesh_lo: 0x14476130C,            // mesh (low hit)
    mesh_hi: 0x14476130D,            // mesh (high hit)
    instaqo: 0x144746988,            // insta qo
    version_string_ptr: 0x1428FD262, // version string
    base_souls: 0x144704268,         // souls base ptr
    item_spawn: (0x1407BB750, 0x14474c9a0, 0x144763518),
    mouse_enable: (0x144746988, 0x54),
    version: "1.12",
    format_string: 0x142952940,
};

const VER115: BaseAddresses = BaseAddresses {
    // base_b: 0x1404C1A3A, // base b
    // base_d: 0x1404C7240, // base d
    // base_f: 0x1404CA70D, // base f
    // debug: 0x1408D9748,  // debug
    // xa: 0x140841D05,     // xa
    // grend: 0x14062C58B,  // game rend
    base_b: 0x144768e78,
    base_d: 0x144743a80,
    base_f: 0x14473ad78,
    debug: 0x144768f68,
    grend: 0x144555cf0,
    xa: 0x1f90,
    base_z: 0x144768F98,
    base_hbd: 0x144766B80,

    offs_deathcam: 0x90,             // deathcam
    offs_no_goods_consume: 0x1EEA,   // no goods consume
    offs_speed: 0xA58,               // speed
    offs_player_exterminate: 1,      // player exterminate
    offs_all_no_damage: 9,           // all no damage
    offs_no_update_ai: 9 + 4,        // no update ai
    mesh_lo: 0x144766C6C,            // mesh (low hit)
    mesh_hi: 0x144766C6D,            // mesh (high hit)
    instaqo: 0x14474C2E8,            // insta qo
    version_string_ptr: 0x142900782, // version string
    base_souls: 0x144704268,         // souls base ptr
    item_spawn: (0x1407BBA70, 0x144752300, 0x144768E78),
    mouse_enable: (0x14474C2E8, 0x54),
    version: "1.15",
    format_string: 0x142952940,
};

fn vercmp(ptr: usize, ver: &str) -> bool {
    let ver_mem = PointerChain::<[u16; 4]>::new(&[ptr]).read();

    if let Some(ver_mem) = ver_mem {
        let ver_memstr = OsString::from_wide(&ver_mem);
        info!(
            "Version string: matching {:?} against {:?}",
            ver, ver_memstr
        );
        ver_memstr == OsString::from(ver)
    } else {
        false
    }
}

// TODO switch to a proper #[repr(C)] struct here
fn base_chain(base: u64, sa: u64, sb: u64) -> Option<usize> {
    let boffs = PointerChain::<u32>::new(&[(base + sa) as _]).read()? as u64;
    Some((base + sb + boffs) as usize)
}

impl BaseAddresses {
    pub(crate) fn detect_version() -> Option<BaseAddresses> {
        if vercmp(VER104.version_string_ptr as _, "1.04") {
            Some(VER104)
        } else if vercmp(VER108.version_string_ptr as _, "1.08") {
            Some(VER108)
        } else if vercmp(VER112.version_string_ptr as _, "1.12") {
            Some(VER112)
        } else if vercmp(VER115.version_string_ptr as _, "1.15") {
            Some(VER115)
        } else {
            None
        }
    }

    // pub(crate) fn make_commands(&self) -> Vec<Box<dyn crate::command_ui::Command>> {

    pub(crate) fn make_commands(&self) -> Option<PointerChains> {
        // SAFETY TODO

        let &BaseAddresses {
            base_b,
            base_d,
            base_f,
            debug,
            grend,
            xa,
            base_z,
            base_hbd,
            base_souls,
            item_spawn,
            offs_all_no_damage,
            offs_player_exterminate,
            offs_no_goods_consume,
            offs_deathcam,
            offs_no_update_ai,
            offs_speed,
            mesh_hi,
            mesh_lo,
            instaqo,
            format_string,
            ..
        } = self;

        let item_spawn = (item_spawn.0, item_spawn.1, item_spawn.2);

        Some(PointerChains {
            all_no_damage: (PointerChain::new(&[debug + offs_all_no_damage as usize]), 0),
            no_death: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 2),
            one_shot: (
                PointerChain::new(&[debug + offs_player_exterminate as usize]),
                0,
            ),
            inf_stamina: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 4),
            inf_focus: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 5),
            inf_consumables: (
                PointerChain::new(&[base_b, 0x80, offs_no_goods_consume as _]),
                3,
            ),
            deathcam: (PointerChain::new(&[base_b, offs_deathcam as usize]), 0),
            evt_draw: (PointerChain::new(&[base_f, 0xa8]), 0),
            evt_disable: (PointerChain::new(&[base_f, 0xd4]), 0),
            ai_disable: (PointerChain::new(&[debug + offs_no_update_ai as usize]), 0),
            rend_chr: (PointerChain::new(&[grend + 2]), 0),
            rend_obj: (PointerChain::new(&[grend + 1]), 0),
            rend_map: (PointerChain::new(&[grend]), 0),
            rend_mesh_hi: (PointerChain::new(&[mesh_hi as usize]), 0),
            rend_mesh_lo: (PointerChain::new(&[mesh_lo as usize]), 0),
            all_draw_hit: PointerChain::new(&[base_z as usize, 0x66]),
            ik_foot_ray: PointerChain::new(&[base_z as usize, 0x6B]),
            debug_sphere_1: PointerChain::new(&[base_hbd as usize, 0x30]),
            debug_sphere_2: PointerChain::new(&[base_hbd as usize, 0x31]),
            gravity: (PointerChain::new(&[base_d, 0x60, 0x48]), 0),
            speed: PointerChain::new(&[base_b, 0x80, xa as _, 0x28, offs_speed as _]),
            position: (
                PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x80]),
                PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x88]),
                PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x84]),
                PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x74]),
            ),
            souls: PointerChain::new(&[base_souls as _, 0x3d0, 0x74]),
            item_spawn,
            mouse_enable: PointerChain::new(&[self.mouse_enable.0 as _, self.mouse_enable.1 as _]),
            quitout: PointerChain::new(&[instaqo as _, 0x250]),
            format_string: PointerChain::new(&[format_string as _]),
        })
    }
}