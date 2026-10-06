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
mod dlc_ownership;
mod practice_tool;
mod widgets;

use std::ffi::c_void;

use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::tracing::{error, trace};
use hudhook::{eject, Hudhook};
use libds3::pointers::POINTER_CHAINS;
use libds3::version::check_version;
use once_cell::sync::Lazy;
use pkg_version::*;
use practice_tool::PracticeTool;
use practice_tool_core::update::{Update, Version};
use practice_tool_core_windows::dinput8::{system_direct_input8_create, FDirectInput8Create};
use practice_tool_core_windows::startup::{on_process_attach, Events};
use practice_tool_core_windows::xinput::hook_xinput;
use windows::core::{GUID, HRESULT};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::Input::XboxController::XINPUT_STATE;

/// Events coordinating the copies of the tool and its exe.
pub fn events() -> Events {
    Events::new("jdsd_dsiii_practice_tool")
}

/// Checks GitHub for a newer release of the tool.
pub fn check_update() -> Update {
    Update::check(
        "veeenu/darksoulsiii-practice-tool",
        Version::new(pkg_version_major!(), pkg_version_minor!(), pkg_version_patch!()),
    )
}

static DIRECTINPUT8CREATE: Lazy<FDirectInput8Create> = Lazy::new(|| {
    let directinput8create = system_direct_input8_create();

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

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(
    hmodule: HINSTANCE,
    reason: u32,
    reserved: *mut c_void,
) -> bool {
    if reason == DLL_PROCESS_ATTACH {
        trace!("DllMain()");
        if check_version().is_err() {
            return false;
        }

        Lazy::force(&DIRECTINPUT8CREATE);
        if let Err(e) = hook_xinput("xinput1_3.dll", Some(apply_deadzone)) {
            error!("{e}");
        }

        let hmodule_ptr = hmodule.0 as usize;
        on_process_attach(events(), !reserved.is_null(), move || {
            start_practice_tool(HINSTANCE(hmodule_ptr as *mut c_void))
        });
    }

    true
}
