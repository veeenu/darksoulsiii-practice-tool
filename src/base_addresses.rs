use hudhook::memory::*;

use log::*;

use std::ffi::OsString;
use std::os::windows::prelude::*;

pub(crate) struct BaseAddresses {
  // offsets from base_b
  pub base_b: u64,
  pub offs_deathcam: u64,
  pub offs_no_goods_consume: u64,
  pub offs_speed: u64,
  pub base_d: u64,
  pub base_f: u64,
  pub xa: u64,

  // offsets from debug
  pub debug: u64,
  pub offs_player_exterminate: u64,
  pub offs_all_no_damage: u64,
  pub offs_no_update_ai: u64,

  pub grend: u64,
  pub instaqo: u64,
  pub version_string_ptr: u64,
  pub base_souls: u64,

  pub version: &'static str,
}

const VER104: BaseAddresses = BaseAddresses {
  base_b: 0x1404BC5FA,             // base b
  offs_deathcam: 0x88,             // deathcam
  offs_no_goods_consume: 0x1ECA,   // no goods consume
  offs_speed: 0xA38,               // speed
  base_d: 0x1404C1DC0,             // base d
  base_f: 0x1404C527D,             // base f
  xa: 0x140830AF1,                 // xa
  debug: 0x1408C3388,              // debug
  offs_player_exterminate: 1,      // player exterminate
  offs_all_no_damage: 9,           // all no damage
  offs_no_update_ai: 9 + 4,        // no update ai
  grend: 0x140620B1B,              // game rend
  instaqo: 0x1446A9280,            // insta qo
  version_string_ptr: 0x14288C422, // version string
  base_souls: 0x144704268,         // souls base ptr
  version: "1.04",
};

const VER108: BaseAddresses = BaseAddresses {
  base_b: 0x1404C0DDA,             // base b
  offs_deathcam: 0x88,             // deathcam
  offs_no_goods_consume: 0x1EDA,   // no goods consume
  offs_speed: 0xA38,               // speed
  base_d: 0x1404C6580,             // base d
  base_f: 0x1404C9A4D,             // base f
  xa: 0x14083BA91,                 // xa
  debug: 0x1408D06F8,              // debug
  offs_player_exterminate: 1,      // player exterminate
  offs_all_no_damage: 9,           // all no damage
  offs_no_update_ai: 9 + 3,        // no update ai
  grend: 0x1406287AB,              // game rend
  instaqo: 0x1447103D8,            // insta qo
  version_string_ptr: 0x1428D3F92, // version string
  base_souls: 0x1446FEE88,         // souls base ptr
  version: "1.08",
};

const VER112: BaseAddresses = BaseAddresses {
  base_b: 0x1404C191A,             // base b
  offs_deathcam: 0x90,             // deathcam
  offs_no_goods_consume: 0x1EE2,   // no goods consume
  offs_speed: 0xA58,               // speed
  base_d: 0x1404C7120,             // base d
  base_f: 0x1404CA5ED,             // base f
  xa: 0x140841875,                 // xa
  debug: 0x1408D7C88,              // debug
  offs_player_exterminate: 1,      // player exterminate
  offs_all_no_damage: 9,           // all no damage
  offs_no_update_ai: 9 + 4,        // no update ai
  grend: 0x14062C45B,              // game rend
  instaqo: 0x144746988,            // insta qo
  version_string_ptr: 0x1428FD262, // version string
  base_souls: 0x144704268,         // souls base ptr
  version: "1.12",
};

const VER115: BaseAddresses = BaseAddresses {
  base_b: 0x1404C1A3A,             // base b
  offs_deathcam: 0x90,             // deathcam
  offs_no_goods_consume: 0x1EEA,   // no goods consume
  offs_speed: 0xA58,               // speed
  base_d: 0x1404C7240,             // base d
  base_f: 0x1404CA70D,             // base f
  xa: 0x140841D05,                 // xa
  debug: 0x1408D9748,              // debug
  offs_player_exterminate: 1,      // player exterminate
  offs_all_no_damage: 9,           // all no damage
  offs_no_update_ai: 9 + 4,        // no update ai
  grend: 0x14062C58B,              // game rend
  instaqo: 0x14474C2E8,            // insta qo
  version_string_ptr: 0x142900782, // version string
  base_souls: 0x144704268,         // souls base ptr
  version: "1.15",
};

//unsafe fn vercmp(ptr: *const u16, ver: &str) -> bool {
// OsString::from_wide(std::slice::from_raw_parts(ptr, 4)) == OsString::from(ver)
//}

fn vercmp(ptr: isize, ver: &str) -> bool {
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
}
