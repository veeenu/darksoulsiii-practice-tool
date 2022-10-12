#![feature(once_cell)]

use std::ffi::c_void;
use std::sync::LazyLock;

use libds3::prelude::*;
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

fn patch() {
    no_logo();

    std::thread::spawn(|| {
        let mut params = PARAMS.write();

        if let Some(shortsword) = wait_option(|| unsafe {
            params.refresh().ok();
            params.get_equip_param_weapon()
        })
        .find(|i| i.id == 2000000)
        .and_then(|p| p.param)
        {
            shortsword.atk_base_physics = i16::MAX;
            shortsword.atk_base_magic = i16::MAX;
            shortsword.atk_base_fire = i16::MAX;
            shortsword.atk_base_thunder = i16::MAX;
            shortsword.stamina_consume_rate = 0.;
            shortsword.wepmotion_category = 50;
            println!("{:?}", shortsword);
        }

        if let Some(item_lot_param) = wait_option(|| unsafe {
            params.refresh().ok();
            params.get_item_lot_param()
        })
        .find(|i| i.id == 11700000)
        .and_then(|p| p.param)
        {
            item_lot_param.lot_item_base_point01 = 0;
            item_lot_param.lot_item_base_point02 = 1000;
        }
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
