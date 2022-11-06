#![feature(once_cell)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::LazyLock;

use libds3::prelude::*;
use serde::{de, Deserialize, Deserializer};
use toml::Value;
use windows::core::{GUID, HRESULT, PCSTR};
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

type FDirectInput8Create = unsafe extern "stdcall" fn(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT;

struct State {
    directinput8create: FDirectInput8Create,
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

static STATE: LazyLock<State> = LazyLock::new(|| unsafe {
    let dinput8 = LoadLibraryA(PCSTR(b"C:\\Windows\\System32\\dinput8.dll\0".as_ptr())).unwrap();
    let directinput8create =
        std::mem::transmute(GetProcAddress(dinput8, PCSTR(b"DirectInput8Create\0".as_ptr())));
    println!("Called!");

    State { directinput8create }
});

fn initialize() {
    LazyLock::force(&STATE);
    unsafe { AllocConsole() };
}

fn no_logo() {
    let pointer_chains = PointerChains::new();
    pointer_chains
        .no_logo
        .write([
            0x48, 0x31, 0xC0, 0x48, 0x89, 0x02, 0x49, 0x89, 0x04, 0x24, 0x90, 0x90, 0x90, 0x90,
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        ])
        .unwrap();
}

#[derive(Deserialize)]
#[serde(transparent)]
struct PatchConfig(HashMap<String, ParamIdConfig>);

impl PatchConfig {
    fn apply(self, params: &mut Params) {
        for (param_name, param_id_cfg) in self.0 {
            println!("Applying to {param_name}");
            param_id_cfg.apply(&param_name, params);
        }
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ParamIdConfig(
    #[serde(deserialize_with = "deserialize_param_id_config_kv")] HashMap<u64, ParamValues>,
);

impl ParamIdConfig {
    fn apply(mut self, param_name: &str, params: &mut Params) {
        let idx_map = match unsafe { params.iter_param_ids(param_name) } {
            Some(it) => {
                it.enumerate().filter(|(_, id)| self.0.contains_key(id as _)).collect::<Vec<_>>()
            },
            None => {
                eprintln!("  Couldn't iter param ids: {param_name}");
                return;
            },
        };

        for (param_idx, id) in idx_map {
            println!("  Visiting {param_name}:{param_idx}");
            params.visit_param_item(param_name, param_idx, self.0.get_mut(&id).unwrap());
        }
    }
}

fn deserialize_param_id_config_kv<'de, D>(
    deserializer: D,
) -> Result<HashMap<u64, ParamValues>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_map = HashMap::<&str, ParamValues>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err({
                    de::Error::invalid_value(
                        de::Unexpected::Str(str_key),
                        &"a non-negative integer",
                    )
                }),
            })
            .collect::<Result<HashMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}

#[derive(Deserialize)]
#[serde(transparent)]
struct ParamValues(HashMap<String, Value>);

impl ParamVisitor for ParamValues {
    fn visit_u8(&mut self, name: &str, v: &mut u8) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_u16(&mut self, name: &str, v: &mut u16) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_u32(&mut self, name: &str, v: &mut u32) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_i8(&mut self, name: &str, v: &mut i8) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_i16(&mut self, name: &str, v: &mut i16) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_i32(&mut self, name: &str, v: &mut i32) {
        if let Some(Value::Integer(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_f32(&mut self, name: &str, v: &mut f32) {
        if let Some(Value::Float(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i as _;
        }
    }

    fn visit_bool(&mut self, name: &str, v: &mut bool) {
        if let Some(Value::Boolean(i)) = self.0.get(name) {
            println!("    Applied {name} := {i}");
            *v = *i;
        }
    }
}

unsafe fn patch() {
    no_logo();
    std::thread::spawn(|| {
        let mut params = PARAMS.write();

        let patch_config: PatchConfig = toml::from_str(&std::fs::read_to_string("param-mod.toml").unwrap()).unwrap();

        drop(wait_option(|| {
            if let Err(e) = params.refresh() {
                eprintln!("Error: {:?}", e);
            }
            params.get_equip_param_weapon()
        }));

        patch_config.apply(&mut params);

        // if let Some(shortsword) = wait_option(|| {
        //     if let Err(e) = params.refresh() {
        //         eprintln!("Error: {:?}", e);
        //     }
        //     params.get_equip_param_weapon()
        // })
        // .find(|i| i.id == 2000000)
        // .and_then(|p| p.param)
        // {
        //     shortsword.atk_base_physics = i16::MAX;
        //     shortsword.atk_base_magic = i16::MAX;
        //     shortsword.atk_base_fire = i16::MAX;
        //     shortsword.atk_base_thunder = i16::MAX;
        //     shortsword.stamina_consume_rate = 0.;
        // } else {
        //     eprintln!("Shortsword not found");
        // }

        // if let Some(sellsword) = wait_option(|| {
        //     if let Err(e) = params.refresh() {
        //         eprintln!("Error: {:?}", e);
        //     }
        //     params.get_equip_param_weapon()
        // })
        // .find(|i| i.id == 16000200)
        // .and_then(|p| p.param)
        // {
        //     sellsword.atk_base_physics = i16::MAX;
        //     sellsword.atk_base_magic = i16::MAX;
        //     sellsword.atk_base_fire = i16::MAX;
        //     sellsword.atk_base_thunder = i16::MAX;
        //     sellsword.stamina_consume_rate = 0.;
        // } else {
        //     eprintln!("Sellsword Twinblades not found");
        // }

        // if let Some(trousers) = wait_option(|| {
        //     if let Err(e) = params.refresh() {
        //         eprintln!("Error: {:?}", e);
        //     }
        //     params.get_equip_param_protector()
        // })
        // .find(|i| i.id == 23003000)
        // .and_then(|p| p.param)
        // {
        //     trousers.defense_phys = i16::MAX;
        //     trousers.defense_magic = i16::MAX;
        //     trousers.defense_fire = i16::MAX;
        //     trousers.defense_thunder = i16::MAX;
        //     trousers.defense_thunder = i16::MAX;
        //     trousers.defense_slash = i16::MAX;
        //     trousers.defense_blow = i16::MAX;
        //     trousers.defense_thrust = i16::MAX;
        //     trousers.phys_damage_cut_rate = 0.;
        //     trousers.slash_damage_cut_rate = 0.;
        //     trousers.strike_damage_cut_rate = 0.;
        //     trousers.thrust_damage_cut_rate = 0.;
        //     trousers.magic_damage_cut_rate = 0.;
        //     trousers.fire_damage_cut_rate = 0.;
        //     trousers.thunder_damage_cut_rate = 0.;
        // } else {
        //     eprintln!("Assassin Trousers not found");
        // }

        // if let Some(item_lot_param) = wait_option(|| {
        //     if let Err(e) = params.refresh() {
        //         eprintln!("Error: {:?}", e);
        //     }
        //     params.get_item_lot_param()
        // })
        // .find(|i| i.id == 11700000)
        // .and_then(|p| p.param)
        // {
        //     item_lot_param.lot_item_base_point01 = 0;
        //     item_lot_param.lot_item_base_point02 = 1000;
        // }

        // let proof_of_a_concord_kept_ids = [14101006, 14101105, 14101207, 14101306];
        // let human_dregs_ids = [52250003, 52250103, 52260003, 52260103, 52270003,
        // 52270103]; let pale_tongue_ids = [31700005, 31701005];
        // let swordgrass_ids = [12100005, 12100104, 12100204, 12100303];
        // let sunlight_medal_ids = [12801009, 12801108, 12801208, 12801308, 12801407];

        // let vertebra_ids = [11700003, 11700102, 11700203, 11800002, 11800102,
        // 11801102];

        // let mut all_ids = Vec::new();
        // all_ids.extend(proof_of_a_concord_kept_ids);
        // all_ids.extend(human_dregs_ids);
        // all_ids.extend(pale_tongue_ids);
        // all_ids.extend(swordgrass_ids);
        // all_ids.extend(sunlight_medal_ids);
        // all_ids.extend(vertebra_ids);

        // for p in params.get_item_lot_param().unwrap().filter_map(|i| {
        //     if all_ids.contains(&i.id) {
        //         i.param
        //     } else {
        //         None
        //     }
        // }) {
        //     p.lot_item_base_point01 = 0;
        //     p.lot_item_base_point02 = 1000;
        // }

        println!("Done");
    });
}

#[no_mangle]
unsafe extern "stdcall" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT {
    patch();

    (STATE.directinput8create)(hinst, dwversion, riidltf, ppvout, punkouter)
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, reserved: *mut c_void) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => initialize(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    BOOL::from(true)
}
