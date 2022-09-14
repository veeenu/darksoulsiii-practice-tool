use crate::memedit::*;
use crate::prelude::base_addresses::BaseAddresses;
use crate::prelude::Version;

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
            offs_all_no_damage,
            offs_player_exterminate,
            offs_no_goods_consume,
            offs_deathcam,
            offs_no_update_ai,
            offs_speed,
            mesh_hi,
            mesh_lo,
            menu_man,
            mouse_enable_offs,
            spawn_item_func_ptr,
            map_item_man,
            ..
        } = b;

        const MESH_HI: usize = 0xEC;
        const MESH_LO: usize = 0xED;

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
            souls: pointer_chain!(sprj_debug_event as _, 0x3d0, 0x74),
            map_item_man,
            spawn_item_func_ptr,
            world_chr_man,
            mouse_enable: pointer_chain!(menu_man as _, mouse_enable_offs as _),
            quitout: pointer_chain!(menu_man as _, 0x250),
        }
    }
}
