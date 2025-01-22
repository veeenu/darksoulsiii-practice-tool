use std::ffi::c_void;
use std::mem;

use libds3::prelude::*;
use once_cell::sync::Lazy;
use windows::core::{GUID, HRESULT, PCSTR};
use windows::Win32::Foundation::{BOOL, HINSTANCE};
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

static STATE: Lazy<State> = Lazy::new(|| unsafe {
    let dinput8 = LoadLibraryA(PCSTR(c"C:\\Windows\\System32\\dinput8.dll".as_ptr() as _)).unwrap();
    let directinput8create =
        mem::transmute::<Option<unsafe extern "system" fn() -> isize>, FDirectInput8Create>(
            GetProcAddress(dinput8, PCSTR(c"DirectInput8Create".as_ptr() as _)),
        );
    println!("Called!");

    State { directinput8create }
});

fn initialize() {
    Lazy::force(&STATE);
}

fn patch() {
    let pointer_chains = PointerChains::new();
    pointer_chains
        .no_logo
        .write([
            0x48, 0x31, 0xC0, 0x48, 0x89, 0x02, 0x49, 0x89, 0x04, 0x24, 0x90, 0x90, 0x90, 0x90,
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        ])
        .unwrap();
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
