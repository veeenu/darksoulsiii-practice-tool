//! Keeps transient Steam failures from revoking DLC ownership mid-session.
//!
//! From 1.08 on, the game re-queries DLC ownership during gameplay (a
//! watchdog thread every ~16.7s, plus a main-thread check at random
//! intervals). A single spurious `false` from `ISteamApps::BIsSubscribedApp`
//! or `ISteamApps::BIsDlcInstalled` is treated as a revoked license: the game
//! shows the DLC license error and returns to the title screen.
//!
//! Those checks live in Arxan-protected code, which crashes the game minutes
//! after it is modified, so the Steam side is hooked instead: once Steam has
//! reported an app as owned, later `false`s for it are ignored for the rest
//! of the session. DLC that is not owned is still reported as such.

use std::collections::BTreeSet;
use std::ffi::c_void;
use std::mem;
use std::sync::{Mutex, Once, OnceLock, PoisonError};

use hudhook::mh::{MH_ApplyQueued, MH_Initialize, MhHook, MH_STATUS};
use hudhook::tracing::{error, info, warn};
use windows::core::{s, w, PCSTR};
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};

type FSteamApiInit = unsafe extern "C" fn() -> bool;
type FSteamApps = unsafe extern "C" fn() -> *const *const *mut c_void;
type FOwnershipCheck = unsafe extern "system" fn(this: *mut c_void, app_id: u32) -> bool;

// `ISteamApps` vtable slots. Stable across every interface version.
const BIS_SUBSCRIBED_APP: usize = 6;
const BIS_DLC_INSTALLED: usize = 7;

static STEAM_API_INIT: OnceLock<FSteamApiInit> = OnceLock::new();
static IS_SUBSCRIBED_APP: OnceLock<FOwnershipCheck> = OnceLock::new();
static IS_DLC_INSTALLED: OnceLock<FOwnershipCheck> = OnceLock::new();

static OWNED_SUBSCRIBED: Mutex<BTreeSet<u32>> = Mutex::new(BTreeSet::new());
static OWNED_INSTALLED: Mutex<BTreeSet<u32>> = Mutex::new(BTreeSet::new());

/// Hooks `SteamAPI_Init`, which in turn hooks the ownership checks as soon as
/// the `ISteamApps` interface exists. Must run before the game initializes
/// Steam.
pub fn hook() -> Result<(), String> {
    let init = steam_api_proc(s!("SteamAPI_Init"))?;

    unsafe {
        match MH_Initialize() {
            MH_STATUS::MH_ERROR_ALREADY_INITIALIZED | MH_STATUS::MH_OK => {},
            status => return Err(format!("MH_Initialize: {status:?}")),
        }

        let hook = MhHook::new(init, steam_api_init_impl as *mut c_void)
            .map_err(|e| format!("SteamAPI_Init: create: {e:?}"))?;
        let _ = STEAM_API_INIT.set(mem::transmute::<*mut c_void, FSteamApiInit>(hook.trampoline()));
        hook.queue_enable().map_err(|e| format!("SteamAPI_Init: queue enable: {e:?}"))?;
        MH_ApplyQueued().ok_context("SteamAPI_Init: apply queued").map_err(|e| format!("{e:?}"))
    }
}

fn steam_api_proc(name: PCSTR) -> Result<*mut c_void, String> {
    unsafe {
        let module = GetModuleHandleW(w!("steam_api64.dll"))
            .map_err(|e| format!("steam_api64.dll not loaded: {e:?}"))?;
        GetProcAddress(module, name)
            .map(|f| f as *mut c_void)
            .ok_or_else(|| format!("steam_api64.dll export {} not found", name.display()))
    }
}

unsafe extern "C" fn steam_api_init_impl() -> bool {
    static HOOK_OWNERSHIP_CHECKS: Once = Once::new();

    let initialized = (STEAM_API_INIT.get().unwrap())();

    if initialized {
        HOOK_OWNERSHIP_CHECKS.call_once(|| match hook_ownership_checks() {
            Ok(()) => info!("DLC ownership checks hooked"),
            Err(e) => error!("DLC ownership checks not hooked: {e}"),
        });
    }

    initialized
}

unsafe fn hook_ownership_checks() -> Result<(), String> {
    let steam_apps = mem::transmute::<*mut c_void, FSteamApps>(steam_api_proc(s!("SteamApps"))?)();
    if steam_apps.is_null() {
        return Err("SteamApps() returned null".to_string());
    }
    let vtable = *steam_apps;

    for (slot, trampoline, hook_impl) in [
        (BIS_SUBSCRIBED_APP, &IS_SUBSCRIBED_APP, is_subscribed_app_impl as FOwnershipCheck),
        (BIS_DLC_INSTALLED, &IS_DLC_INSTALLED, is_dlc_installed_impl as FOwnershipCheck),
    ] {
        let hook = MhHook::new(*vtable.add(slot), hook_impl as *mut c_void)
            .map_err(|e| format!("slot {slot}: create: {e:?}"))?;
        let _ = trampoline.set(mem::transmute::<*mut c_void, FOwnershipCheck>(hook.trampoline()));
        hook.queue_enable().map_err(|e| format!("slot {slot}: queue enable: {e:?}"))?;
    }

    MH_ApplyQueued().ok_context("apply queued").map_err(|e| format!("{e:?}"))
}

unsafe extern "system" fn is_subscribed_app_impl(this: *mut c_void, app_id: u32) -> bool {
    let owned = (IS_SUBSCRIBED_APP.get().unwrap())(this, app_id);
    sticky_ownership(&OWNED_SUBSCRIBED, "BIsSubscribedApp", app_id, owned)
}

unsafe extern "system" fn is_dlc_installed_impl(this: *mut c_void, app_id: u32) -> bool {
    let owned = (IS_DLC_INSTALLED.get().unwrap())(this, app_id);
    sticky_ownership(&OWNED_INSTALLED, "BIsDlcInstalled", app_id, owned)
}

/// Records apps reported as owned, and keeps reporting them as owned.
fn sticky_ownership(
    known_owned: &Mutex<BTreeSet<u32>>,
    check: &str,
    app_id: u32,
    owned: bool,
) -> bool {
    let mut known_owned = known_owned.lock().unwrap_or_else(PoisonError::into_inner);

    if owned {
        known_owned.insert(app_id);
        true
    } else if known_owned.contains(&app_id) {
        warn!("{check}({app_id}) returned false after returning true; ignoring");
        true
    } else {
        false
    }
}
