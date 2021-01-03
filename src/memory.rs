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
  pub(crate) gravity: (PointerChain<u8>, u8),
  pub(crate) speed: PointerChain<f32>,
  pub(crate) position: (PointerChain<f32>, PointerChain<f32>, PointerChain<f32>),
  pub(crate) souls: PointerChain<u32>,
  pub(crate) quitout: PointerChain<u8>,
}

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
  pub mesh_lo: u64,
  pub mesh_hi: u64,
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
  mesh_lo: 0x1446C3BBC,            // mesh (low hit)
  mesh_hi: 0x1446C3BBD,            // mesh (high hit)
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
  mesh_lo: 0x14472AD4C,            // mesh (low hit)
  mesh_hi: 0x14472AD4D,            // mesh (high hit)
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
  mesh_lo: 0x14476130C,            // mesh (low hit)
  mesh_hi: 0x14476130D,            // mesh (high hit)
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
  mesh_lo: 0x144766C6C,            // mesh (low hit)
  mesh_hi: 0x144766C6D,            // mesh (high hit)
  instaqo: 0x14474C2E8,            // insta qo
  version_string_ptr: 0x142900782, // version string
  base_souls: 0x144704268,         // souls base ptr
  version: "1.15",
};

/**
*
MESH FLAGS
115       112       108       104
144766C6C 14476130C 14472AD4C 1446C3BBC
144766C6D 14476130D 14472AD4D 1446C3BBC
*/

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
    let base_b = match base_chain(self.base_b, 3, 7) {
      Some(b) => b,
      None => {
        error!("Could not read base_b!");
        return None;
      }
    };
    let base_d = match base_chain(self.base_d, 3, 7) {
      Some(b) => b,
      None => {
        error!("Could not read base_d!");
        return None;
      }
    };
    let base_f = match base_chain(self.base_f, 3, 7) {
      Some(b) => b,
      None => {
        error!("Could not read base_f!");
        return None;
      }
    };
    let debug = match base_chain(self.debug, 3, 7) {
      Some(b) => b,
      None => {
        error!("Could not read debug!");
        return None;
      }
    };
    let grend = match base_chain(self.grend, 2, 7) {
      Some(b) => b,
      None => {
        error!("Could not read grend!");
        return None;
      }
    };
    let xa = match PointerChain::<u32>::new(&[self.xa as usize + 3]).read() {
      Some(b) => b,
      None => {
        error!("Could not read xa!");
        return None;
      }
    };

    /*let x = PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x80]);
    let y = PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x88]);
    let z = PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x84]);

    let position_pointer = Rc::new(RefCell::new(PositionPointer::new(x, y, z)));
    let save_position = Box::new(PositionPointerSaver(Rc::clone(&position_pointer)));
    let load_position = Box::new(PositionPointerLoader(Rc::clone(&position_pointer)));*/

    Some(PointerChains {
      all_no_damage: (
        PointerChain::new(&[debug + self.offs_all_no_damage as usize]),
        0,
      ),
      no_death: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 2),
      one_shot: (
        PointerChain::new(&[debug + self.offs_player_exterminate as usize]),
        0,
      ),
      inf_stamina: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 4),
      inf_focus: (PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]), 5),
      inf_consumables: (
        PointerChain::new(&[base_b, 0x80, self.offs_no_goods_consume as _]),
        3,
      ),
      deathcam: (PointerChain::new(&[base_b, self.offs_deathcam as usize]), 0),
      evt_draw: (PointerChain::new(&[base_f, 0xa8]), 0),
      evt_disable: (PointerChain::new(&[base_f, 0xd4]), 0),
      ai_disable: (
        PointerChain::new(&[debug + self.offs_no_update_ai as usize]),
        0,
      ),
      rend_chr: (PointerChain::new(&[grend + 2]), 0),
      rend_obj: (PointerChain::new(&[grend + 1]), 0),
      rend_map: (PointerChain::new(&[grend + 0]), 0),
      rend_mesh_hi: (PointerChain::new(&[self.mesh_hi as usize]), 0),
      rend_mesh_lo: (PointerChain::new(&[self.mesh_lo as usize]), 0),
      gravity: (PointerChain::new(&[base_d, 0x60, 0x48]), 0),
      speed: PointerChain::new(&[base_b, 0x80, xa as _, 0x28, self.offs_speed as _]),
      position: (
        PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x80]),
        PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x88]),
        PointerChain::<f32>::new(&[base_b, 0x40, 0x28, 0x84]),
      ),
      souls: PointerChain::new(&[self.base_souls as _, 0x3d0, 0x74]),
      quitout: PointerChain::new(&[self.instaqo as _, 0x250]),
    })

    /*
       vec![
       Box::new(FlagPointer::new(
       "all_no_damage",
       "All No Damage",
       PointerChain::new(&[debug + self.offs_all_no_damage as usize]),
       0,
       )),
       Box::new(FlagPointer::new(
       "no_death",
       "No Death",
       PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]),
       2,
       )),
       Box::new(FlagPointer::new(
       "one_shot",
       "One Shot",
       PointerChain::new(&[debug + self.offs_player_exterminate as usize]),
       0,
       )),
       Box::new(FlagPointer::new(
       "inf_stamina",
       "Inf Stamina",
       PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]),
       4,
       )),
       Box::new(FlagPointer::new(
       "inf_focus",
       "Inf Focus",
       PointerChain::new(&[base_b, 0x80, xa as _, 0x18, 0x1c0]),
       5,
       )),
       Box::new(FlagPointer::new(
       "inf_consumables",
       "Inf Consumables",
       PointerChain::new(&[base_b, 0x80, self.offs_no_goods_consume as _]),
       3,
       )),
       Box::new(FlagPointer::new(
       "deathcam",
       "Deathcam",
       PointerChain::new(&[base_b, self.offs_deathcam as usize]),
       0,
       )),
       Box::new(FlagPointer::new(
       "evt_draw",
       "Event Draw",
       PointerChain::new(&[base_f, 0xa8]),
       0,
       )),
       Box::new(FlagPointer::new(
       "evt_disable",
       "Event Disable",
       PointerChain::new(&[base_f, 0xd4]),
       0,
       )),
       Box::new(FlagPointer::new(
       "ai_disable",
       "AI Disable",
       PointerChain::new(&[debug + self.offs_no_update_ai as usize]),
       0,
       )),
       Box::new(FlagPointer::new(
       "rend_chr",
       "Render Character",
       PointerChain::new(&[grend + 2]),
       0,
       )),
       Box::new(FlagPointer::new(
       "rend_obj",
       "Render Objects",
    PointerChain::new(&[grend + 1]),
    0,
    )),
    Box::new(FlagPointer::new(
        "rend_map",
        "Render Map",
        PointerChain::new(&[grend + 0]),
        0,
    )),
    Box::new(FlagPointer::new(
        "rend_mesh_hi",
        "Render Mesh (high)",
        PointerChain::new(&[self.mesh_hi as usize]),
        0,
    )),
    Box::new(FlagPointer::new(
        "rend_mesh_lo",
        "Render Mesh (low)",
        PointerChain::new(&[self.mesh_lo as usize]),
        0,
    )),
    Box::new(FlagPointer::new(
        "gravity",
        "Gravity",
        PointerChain::new(&[base_d, 0x60, 0x48]),
        0,
    )),
    Box::new(CycleSpeedPointer::new(
        PointerChain::new(&[base_b, 0x80, xa as _, 0x28, self.offs_speed as _]),
    )),
    save_position,
    load_position,
    Box::new(SoulsPointer(PointerChain::new(&[
          self.base_souls as _,
          0x3d0,
          0x74,
    ]))),
    Box::new(QuitoutPointer(PointerChain::new(&[
          self.instaqo as _,
          0x250,
    ]))),
    ]
      */
  }
}
