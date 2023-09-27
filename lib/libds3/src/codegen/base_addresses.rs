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
    pub xa: usize,
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
            xa: self.xa,
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
            },
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
    base_a: 0x469adf8,
    base_d: 0x469e6d8,
    base_hbd: 0x46c17b0,
    debug: 0x46c3b98,
    grend: 0x44b9000,
    map_item_man: 0x469a988,
    menu_man: 0x46a6f60,
    param: 0x46e0760,
    spawn_item_func_ptr: 0x7ab590,
    sprj_debug_event: 0x4695a68,
    world_chr_man: 0x46c3aa8,
    world_chr_man_dbg: 0x46c3bc8,
    current_target: 0x847cfa,
    format_string: 0x2905920,
    no_logo: 0xbbafdf,
    xa: 0x1f70,
};

pub const BASE_ADDRESSES_1_03_2: BaseAddresses = BaseAddresses {
    base_a: 0x469bdf8,
    base_d: 0x469f6d8,
    base_hbd: 0x46c27b0,
    debug: 0x46c4b98,
    grend: 0x44ba000,
    map_item_man: 0x469b988,
    menu_man: 0x46a7f60,
    param: 0x46e1760,
    spawn_item_func_ptr: 0x7ab590,
    sprj_debug_event: 0x4696a68,
    world_chr_man: 0x46c4aa8,
    world_chr_man_dbg: 0x46c4bc8,
    current_target: 0x847cfa,
    format_string: 0x2905ac0,
    no_logo: 0xbbafdf,
    xa: 0x1f70,
};

pub const BASE_ADDRESSES_1_04_1: BaseAddresses = BaseAddresses {
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    base_hbd: 0x46c3ad0,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    map_item_man: 0x469cca8,
    menu_man: 0x46a9280,
    param: 0x46e2a80,
    spawn_item_func_ptr: 0x7abc00,
    sprj_debug_event: 0x4697d88,
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    current_target: 0x847a4a,
    format_string: 0x2906ae0,
    no_logo: 0xbbb0cf,
    xa: 0x1f70,
};

pub const BASE_ADDRESSES_1_04_2: BaseAddresses = BaseAddresses {
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    base_hbd: 0x46c3ad0,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    map_item_man: 0x469cca8,
    menu_man: 0x46a9280,
    param: 0x46e2a80,
    spawn_item_func_ptr: 0x7abc00,
    sprj_debug_event: 0x4697d88,
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    current_target: 0x847a4a,
    format_string: 0x2906cf0,
    no_logo: 0xbbb0cf,
    xa: 0x1f70,
};

pub const BASE_ADDRESSES_1_04_3: BaseAddresses = BaseAddresses {
    base_a: 0x469d118,
    base_d: 0x46a09f8,
    base_hbd: 0x46c3ad0,
    debug: 0x46c5eb8,
    grend: 0x44bb000,
    map_item_man: 0x469cca8,
    menu_man: 0x46a9280,
    param: 0x46e2a80,
    spawn_item_func_ptr: 0x7abc00,
    sprj_debug_event: 0x4697d88,
    world_chr_man: 0x46c5dc8,
    world_chr_man_dbg: 0x46c5ee8,
    current_target: 0x847a4a,
    format_string: 0x2906cf0,
    no_logo: 0xbbb0cf,
    xa: 0x1f70,
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
    base_a: 0x46a1218,
    base_d: 0x46a4af8,
    base_hbd: 0x46c7bd0,
    debug: 0x46c9fb8,
    grend: 0x44bf010,
    map_item_man: 0x46a0da8,
    menu_man: 0x46ad380,
    param: 0x46e6b90,
    spawn_item_func_ptr: 0x7ac1e0,
    sprj_debug_event: 0x469be88,
    world_chr_man: 0x46c9ec8,
    world_chr_man_dbg: 0x46c9fe8,
    current_target: 0x84809a,
    format_string: 0x290a020,
    no_logo: 0xbbbf2f,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_05_1: BaseAddresses = BaseAddresses {
    base_a: 0x46a0218,
    base_d: 0x46a3af8,
    base_hbd: 0x46c6bd0,
    debug: 0x46c8fb8,
    grend: 0x44be010,
    map_item_man: 0x469fda8,
    menu_man: 0x46ac380,
    param: 0x46e5b90,
    spawn_item_func_ptr: 0x7ac010,
    sprj_debug_event: 0x469ae88,
    world_chr_man: 0x46c8ec8,
    world_chr_man_dbg: 0x46c8fe8,
    current_target: 0x847eca,
    format_string: 0x2909240,
    no_logo: 0xbbbd5f,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
    base_a: 0x46a1278,
    base_d: 0x46a4b58,
    base_hbd: 0x46c7c30,
    debug: 0x46ca018,
    grend: 0x44bf010,
    map_item_man: 0x46a0e08,
    menu_man: 0x46ad3e0,
    param: 0x46e6bf0,
    spawn_item_func_ptr: 0x7ac5e0,
    sprj_debug_event: 0x469bee8,
    world_chr_man: 0x46c9f28,
    world_chr_man_dbg: 0x46ca048,
    current_target: 0x84849a,
    format_string: 0x290a040,
    no_logo: 0xbbc32f,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_07_0: BaseAddresses = BaseAddresses {
    base_a: 0x46a5ab8,
    base_d: 0x46a9398,
    base_hbd: 0x46cc470,
    debug: 0x46ce858,
    grend: 0x44c2ec8,
    map_item_man: 0x46a5648,
    menu_man: 0x46b1c18,
    param: 0x46eb458,
    spawn_item_func_ptr: 0x7ad4f0,
    sprj_debug_event: 0x46a0728,
    world_chr_man: 0x46ce768,
    world_chr_man_dbg: 0x46ce888,
    current_target: 0x8493aa,
    format_string: 0x290d7a0,
    no_logo: 0xbbea5f,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_08_0: BaseAddresses = BaseAddresses {
    base_a: 0x4704268,
    base_d: 0x4707b58,
    base_hbd: 0x472ac60,
    debug: 0x472d049,
    grend: 0x451b608,
    map_item_man: 0x4703df8,
    menu_man: 0x47103d8,
    param: 0x4749dd0,
    spawn_item_func_ptr: 0x7b6230,
    sprj_debug_event: 0x46fee88,
    world_chr_man: 0x472cf58,
    world_chr_man_dbg: 0x472d078,
    current_target: 0x852b7a,
    format_string: 0x2952940,
    no_logo: 0xbd6acf,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_09_0: BaseAddresses = BaseAddresses {
    base_a: 0x47043a8,
    base_d: 0x4707c98,
    base_hbd: 0x472ada0,
    debug: 0x472d189,
    grend: 0x451b608,
    map_item_man: 0x4703f38,
    menu_man: 0x4710518,
    param: 0x4749f10,
    spawn_item_func_ptr: 0x7b6230,
    sprj_debug_event: 0x46fefc8,
    world_chr_man: 0x472d098,
    world_chr_man_dbg: 0x472d1b8,
    current_target: 0x852b7a,
    format_string: 0x2952670,
    no_logo: 0xbd708f,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_10_0: BaseAddresses = BaseAddresses {
    base_a: 0x47043a8,
    base_d: 0x4707c98,
    base_hbd: 0x472ada0,
    debug: 0x472d189,
    grend: 0x451b608,
    map_item_man: 0x4703f38,
    menu_man: 0x4710518,
    param: 0x4749f10,
    spawn_item_func_ptr: 0x7b6230,
    sprj_debug_event: 0x46fefc8,
    world_chr_man: 0x472d098,
    world_chr_man_dbg: 0x472d1b8,
    current_target: 0x852b7a,
    format_string: 0x2952670,
    no_logo: 0xbd70ff,
    xa: 0x1f80,
};

pub const BASE_ADDRESSES_1_11_0: BaseAddresses = BaseAddresses {
    base_a: 0x4737698,
    base_d: 0x473afa0,
    base_hbd: 0x475e0a0,
    debug: 0x4760488,
    grend: 0x454dce0,
    map_item_man: 0x4749820,
    menu_man: 0x4743808,
    param: 0x477d4d0,
    spawn_item_func_ptr: 0x7bafc0,
    sprj_debug_event: 0x4732298,
    world_chr_man: 0x4760398,
    world_chr_man_dbg: 0x47604b8,
    current_target: 0x85857a,
    format_string: 0x297ae40,
    no_logo: 0xbe6f8f,
    xa: 0x1f88,
};

pub const BASE_ADDRESSES_1_12_0: BaseAddresses = BaseAddresses {
    base_a: 0x473a818,
    base_d: 0x473e120,
    base_hbd: 0x4761220,
    debug: 0x4763608,
    grend: 0x4550cf0,
    map_item_man: 0x474c9a0,
    menu_man: 0x4746988,
    param: 0x4780660,
    spawn_item_func_ptr: 0x7bb750,
    sprj_debug_event: 0x4735418,
    world_chr_man: 0x4763518,
    world_chr_man_dbg: 0x4763638,
    current_target: 0x858d6a,
    format_string: 0x297d2e0,
    no_logo: 0xbe7d9f,
    xa: 0x1f88,
};

pub const BASE_ADDRESSES_1_13_0: BaseAddresses = BaseAddresses {
    base_a: 0x473e018,
    base_d: 0x4741920,
    base_hbd: 0x4764a20,
    debug: 0x4766e08,
    grend: 0x4553cf0,
    map_item_man: 0x47501a0,
    menu_man: 0x474a188,
    param: 0x4783e80,
    spawn_item_func_ptr: 0x7bb940,
    sprj_debug_event: 0x4738c18,
    world_chr_man: 0x4766d18,
    world_chr_man_dbg: 0x4766e38,
    current_target: 0x85a61a,
    format_string: 0x297f9f0,
    no_logo: 0xbe993f,
    xa: 0x1f90,
};

pub const BASE_ADDRESSES_1_14_0: BaseAddresses = BaseAddresses {
    base_a: 0x4740178,
    base_d: 0x4743a80,
    base_hbd: 0x4766b80,
    debug: 0x4768f68,
    grend: 0x4555cf0,
    map_item_man: 0x4752300,
    menu_man: 0x474c2e8,
    param: 0x4785fe0,
    spawn_item_func_ptr: 0x7bba30,
    sprj_debug_event: 0x473ad78,
    world_chr_man: 0x4768e78,
    world_chr_man_dbg: 0x4768f98,
    current_target: 0x85a70a,
    format_string: 0x2980a00,
    no_logo: 0xbe9c0f,
    xa: 0x1f90,
};

pub const BASE_ADDRESSES_1_15_0: BaseAddresses = BaseAddresses {
    base_a: 0x4740178,
    base_d: 0x4743a80,
    base_hbd: 0x4766b80,
    debug: 0x4768f68,
    grend: 0x4555cf0,
    map_item_man: 0x4752300,
    menu_man: 0x474c2e8,
    param: 0x4785fe0,
    spawn_item_func_ptr: 0x7bba70,
    sprj_debug_event: 0x473ad78,
    world_chr_man: 0x4768e78,
    world_chr_man_dbg: 0x4768f98,
    current_target: 0x85a74a,
    format_string: 0x2980a30,
    no_logo: 0xbe9d0f,
    xa: 0x1f90,
};

pub const BASE_ADDRESSES_1_15_1: BaseAddresses = BaseAddresses {
    base_a: 0x47572b8,
    base_d: 0x475abd0,
    base_hbd: 0x477dac0,
    debug: 0x477fea8,
    grend: 0x456cba8,
    map_item_man: 0x4769240,
    menu_man: 0x4763258,
    param: 0x479b8c0,
    spawn_item_func_ptr: 0x7c3cd0,
    sprj_debug_event: 0x4751eb8,
    world_chr_man: 0x477fdb8,
    world_chr_man_dbg: 0x477fed8,
    current_target: 0x862cba,
    format_string: 0x2991650,
    no_logo: 0xbf42bf,
    xa: 0x1f90,
};

pub const BASE_ADDRESSES_1_15_2: BaseAddresses = BaseAddresses {
    base_a: 0x47572b8,
    base_d: 0x475abd0,
    base_hbd: 0x477dac0,
    debug: 0x477fe94,
    grend: 0x456cba8,
    map_item_man: 0x4769240,
    menu_man: 0x4763258,
    param: 0x479b8b0,
    spawn_item_func_ptr: 0x7c4080,
    sprj_debug_event: 0x4751eb8,
    world_chr_man: 0x477fdb8,
    world_chr_man_dbg: 0x477fec8,
    current_target: 0x86306a,
    format_string: 0x2991610,
    no_logo: 0xbf43ef,
    xa: 0x1f90,
};
