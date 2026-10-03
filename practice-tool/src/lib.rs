// johndisandonato's Dark Souls III Practice Tool
// Copyright (C) 2022-2024  johndisandonato <https://github.com/veeenu>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod config;
mod config_editor;
mod dlc_ownership;
mod icons;
mod practice_tool;
pub mod update;
mod util;
mod widgets;

use std::ffi::c_void;
use std::time::{Duration, Instant};
use std::{env, mem, ptr, thread};

use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::tracing::{error, trace};
use hudhook::{eject, Hudhook};
use libds3::pointers::POINTER_CHAINS;
use once_cell::sync::Lazy;
use practice_tool::PracticeTool;
use practice_tool_core_windows::xinput::hook_xinput;
use windows::core::{s, w, GUID, HRESULT, PCWSTR};
use windows::Win32::Foundation::{
    GetLastError, ERROR_ALREADY_EXISTS, HANDLE, HINSTANCE, MAX_PATH, WAIT_OBJECT_0,
};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject, INFINITE};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_RSHIFT};
use windows::Win32::UI::Input::XboxController::XINPUT_STATE;

/// Created by the copy of the tool that starts: while it exists, the tool is
/// running.
pub const RUNNING_EVENT: PCWSTR = w!("Local\\jdsd_dsiii_practice_tool_running");
/// Set by the exe to start the copy that the game loaded at startup.
pub const START_EVENT: PCWSTR = w!("Local\\jdsd_dsiii_practice_tool_start");

type FDirectInput8Create = unsafe extern "system" fn(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT;

static DIRECTINPUT8CREATE: Lazy<FDirectInput8Create> = Lazy::new(|| unsafe {
    let mut dinput8_path = [0u16; MAX_PATH as usize];
    let count = GetSystemDirectoryW(Some(&mut dinput8_path)) as usize;

    // If count == 0, this will be fun
    ptr::copy_nonoverlapping(w!("\\dinput8.dll").0, dinput8_path[count..].as_mut_ptr(), 12);

    let dinput8 = LoadLibraryW(PCWSTR(dinput8_path.as_ptr())).unwrap();
    let directinput8create = mem::transmute::<
        Option<unsafe extern "system" fn() -> isize>,
        FDirectInput8Create,
    >(GetProcAddress(dinput8, s!("DirectInput8Create")));

    apply_no_logo();
    apply_license_check_patch();

    directinput8create
});

#[no_mangle]
unsafe extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT {
    (DIRECTINPUT8CREATE)(hinst, dwversion, riidltf, ppvout, punkouter)
}

/// Zeroes the stick axes within the deadzone, before the game sees them.
fn apply_deadzone(state: &mut XINPUT_STATE) {
    const DEADZONE: i16 = 64;

    if (-DEADZONE..=DEADZONE).contains(&state.Gamepad.sThumbLX) {
        state.Gamepad.sThumbLX = 0;
    }
    if (-DEADZONE..=DEADZONE).contains(&state.Gamepad.sThumbLY) {
        state.Gamepad.sThumbLY = 0;
    }
    if (-DEADZONE..=DEADZONE).contains(&state.Gamepad.sThumbRX) {
        state.Gamepad.sThumbRX = 0;
    }
    if (-DEADZONE..=DEADZONE).contains(&state.Gamepad.sThumbRY) {
        state.Gamepad.sThumbRY = 0;
    }
}

fn apply_no_logo() {
    POINTER_CHAINS.no_logo.write([
        0x48, 0x31, 0xC0, 0x48, 0x89, 0x02, 0x49, 0x89, 0x04, 0x24, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90,
    ]);
}

fn apply_license_check_patch() {
    if let Err(e) = dlc_ownership::hook() {
        error!("License check patch not applied: {e}");
    }
}

fn start_practice_tool(hmodule: HINSTANCE) {
    let practice_tool = PracticeTool::new();

    if let Err(e) = Hudhook::builder()
        .with::<ImguiDx11Hooks>(practice_tool)
        .with_hmodule(hmodule)
        .build()
        .apply()
    {
        error!("Couldn't apply hooks: {e:?}");
        eject();
    }
}

/// Waits until right shift is held for 2 seconds within the first 10 seconds,
/// or until the exe sets the start event. Returns `false` if waiting on the
/// event fails.
fn await_start(start_event: HANDLE) -> bool {
    let duration_threshold = Duration::from_secs(2);
    let check_window = Duration::from_secs(10);
    let poll_interval_ms = 100;

    let start_time = Instant::now();
    let mut key_down_start: Option<Instant> = None;

    while start_time.elapsed() < check_window {
        let state = unsafe { GetAsyncKeyState(VK_RSHIFT.0 as i32) };
        let key_down = state < 0;

        match (key_down, key_down_start) {
            (true, None) => {
                key_down_start = Some(Instant::now());
            },
            (true, Some(start)) => {
                if start.elapsed() >= duration_threshold {
                    return true;
                }
            },
            (false, _) => {
                key_down_start = None;
            },
        }

        if unsafe { WaitForSingleObject(start_event, poll_interval_ms) } == WAIT_OBJECT_0 {
            return true;
        }
    }

    unsafe { WaitForSingleObject(start_event, INFINITE) == WAIT_OBJECT_0 }
}

fn env_start_requested() -> bool {
    if env::var("DOLL_SKIP").ok().map(|s| s == "consistent").unwrap_or(false) {
        thread::sleep(Duration::from_millis(2000));
        true
    } else {
        false
    }
}

/// Marks the tool as running in this process. Returns `false` if another copy
/// already did.
fn claim_running() -> bool {
    // The handle is never closed: the event must exist for as long as the
    // process.
    match unsafe { CreateEventW(None, true, false, RUNNING_EVENT) } {
        Ok(_) => unsafe { GetLastError() != ERROR_ALREADY_EXISTS },
        Err(e) => {
            error!("Couldn't create running event: {e:?}");
            false
        },
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(hmodule: HINSTANCE, reason: u32, reserved: *mut c_void) {
    if reason == DLL_PROCESS_ATTACH {
        trace!("DllMain()");
        Lazy::force(&DIRECTINPUT8CREATE);
        if let Err(e) = hook_xinput("xinput1_3.dll", Some(apply_deadzone)) {
            error!("{e}");
        }

        let hmodule_ptr = hmodule.0 as usize;

        // `reserved` is null when the DLL is loaded dynamically, i.e. injected
        // by the exe: start right away. The running event must be
        // claimed before returning, as the exe checks for it as soon as
        // the injection completes.
        if reserved.is_null() {
            if claim_running() {
                thread::spawn(move || start_practice_tool(HINSTANCE(hmodule_ptr as *mut c_void)));
            }
            return;
        }

        // Otherwise the game loaded it at startup as its `dinput8.dll`: wait to
        // be asked to start. The handle is never closed: the event must
        // exist for as long as the process.
        let start_event = match CreateEventW(None, true, false, START_EVENT) {
            Ok(start_event) => start_event.0 as usize,
            Err(e) => {
                error!("Couldn't create start event: {e:?}");
                return;
            },
        };

        thread::spawn(move || {
            if (env_start_requested() || await_start(HANDLE(start_event as *mut c_void)))
                && claim_running()
            {
                start_practice_tool(HINSTANCE(hmodule_ptr as *mut c_void))
            }
        });
    }
}
