use std::ffi::OsString;
use std::os::windows::prelude::OsStringExt;
use std::sync::LazyLock;

#[derive(Clone, Copy)]
pub enum Version {
    Ver104,
    Ver108,
    Ver112,
    Ver115,
}

pub static VERSION: LazyLock<Option<Version>> = LazyLock::new(|| unsafe { detect_version() });

unsafe fn vercmp(ptr: *const [u16; 4], ver: &str) -> bool {
    if let Some(ver_mem) = ptr.as_ref().map(|s| &s[..]).map(OsString::from_wide) {
        ver_mem == ver
    } else {
        false
    }
}

/// # Safety
///
/// This must be only executed within a running instance of the DarkSoulsIII.exe
/// process. The addresses are static and available from the get-go so there is
/// no risk of crashes.
pub unsafe fn detect_version() -> Option<Version> {
    const VERSION_PTR_104: *mut [u16; 4] = 0x14288C422usize as _;
    const VERSION_PTR_108: *mut [u16; 4] = 0x1428D3F92usize as _;
    const VERSION_PTR_112: *mut [u16; 4] = 0x1428FD262usize as _;
    const VERSION_PTR_115: *mut [u16; 4] = 0x142900782usize as _;

    if vercmp(VERSION_PTR_104, "1.04") {
        Some(Version::Ver104)
    } else if vercmp(VERSION_PTR_108, "1.08") {
        Some(Version::Ver108)
    } else if vercmp(VERSION_PTR_112, "1.12") {
        Some(Version::Ver112)
    } else if vercmp(VERSION_PTR_115, "1.15") {
        Some(Version::Ver115)
    } else {
        None
    }
}
