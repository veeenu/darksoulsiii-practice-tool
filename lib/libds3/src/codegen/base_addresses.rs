// **********************************
// *** AUTOGENERATED, DO NOT EDIT ***
// **********************************
#[derive(Debug)]
pub struct BaseAddresses {
    pub world_chr_man: usize,
    pub world_chr_man_dbg: usize,
    pub menu_man: usize,
    pub base_a: usize,
    pub base_d: usize,
    pub sprj_debug_event: usize,
    pub debug: usize,
    pub grend: usize,
    pub base_hbd: usize,
    pub map_item_man: usize,
    pub spawn_item_func_ptr: usize,
    pub param: usize,
    pub format_string: usize,
    pub no_logo: usize,
    pub current_target: usize,
    pub menu_travel: usize,
    pub menu_attune: usize,
    pub xa: usize,
    pub base_fps: usize,
    pub base_bloodstain: usize,
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
            world_chr_man: self.world_chr_man + base,
            world_chr_man_dbg: self.world_chr_man_dbg + base,
            menu_man: self.menu_man + base,
            base_a: self.base_a + base,
            base_d: self.base_d + base,
            sprj_debug_event: self.sprj_debug_event + base,
            debug: self.debug + base,
            grend: self.grend + base,
            base_hbd: self.base_hbd + base,
            map_item_man: self.map_item_man + base,
            spawn_item_func_ptr: self.spawn_item_func_ptr + base,
            param: self.param + base,
            format_string: self.format_string + base,
            no_logo: self.no_logo + base,
            current_target: self.current_target + base,
            menu_travel: self.menu_travel + base,
            menu_attune: self.menu_attune + base,
            xa: self.xa,
            base_fps: self.base_fps + base,
            base_bloodstain: self.base_bloodstain + base,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    V1_03_1,
    V1_03_2,
    V1_04_1,
    V1_04_2,
    V1_04_3,
    V1_05_0,
    V1_05_1,
    V1_06_0,
    V1_07_0,
    V1_08_0,
    V1_09_0,
    V1_10_0,
    V1_11_0,
    V1_12_0,
    V1_13_0,
    V1_14_0,
    V1_15_0,
    V1_15_1,
    V1_15_2,
}

impl From<(u32, u32, u32)> for Version {
    fn from(v: (u32, u32, u32)) -> Self {
        match v {
            (1, 3, 1) => Version::V1_03_1,
            (1, 3, 2) => Version::V1_03_2,
            (1, 4, 1) => Version::V1_04_1,
            (1, 4, 2) => Version::V1_04_2,
            (1, 4, 3) => Version::V1_04_3,
            (1, 5, 0) => Version::V1_05_0,
            (1, 5, 1) => Version::V1_05_1,
            (1, 6, 0) => Version::V1_06_0,
            (1, 7, 0) => Version::V1_07_0,
            (1, 8, 0) => Version::V1_08_0,
            (1, 9, 0) => Version::V1_09_0,
            (1, 10, 0) => Version::V1_10_0,
            (1, 11, 0) => Version::V1_11_0,
            (1, 12, 0) => Version::V1_12_0,
            (1, 13, 0) => Version::V1_13_0,
            (1, 14, 0) => Version::V1_14_0,
            (1, 15, 0) => Version::V1_15_0,
            (1, 15, 1) => Version::V1_15_1,
            (1, 15, 2) => Version::V1_15_2,
            (maj, min, patch) => {
                log::error!("Unrecognized version {maj}.{min:02}.{patch}");
                panic!()
            }
        }
    }
}

impl From<Version> for (u32, u32, u32) {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_03_1 => (1, 3, 1),
            Version::V1_03_2 => (1, 3, 2),
            Version::V1_04_1 => (1, 4, 1),
            Version::V1_04_2 => (1, 4, 2),
            Version::V1_04_3 => (1, 4, 3),
            Version::V1_05_0 => (1, 5, 0),
            Version::V1_05_1 => (1, 5, 1),
            Version::V1_06_0 => (1, 6, 0),
            Version::V1_07_0 => (1, 7, 0),
            Version::V1_08_0 => (1, 8, 0),
            Version::V1_09_0 => (1, 9, 0),
            Version::V1_10_0 => (1, 10, 0),
            Version::V1_11_0 => (1, 11, 0),
            Version::V1_12_0 => (1, 12, 0),
            Version::V1_13_0 => (1, 13, 0),
            Version::V1_14_0 => (1, 14, 0),
            Version::V1_15_0 => (1, 15, 0),
            Version::V1_15_1 => (1, 15, 1),
            Version::V1_15_2 => (1, 15, 2),
        }
    }
}

impl From<Version> for BaseAddresses {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_03_1 => BASE_ADDRESSES_1_03_1,
            Version::V1_03_2 => BASE_ADDRESSES_1_03_2,
            Version::V1_04_1 => BASE_ADDRESSES_1_04_1,
            Version::V1_04_2 => BASE_ADDRESSES_1_04_2,
            Version::V1_04_3 => BASE_ADDRESSES_1_04_3,
            Version::V1_05_0 => BASE_ADDRESSES_1_05_0,
            Version::V1_05_1 => BASE_ADDRESSES_1_05_1,
            Version::V1_06_0 => BASE_ADDRESSES_1_06_0,
            Version::V1_07_0 => BASE_ADDRESSES_1_07_0,
            Version::V1_08_0 => BASE_ADDRESSES_1_08_0,
            Version::V1_09_0 => BASE_ADDRESSES_1_09_0,
            Version::V1_10_0 => BASE_ADDRESSES_1_10_0,
            Version::V1_11_0 => BASE_ADDRESSES_1_11_0,
            Version::V1_12_0 => BASE_ADDRESSES_1_12_0,
            Version::V1_13_0 => BASE_ADDRESSES_1_13_0,
            Version::V1_14_0 => BASE_ADDRESSES_1_14_0,
            Version::V1_15_0 => BASE_ADDRESSES_1_15_0,
            Version::V1_15_1 => BASE_ADDRESSES_1_15_1,
            Version::V1_15_2 => BASE_ADDRESSES_1_15_2,
        }
    }
}

pub const BASE_ADDRESSES_1_03_1: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c3aa8,
    world_chr_man_dbg: 0x46c3bc8,
    menu_man: 0x46a6f60,
    base_a: 0x469adf8,
    base_d: 0x469e6d8,
    sprj_debug_event: 0x4695a68,
    debug: 0x46c3b98,
    grend: 0x44b9000,
    base_hbd: 0x46c17b0,
    map_item_man: 0x469a988,
    spawn_item_func_ptr: 0x7ab590,
    param: 0x46e0760,
    format_string: 0x2905920,
    no_logo: 0xbbafdf,
    current_target: 0x847cfa,
    menu_travel: 0xbb44d0,
    menu_attune: 0xbb3af9,
    xa: 0x1f70,
    base_fps: 0x46e08a8,
    base_bloodstain: 0x4490f50,
};

pub const BASE_ADDRESSES_1_03_2: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c4aa8,
    world_chr_man_dbg: 0x46c4bc8,
    menu_man: 0x46a7f60,
    base_a: 0x469bdf8,
    base_d: 0x469f6d8,
    sprj_debug_event: 0x4696a68,
    debug: 0x46c4b98,
    grend: 0x44ba000,
    base_hbd: 0x46c27b0,
    map_item_man: 0x469b988,
    spawn_item_func_ptr: 0x7ab590,
    param: 0x46e1760,
    format_string: 0x2905ac0,
    no_logo: 0xbbafdf,
    current_target: 0x847cfa,
    menu_travel: 0xbb44d0,
    menu_attune: 0xbb3af9,
    xa: 0x1f70,
    base_fps: 0x46e18a8,
    base_bloodstain: 0x4491f50,
};

pub const BASE_ADDRESSES_1_04_1: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    menu_man: 0x46a9280,
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    sprj_debug_event: 0x4697d88,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    base_hbd: 0x46c3ad0,
    map_item_man: 0x469cca8,
    spawn_item_func_ptr: 0x7abc00,
    param: 0x46e2a80,
    format_string: 0x2906ae0,
    no_logo: 0xbbb0cf,
    current_target: 0x847a4a,
    menu_travel: 0xbb45c0,
    menu_attune: 0xbb3be9,
    xa: 0x1f70,
    base_fps: 0x46e2bc8,
    base_bloodstain: 0x4492f50,
};

pub const BASE_ADDRESSES_1_04_2: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    menu_man: 0x46a9280,
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    sprj_debug_event: 0x4697d88,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    base_hbd: 0x46c3ad0,
    map_item_man: 0x469cca8,
    spawn_item_func_ptr: 0x7abc00,
    param: 0x46e2a80,
    format_string: 0x2906cf0,
    no_logo: 0xbbb0cf,
    current_target: 0x847a4a,
    menu_travel: 0xbb45c0,
    menu_attune: 0xbb3be9,
    xa: 0x1f70,
    base_fps: 0x46e2bc8,
    base_bloodstain: 0x4492f50,
};

pub const BASE_ADDRESSES_1_04_3: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    menu_man: 0x46a9280,
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    sprj_debug_event: 0x4697d88,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    base_hbd: 0x46c3ad0,
    map_item_man: 0x469cca8,
    spawn_item_func_ptr: 0x7abc00,
    param: 0x46e2a80,
    format_string: 0x2906cf0,
    no_logo: 0xbbb0cf,
    current_target: 0x847a4a,
    menu_travel: 0xbb45c0,
    menu_attune: 0xbb3be9,
    xa: 0x1f70,
    base_fps: 0x46e2bc8,
    base_bloodstain: 0x4492f50,
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c9ec8,
    world_chr_man_dbg: 0x46c9fe8,
    menu_man: 0x46ad380,
    base_a: 0x46a1218,
    base_d: 0x46a4af8,
    sprj_debug_event: 0x469be88,
    debug: 0x46c9fb8,
    grend: 0x44bf010,
    base_hbd: 0x46c7bd0,
    map_item_man: 0x46a0da8,
    spawn_item_func_ptr: 0x7ac1e0,
    param: 0x46e6b90,
    format_string: 0x290a020,
    no_logo: 0xbbbf2f,
    current_target: 0x84809a,
    menu_travel: 0xbb5420,
    menu_attune: 0xbb4a49,
    xa: 0x1f80,
    base_fps: 0x46e6cd8,
    base_bloodstain: 0x4496f50,
};

pub const BASE_ADDRESSES_1_05_1: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c8ec8,
    world_chr_man_dbg: 0x46c8fe8,
    menu_man: 0x46ac380,
    base_a: 0x46a0218,
    base_d: 0x46a3af8,
    sprj_debug_event: 0x469ae88,
    debug: 0x46c8fb8,
    grend: 0x44be010,
    base_hbd: 0x46c6bd0,
    map_item_man: 0x469fda8,
    spawn_item_func_ptr: 0x7ac010,
    param: 0x46e5b90,
    format_string: 0x2909240,
    no_logo: 0xbbbd5f,
    current_target: 0x847eca,
    menu_travel: 0xbb5250,
    menu_attune: 0xbb4879,
    xa: 0x1f80,
    base_fps: 0x46e5cd8,
    base_bloodstain: 0x4495f50,
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46c9f28,
    world_chr_man_dbg: 0x46ca048,
    menu_man: 0x46ad3e0,
    base_a: 0x46a1278,
    base_d: 0x46a4b58,
    sprj_debug_event: 0x469bee8,
    debug: 0x46ca018,
    grend: 0x44bf010,
    base_hbd: 0x46c7c30,
    map_item_man: 0x46a0e08,
    spawn_item_func_ptr: 0x7ac5e0,
    param: 0x46e6bf0,
    format_string: 0x290a040,
    no_logo: 0xbbc32f,
    current_target: 0x84849a,
    menu_travel: 0xbb5820,
    menu_attune: 0xbb4e49,
    xa: 0x1f80,
    base_fps: 0x46e6d38,
    base_bloodstain: 0x4496f50,
};

pub const BASE_ADDRESSES_1_07_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x46ce768,
    world_chr_man_dbg: 0x46ce888,
    menu_man: 0x46b1c18,
    base_a: 0x46a5ab8,
    base_d: 0x46a9398,
    sprj_debug_event: 0x46a0728,
    debug: 0x46ce858,
    grend: 0x44c2ec8,
    base_hbd: 0x46cc470,
    map_item_man: 0x46a5648,
    spawn_item_func_ptr: 0x7ad4f0,
    param: 0x46eb458,
    format_string: 0x290d7a0,
    no_logo: 0xbbea5f,
    current_target: 0x8493aa,
    menu_travel: 0xbb7f50,
    menu_attune: 0xbb7579,
    xa: 0x1f80,
    base_fps: 0x46eb5a8,
    base_bloodstain: 0x449af50,
};

pub const BASE_ADDRESSES_1_08_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x472cf58,
    world_chr_man_dbg: 0x472d078,
    menu_man: 0x47103d8,
    base_a: 0x4704268,
    base_d: 0x4707b58,
    sprj_debug_event: 0x46fee88,
    debug: 0x472d049,
    grend: 0x451b608,
    base_hbd: 0x472ac60,
    map_item_man: 0x4703df8,
    spawn_item_func_ptr: 0x7b6230,
    param: 0x4749dd0,
    format_string: 0x2952940,
    no_logo: 0xbd6acf,
    current_target: 0x852b7a,
    menu_travel: 0xbcf6d0,
    menu_attune: 0xbcebf9,
    xa: 0x1f80,
    base_fps: 0x4749f18,
    base_bloodstain: 0x44f2f60,
};

pub const BASE_ADDRESSES_1_09_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x472d098,
    world_chr_man_dbg: 0x472d1b8,
    menu_man: 0x4710518,
    base_a: 0x47043a8,
    base_d: 0x4707c98,
    sprj_debug_event: 0x46fefc8,
    debug: 0x472d189,
    grend: 0x451b608,
    base_hbd: 0x472ada0,
    map_item_man: 0x4703f38,
    spawn_item_func_ptr: 0x7b6230,
    param: 0x4749f10,
    format_string: 0x2952670,
    no_logo: 0xbd708f,
    current_target: 0x852b7a,
    menu_travel: 0xbcfc90,
    menu_attune: 0xbcf1b9,
    xa: 0x1f80,
    base_fps: 0x474a058,
    base_bloodstain: 0x44f2f60,
};

pub const BASE_ADDRESSES_1_10_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x472d098,
    world_chr_man_dbg: 0x472d1b8,
    menu_man: 0x4710518,
    base_a: 0x47043a8,
    base_d: 0x4707c98,
    sprj_debug_event: 0x46fefc8,
    debug: 0x472d189,
    grend: 0x451b608,
    base_hbd: 0x472ada0,
    map_item_man: 0x4703f38,
    spawn_item_func_ptr: 0x7b6230,
    param: 0x4749f10,
    format_string: 0x2952670,
    no_logo: 0xbd70ff,
    current_target: 0x852b7a,
    menu_travel: 0xbcfd00,
    menu_attune: 0xbcf229,
    xa: 0x1f80,
    base_fps: 0x474a058,
    base_bloodstain: 0x44f2f60,
};

pub const BASE_ADDRESSES_1_11_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x4760398,
    world_chr_man_dbg: 0x47604b8,
    menu_man: 0x4743808,
    base_a: 0x4737698,
    base_d: 0x473afa0,
    sprj_debug_event: 0x4732298,
    debug: 0x4760488,
    grend: 0x454dce0,
    base_hbd: 0x475e0a0,
    map_item_man: 0x4749820,
    spawn_item_func_ptr: 0x7bafc0,
    param: 0x477d4d0,
    format_string: 0x297ae40,
    no_logo: 0xbe6f8f,
    current_target: 0x85857a,
    menu_travel: 0xbdfb90,
    menu_attune: 0xbdf0b9,
    xa: 0x1f88,
    base_fps: 0x477d618,
    base_bloodstain: 0x4524f60,
};

pub const BASE_ADDRESSES_1_12_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x4763518,
    world_chr_man_dbg: 0x4763638,
    menu_man: 0x4746988,
    base_a: 0x473a818,
    base_d: 0x473e120,
    sprj_debug_event: 0x4735418,
    debug: 0x4763608,
    grend: 0x4550cf0,
    base_hbd: 0x4761220,
    map_item_man: 0x474c9a0,
    spawn_item_func_ptr: 0x7bb750,
    param: 0x4780660,
    format_string: 0x297d2e0,
    no_logo: 0xbe7d9f,
    current_target: 0x858d6a,
    menu_travel: 0xbe09a0,
    menu_attune: 0xbdfec9,
    xa: 0x1f88,
    base_fps: 0x47807a8,
    base_bloodstain: 0x4527f60,
};

pub const BASE_ADDRESSES_1_13_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x4766d18,
    world_chr_man_dbg: 0x4766e38,
    menu_man: 0x474a188,
    base_a: 0x473e018,
    base_d: 0x4741920,
    sprj_debug_event: 0x4738c18,
    debug: 0x4766e08,
    grend: 0x4553cf0,
    base_hbd: 0x4764a20,
    map_item_man: 0x47501a0,
    spawn_item_func_ptr: 0x7bb940,
    param: 0x4783e80,
    format_string: 0x297f9f0,
    no_logo: 0xbe993f,
    current_target: 0x85a61a,
    menu_travel: 0xbe2540,
    menu_attune: 0xbe1a69,
    xa: 0x1f90,
    base_fps: 0x4783fc8,
    base_bloodstain: 0x452af60,
};

pub const BASE_ADDRESSES_1_14_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x4768e78,
    world_chr_man_dbg: 0x4768f98,
    menu_man: 0x474c2e8,
    base_a: 0x4740178,
    base_d: 0x4743a80,
    sprj_debug_event: 0x473ad78,
    debug: 0x4768f68,
    grend: 0x4555cf0,
    base_hbd: 0x4766b80,
    map_item_man: 0x4752300,
    spawn_item_func_ptr: 0x7bba30,
    param: 0x4785fe0,
    format_string: 0x2980a00,
    no_logo: 0xbe9c0f,
    current_target: 0x85a70a,
    menu_travel: 0xbe2810,
    menu_attune: 0xbe1d39,
    xa: 0x1f90,
    base_fps: 0x4786128,
    base_bloodstain: 0x452cf60,
};

pub const BASE_ADDRESSES_1_15_0: BaseAddresses = BaseAddresses {
    world_chr_man: 0x4768e78,
    world_chr_man_dbg: 0x4768f98,
    menu_man: 0x474c2e8,
    base_a: 0x4740178,
    base_d: 0x4743a80,
    sprj_debug_event: 0x473ad78,
    debug: 0x4768f68,
    grend: 0x4555cf0,
    base_hbd: 0x4766b80,
    map_item_man: 0x4752300,
    spawn_item_func_ptr: 0x7bba70,
    param: 0x4785fe0,
    format_string: 0x2980a30,
    no_logo: 0xbe9d0f,
    current_target: 0x85a74a,
    menu_travel: 0xbe2910,
    menu_attune: 0xbe1e39,
    xa: 0x1f90,
    base_fps: 0x4786128,
    base_bloodstain: 0x452cf60,
};

pub const BASE_ADDRESSES_1_15_1: BaseAddresses = BaseAddresses {
    world_chr_man: 0x477fdb8,
    world_chr_man_dbg: 0x477fed8,
    menu_man: 0x4763258,
    base_a: 0x47572b8,
    base_d: 0x475abd0,
    sprj_debug_event: 0x4751eb8,
    debug: 0x477fea8,
    grend: 0x456cba8,
    base_hbd: 0x477dac0,
    map_item_man: 0x4769240,
    spawn_item_func_ptr: 0x7c3cd0,
    param: 0x479b8c0,
    format_string: 0x2991650,
    no_logo: 0xbf42bf,
    current_target: 0x862cba,
    menu_travel: 0xbecec0,
    menu_attune: 0xbec3e9,
    xa: 0x1f90,
    base_fps: 0x479ba08,
    base_bloodstain: 0x4543f60,
};

pub const BASE_ADDRESSES_1_15_2: BaseAddresses = BaseAddresses {
    world_chr_man: 0x477fdb8,
    world_chr_man_dbg: 0x477fec8,
    menu_man: 0x4763258,
    base_a: 0x47572b8,
    base_d: 0x475abd0,
    sprj_debug_event: 0x4751eb8,
    debug: 0x477fe94,
    grend: 0x456cba8,
    base_hbd: 0x477dac0,
    map_item_man: 0x4769240,
    spawn_item_func_ptr: 0x7c4080,
    param: 0x479b8b0,
    format_string: 0x2991610,
    no_logo: 0xbf43ef,
    current_target: 0x86306a,
    menu_travel: 0xbecff0,
    menu_attune: 0xbec519,
    xa: 0x1f90,
    base_fps: 0x479b9f8,
    base_bloodstain: 0x4543f60,
};

