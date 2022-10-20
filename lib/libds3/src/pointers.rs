use std::fmt::Display;
use std::mem::size_of;

use log::debug;
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

pub struct PointerChains {
    pub all_no_damage: Bitflag<u8>,
    pub no_death: Bitflag<u8>,
    pub one_shot: Bitflag<u8>,
    pub inf_stamina: Bitflag<u8>,
    pub inf_focus: Bitflag<u8>,
    pub inf_consumables: Bitflag<u8>,
    pub deathcam: Bitflag<u8>,
    pub evt_draw: Bitflag<u8>,
    pub evt_disable: Bitflag<u8>,
    pub ai_disable: Bitflag<u8>,
    pub rend_chr: Bitflag<u8>,
    pub rend_obj: Bitflag<u8>,
    pub rend_map: Bitflag<u8>,
    pub rend_mesh_hi: Bitflag<u8>,
    pub rend_mesh_lo: Bitflag<u8>,
    pub all_draw_hit: Bitflag<u8>,
    pub ik_foot_ray: Bitflag<u8>,
    pub debug_sphere_1: Bitflag<u8>,
    pub debug_sphere_2: Bitflag<u8>,
    pub gravity: Bitflag<u8>,
    pub speed: PointerChain<f32>,
    pub position: (PointerChain<f32>, PointerChain<[f32; 3]>),
    pub character_stats: PointerChain<CharacterStats>,
    pub souls: PointerChain<u32>,
    pub quitout: PointerChain<u8>,
    pub cursor_show: Bitflag<u8>,
    pub igt: PointerChain<u32>,
    pub no_logo: PointerChain<[u8; 20]>,

    #[allow(unused)]
    pub world_chr_man: usize,
    pub map_item_man: u64,
    pub spawn_item_func_ptr: u64,
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
            base_hbd,
            menu_man,
            spawn_item_func_ptr,
            map_item_man,
            no_logo,
            ..
        } = b;

        let offs_all_no_damage = 9;
        let offs_player_exterminate = 1;
        let offs_no_goods_consume = match *VERSION {
            // Version::V1_04_0 => 0x1ECA,
            Version::V1_03_1 => 0x1ECA,
            Version::V1_03_2 => 0x1ECA,
            Version::V1_04_1 => 0x1ECA,
            Version::V1_04_2 => 0x1ECA,
            Version::V1_04_3 => 0x1ECA,
            Version::V1_05_0 => 0x1ECA,
            Version::V1_05_1 => 0x1ECA,
            // Above this, the offsets are not checked.
            Version::V1_08_0 => 0x1EDA,
            Version::V1_09_0 => 0x1EDA, // todo
            Version::V1_10_0 => 0x1EDA, // todo
            Version::V1_11_0 => 0x1EDA, // todo
            Version::V1_12_0 => 0x1EE2,
            Version::V1_13_0 => 0x1EE2, // todo
            Version::V1_14_0 => 0x1EE2, // todo
            Version::V1_15_0 => 0x1EEA,
            Version::V1_15_1 => 0x1EEA,
        };
        let offs_deathcam = match *VERSION {
            // Version::V1_04_0 => 0x88,
            Version::V1_03_1 => 0x88,
            Version::V1_03_2 => 0x88,
            Version::V1_04_1 => 0x88,
            Version::V1_04_2 => 0x88,
            Version::V1_04_3 => 0x88,
            Version::V1_05_0 => 0x88,
            Version::V1_05_1 => 0x88,
            // Above this, the offsets are not checked.
            Version::V1_08_0 => 0x88,
            Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1 => 0x90,
        };
        let offs_speed = match *VERSION {
            Version::V1_03_1 => 0xa38,
            Version::V1_03_2 => 0xa38,
            Version::V1_04_1 => 0xa38,
            Version::V1_04_2 => 0xa38,
            Version::V1_04_3 => 0xa38,
            Version::V1_05_0 => 0xa38,
            Version::V1_05_1 => 0xa38,
            // Above this, the offsets are not checked.
            Version::V1_08_0 => 0xa38,
            Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1 => 0xa58,
        };
        let offs_igt = match *VERSION {
            Version::V1_03_1 => 0x9c,
            Version::V1_03_2 => 0x9c,
            Version::V1_04_1 => 0x9c,
            Version::V1_04_2 => 0x9c,
            Version::V1_04_3 => 0x9c,
            Version::V1_05_0 => 0x9c,
            Version::V1_05_1 => 0x9c,
            // Above this, the offsets are not checked.
            Version::V1_08_0
            | Version::V1_09_0
            | Version::V1_10_0
            | Version::V1_11_0
            | Version::V1_12_0
            | Version::V1_13_0
            | Version::V1_14_0
            | Version::V1_15_0
            | Version::V1_15_1 => 0xa4,
        };
        let offs_no_update_ai = 0xD;
        let mesh_hi = 0xEC;
        let mesh_lo = 0xED;
        let mouse_enable_offs = 0x54;

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
            rend_mesh_hi: bitflag!(0b1; base_hbd + mesh_hi as usize),
            rend_mesh_lo: bitflag!(0b1; base_hbd + mesh_lo as usize),
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
            character_stats: pointer_chain!(base_a, 0x10, 0x44),
            // souls was previously pointer_chain!(sprj_debug_event as _, 0x3d0, 0x74),
            souls: pointer_chain!(base_a, 0x10, 0x44 + 12 * size_of::<i32>()),
            map_item_man: map_item_man as _,
            spawn_item_func_ptr: spawn_item_func_ptr as _,
            world_chr_man,
            cursor_show: bitflag!(0b1; menu_man as _, mouse_enable_offs as _),
            igt: pointer_chain!(base_a as _, offs_igt),
            quitout: pointer_chain!(menu_man as _, 0x250),
            no_logo: pointer_chain!(no_logo as _),
        }
    }
}

impl Default for PointerChains {
    fn default() -> Self {
        Self::new()
    }
}

impl PointerChains {
    pub fn new() -> Self {
        let base_module_address = unsafe { GetModuleHandleA(None) }.unwrap().0 as usize;
        let base_addresses = BaseAddresses::from(*crate::version::VERSION)
            .with_module_base_addr(base_module_address);

        base_addresses.into()
    }
}
