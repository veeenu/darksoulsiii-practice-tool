use std::fmt::Display;
use std::mem::size_of;

use log::debug;
use once_cell::sync::Lazy;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::memedit::*;
use crate::prelude::base_addresses::BaseAddresses;
use crate::prelude::{Version, VERSION};

// Character stats
//

#[derive(Debug, Clone)]
#[repr(C)]
pub struct CharacterStats {
    pub vigor: i32,
    pub attunement: i32,
    pub endurance: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub luck: i32,
    pub unk1: i32,
    pub unk2: i32,
    pub vitality: i32,
    pub level: i32,
    pub souls: i32,
}

impl Display for CharacterStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "CharacterStats {{ }}")
    }
}

// DLC license check patch
//

/// Disables the DLC ownership re-checks that 1.08+ runs during gameplay.
///
/// Ownership is decided at boot, then re-queried from Steam by a watchdog
/// thread (every ~16.7s) and by a main-thread check requested at random
/// intervals. A single transient `false` from `BIsSubscribedApp` or
/// `BIsDlcInstalled` is treated as a revoked license: the game shows the DLC
/// license error and returns to the title screen. The boot-time check and DLC
/// loading are left untouched.
#[derive(Clone, Debug)]
pub struct LicenseCheckPatch {
    /// Watchdog check comparing ownership against the boot-time state.
    /// Returning 0 also skips the watchdog's own direct Steam queries.
    pub check_ownership_change: CodePatch<3>,
    /// Watchdog check for DLC content in use by the save that Steam now
    /// reports as not owned.
    pub check_ownership_in_use: CodePatch<3>,
    /// `mov byte [rcx+0x42], 1`: requests the main-thread re-check.
    pub request_recheck: CodePatch<4>,
    /// Shows the license error and returns to title. Backstop only.
    pub on_dlc_lost: CodePatch<1>,
}

impl LicenseCheckPatch {
    /// `change`/`in_use` are the function RVAs and their first 3 bytes, which
    /// vary per build.
    fn new(
        module_base: usize,
        change: (usize, [u8; 3]),
        in_use: (usize, [u8; 3]),
        request_recheck: usize,
        on_dlc_lost: usize,
    ) -> Self {
        // xor eax, eax; ret
        const RETURN_ZERO: [u8; 3] = [0x31, 0xC0, 0xC3];

        LicenseCheckPatch {
            check_ownership_change: CodePatch::new(
                pointer_chain!(module_base + change.0),
                change.1,
                RETURN_ZERO,
            ),
            check_ownership_in_use: CodePatch::new(
                pointer_chain!(module_base + in_use.0),
                in_use.1,
                RETURN_ZERO,
            ),
            request_recheck: CodePatch::new(
                pointer_chain!(module_base + request_recheck),
                [0xC6, 0x41, 0x42, 0x01],
                [0xC6, 0x41, 0x42, 0x00],
            ),
            // push rbx -> ret
            on_dlc_lost: CodePatch::new(pointer_chain!(module_base + on_dlc_lost), [0x40], [0xC3]),
        }
    }

    /// Applies every patch, or none of them if any site holds unexpected
    /// bytes. On error, returns the offending site.
    pub fn apply(&self) -> Result<(), (&'static str, CodePatchError)> {
        self.check_ownership_change.check().map_err(|e| ("check_ownership_change", e))?;
        self.check_ownership_in_use.check().map_err(|e| ("check_ownership_in_use", e))?;
        self.request_recheck.check().map_err(|e| ("request_recheck", e))?;
        self.on_dlc_lost.check().map_err(|e| ("on_dlc_lost", e))?;

        self.check_ownership_change.apply().map_err(|e| ("check_ownership_change", e))?;
        self.check_ownership_in_use.apply().map_err(|e| ("check_ownership_in_use", e))?;
        self.request_recheck.apply().map_err(|e| ("request_recheck", e))?;
        self.on_dlc_lost.apply().map_err(|e| ("on_dlc_lost", e))
    }
}

// Pointer chains
//

pub struct PointerChains {
    pub all_no_damage: Bitflag<u8>,
    pub no_death: Bitflag<u8>,
    pub one_shot: Bitflag<u8>,
    pub inf_stamina: Bitflag<u8>,
    pub inf_focus: Bitflag<u8>,
    pub inf_consumables: Bitflag<u8>,
    pub deathcam: Bitflag<u8>,
    pub evt_draw: Bitflag<u8>,
    pub bloodstain_draw: Bitflag<u8>,
    pub evt_disable: Bitflag<u8>,
    pub ai_disable: Bitflag<u8>,
    pub ember: Bitflag<u8>,
    pub rend_chr: Bitflag<u8>,
    pub rend_obj: Bitflag<u8>,
    pub rend_map: Bitflag<u8>,
    pub mesh_color: PointerChain<i32>,
    pub rend_mesh_hi: Bitflag<u8>,
    pub rend_mesh_lo: Bitflag<u8>,
    pub rend_mesh_hit: Bitflag<u8>,
    pub rend_hurtbox: Bitflag<u8>,
    pub debug_draw: Bitflag<u8>,
    pub all_draw_hit: Bitflag<u8>,
    pub ik_foot_ray: Bitflag<u8>,
    pub debug_sphere_1: Bitflag<u8>,
    pub debug_sphere_2: Bitflag<u8>,
    pub gravity: Bitflag<u8>,
    pub collision: Bitflag<u8>,
    pub speed: PointerChain<f32>,
    pub position: (PointerChain<f32>, PointerChain<[f32; 3]>),
    pub character_stats: PointerChain<CharacterStats>,
    pub souls: PointerChain<u32>,
    pub quitout: PointerChain<u8>,
    pub cursor_show: Bitflag<u8>,
    pub igt: PointerChain<u32>,
    pub fps: PointerChain<f32>,
    pub cur_anim: PointerChain<u32>,
    pub cur_anim_time: PointerChain<f32>,
    pub cur_anim_length: PointerChain<f32>,
    pub no_logo: PointerChain<[u8; 20]>,
    /// `None` before 1.08, which has no DLC code.
    pub license_check: Option<LicenseCheckPatch>,
    pub current_target: PointerChain<u64>,
    pub map_item_man: u64,
    pub spawn_item_func_ptr: u64,
    pub travel_ptr: usize,
    pub attune_ptr: usize,
    pub xa: u32,

    #[allow(unused)]
    pub world_chr_man: usize,
}

impl From<BaseAddresses> for PointerChains {
    fn from(b: BaseAddresses) -> Self {
        debug!("{:#?}", b);

        let BaseAddresses {
            world_chr_man,
            sprj_debug_event,
            debug,
            grend,
            xa,
            world_chr_man_dbg,
            base_a,
            base_fps,
            base_hbd,
            menu_man,
            spawn_item_func_ptr,
            map_item_man,
            no_logo,
            current_target,
            menu_travel,
            menu_attune,
            ..
        } = b;

        let offs_all_no_damage = 9;
        let offs_player_exterminate = 1;
        let offs_no_goods_consume = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1 => 0x1ECA,

            Version::V1_06_0
            | Version::V1_07_0
            | Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0 => 0x1EDA,

            Version::V1_12_0 | Version::V1_13_0 | Version::V1_14_0 => 0x1EE2,

            Version::V1_15_0 | Version::V1_15_1 | Version::V1_15_2 => 0x1EEA,
        };
        let offs_deathcam = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0
            | Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0 => 0x88,

            Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0x90,
        };
        let offs_bloodstain_draw = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3 => 0x2155,

            Version::V1_05_0 | Version::V1_05_1 | Version::V1_06_0 | Version::V1_07_0 => 0x2165,

            Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0 => 0x2185,

            Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0x2195,
        };
        let offs_speed = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0
            | Version::V1_08_0 => 0xa38,

            Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0xa58,
        };
        let offs_igt = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0 => 0x9c,

            Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0xa4,
        };
        let offs_fps = 0x08;
        let offs_debug_draw = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0 => 0x55,

            Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0x65,
        };

        let offs_ik_foot_ray = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0 => 0x5B,

            Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0x6B,
        };

        let offs_no_update_ai = 0xD;
        let mesh_hi = 0xEC;
        let mesh_lo = 0xED;
        let hurtbox = 0xEF;
        let mesh_hit = 0xF1;
        let mouse_enable_offs = 0x54;

        let offs_anim = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3 => 0x1F70,

            Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0
            | Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0 => 0x1F80,

            Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1
            | Version::V1_15_2 => 0x1F90,
        };

        // Hand-derived: the sites live in, or right next to, Arxan-obfuscated
        // code, which is laid out differently in every build, so no AoB
        // pattern finds them. Anchors used to locate them:
        // - on_dlc_lost: references "RegistReturnTitle" and loads message
        //   0x1069. Called by the main loop and by the watchdog.
        // - check_ownership_change/in_use: the functions the watchdog calls at
        //   +0x9b/+0xd5, after testing the CSDlc singleton.
        // - request_recheck: CSDlc+0x42 = 1, followed by a rand(1, 500) timer.
        let module_base = unsafe { GetModuleHandleA(None) }.unwrap().0 as usize;
        let license_check = match *VERSION {
            Version::V1_01_1
            | Version::V1_03_1
            | Version::V1_03_2
            | Version::V1_04_1
            | Version::V1_04_2
            | Version::V1_04_3
            | Version::V1_05_0
            | Version::V1_05_1
            | Version::V1_06_0
            | Version::V1_07_0 => None,

            Version::V1_08_0 => Some((
                (0xe72760, [0xE9, 0x1B, 0xAC]),
                (0xe72610, [0x40, 0x55, 0xEB]),
                0xec6b26,
                0x471d20,
            )),
            Version::V1_09_0 => Some((
                (0xe72e00, [0xE9, 0x1A, 0xE9]),
                (0xe72cb0, [0x40, 0x55, 0xEB]),
                0xec71c6,
                0x471d20,
            )),
            Version::V1_10_0 => Some((
                (0xe72e70, [0xE9, 0xAD, 0xE5]),
                (0xe72d20, [0x40, 0x55, 0xEB]),
                0xec7236,
                0x471d20,
            )),
            Version::V1_11_0 => Some((
                (0xe8c5b0, [0xE9, 0x29, 0x4B]),
                (0xe8c460, [0xE9, 0x6F, 0xAD]),
                0xee1096,
                0x4721f0,
            )),
            Version::V1_12_0 => Some((
                (0xe8d7e0, [0xE9, 0xA4, 0x08]),
                (0xe8d690, [0xE9, 0x68, 0xBF]),
                0xee22d6,
                0x472230,
            )),
            Version::V1_13_0 => Some((
                (0xe8fac0, [0xE9, 0xB2, 0x8B]),
                (0xe8f970, [0xE9, 0xE1, 0xD8]),
                0xee45b6,
                0x472300,
            )),
            Version::V1_14_0 => Some((
                (0xe905a0, [0xE9, 0x1B, 0x5D]),
                (0xe90400, [0xE9, 0xB9, 0x12]),
                0xee5176,
                0x472300,
            )),
            Version::V1_15_0 => Some((
                (0xe906b0, [0xE9, 0xAC, 0x29]),
                (0xe90510, [0xE9, 0xF2, 0x1A]),
                0xee5286,
                0x472300,
            )),
            Version::V1_15_1 => Some((
                (0xe9e3d0, [0xE9, 0xEA, 0xA3]),
                (0xe9e230, [0xE9, 0xA4, 0x44]),
                0xef3126,
                0x472440,
            )),
            Version::V1_15_2 => Some((
                (0xe9e500, [0xE9, 0x16, 0x9B]),
                (0xe9e360, [0xE9, 0x43, 0x13]),
                0xef3256,
                0x472440,
            )),
        }
        .map(|(change, in_use, request_recheck, on_dlc_lost)| {
            LicenseCheckPatch::new(module_base, change, in_use, request_recheck, on_dlc_lost)
        });

        PointerChains {
            all_no_damage: bitflag!(0b1; debug + offs_all_no_damage as usize),
            no_death: bitflag!(0b100; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            one_shot: bitflag!(0b1; debug + offs_player_exterminate as usize),
            inf_stamina: bitflag!(0b10000; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            inf_focus: bitflag!(0b100000; world_chr_man, 0x80, xa as _, 0x18, 0x1c0),
            inf_consumables: bitflag!(0b1000; world_chr_man, 0x80, offs_no_goods_consume as _),
            deathcam: bitflag!(0b1; world_chr_man, offs_deathcam as usize),
            evt_draw: bitflag!(0b1; sprj_debug_event, 0xa8),
            bloodstain_draw: bitflag!(0b1; world_chr_man, 0x40, 0x0, offs_bloodstain_draw as _),
            evt_disable: bitflag!(0b1; sprj_debug_event, 0xd4),
            ai_disable: bitflag!(0b1; debug + offs_no_update_ai as usize),
            ember: bitflag!(0b1; base_a, 0x10, 0x100),
            rend_chr: bitflag!(0b1; grend + 2),
            rend_obj: bitflag!(0b1; grend + 1),
            rend_map: bitflag!(0b1; grend),
            rend_mesh_hi: bitflag!(0b1; base_hbd + mesh_hi as usize),
            rend_mesh_lo: bitflag!(0b1; base_hbd + mesh_lo as usize),
            rend_mesh_hit: bitflag!(0b1; base_hbd + mesh_hit as usize),
            mesh_color: pointer_chain!(base_hbd + 0xF4),
            rend_hurtbox: bitflag!(0b1; base_hbd + hurtbox as usize),
            debug_draw: bitflag!(0b1; world_chr_man_dbg, offs_debug_draw),
            all_draw_hit: bitflag!(0b1; world_chr_man_dbg, 0x66),
            ik_foot_ray: bitflag!(0b1; world_chr_man_dbg, offs_ik_foot_ray),
            debug_sphere_1: bitflag!(0b1; base_hbd, 0x30),
            debug_sphere_2: bitflag!(0b1; base_hbd, 0x31),
            gravity: bitflag!(0b1000000; world_chr_man, 0x80, 0x1a08),
            collision: bitflag!(0b1; world_chr_man, 0x40, 0x0, 0x50, 0x187),
            speed: pointer_chain!(world_chr_man, 0x80, xa as _, 0x28, offs_speed as _),
            position: (
                pointer_chain!(world_chr_man, 0x40, 0x28, 0x74),
                pointer_chain!(world_chr_man, 0x40, 0x28, 0x80),
            ),
            character_stats: pointer_chain!(base_a, 0x10, 0x44),
            // souls was previously pointer_chain!(sprj_debug_event as _, 0x3d0, 0x74),
            souls: pointer_chain!(base_a, 0x10, 0x44 + 12 * size_of::<i32>()),
            map_item_man: map_item_man as _,
            spawn_item_func_ptr: spawn_item_func_ptr as _,
            travel_ptr: menu_travel,
            attune_ptr: menu_attune - 0x39,
            world_chr_man,
            cursor_show: bitflag!(0b1; menu_man as _, mouse_enable_offs as _),
            igt: pointer_chain!(base_a as _, offs_igt),
            fps: pointer_chain!(base_fps as _, offs_fps),
            cur_anim: pointer_chain!(world_chr_man as _, 0x80, offs_anim as _, 0x80, 0xC8),
            cur_anim_time: pointer_chain!(world_chr_man as _, 0x80, offs_anim as _, 0x10, 0x24),
            cur_anim_length: pointer_chain!(world_chr_man as _, 0x80, offs_anim as _, 0x10, 0x2C),
            quitout: pointer_chain!(menu_man as _, 0x250),
            current_target: pointer_chain!(current_target),
            no_logo: pointer_chain!(no_logo as _),
            license_check,
            xa: xa as u32,
        }
    }
}

/// The pointer chains for the running game version. They only hold addresses,
/// so one instance serves the whole module.
pub static POINTER_CHAINS: Lazy<PointerChains> = Lazy::new(|| {
    let base_module_address = unsafe { GetModuleHandleA(None) }.unwrap().0 as usize;
    let base_addresses =
        BaseAddresses::from(*crate::version::VERSION).with_module_base_addr(base_module_address);

    base_addresses.into()
});
