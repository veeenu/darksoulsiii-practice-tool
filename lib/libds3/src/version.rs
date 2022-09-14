use std::ptr::null_mut;
use std::sync::LazyLock;

use log::*;

use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};
use windows::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleW};

pub use crate::prelude::base_addresses::Version;

pub static VERSION: LazyLock<Version> = LazyLock::new(get_version);

fn get_version() -> Version {
    let file_path = {
        let mut buf = vec![0u16; MAX_PATH as usize];
        unsafe { GetModuleFileNameW(GetModuleHandleW(PCWSTR(null_mut())), &mut buf) };
        U16CString::from_vec_truncate(buf)
    };

    let mut version_info_size =
        unsafe { GetFileVersionInfoSizeW(PCWSTR(file_path.as_ptr()), null_mut()) };
    let mut version_info_buf = vec![0u8; version_info_size as usize];
    unsafe {
        GetFileVersionInfoW(
            PCWSTR(file_path.as_ptr()),
            0,
            version_info_size,
            version_info_buf.as_mut_ptr() as _,
        )
    };

    let mut version_info: *mut VS_FIXEDFILEINFO = null_mut();
    unsafe {
        VerQueryValueW(
            version_info_buf.as_ptr() as _,
            PCWSTR(widestring::U16CString::from_str("\\\\\0").unwrap().as_ptr()),
            &mut version_info as *mut *mut _ as _,
            &mut version_info_size,
        )
    };
    let version_info = unsafe { version_info.as_ref().unwrap() };
    let major = (version_info.dwFileVersionMS >> 16) & 0xffff;
    let minor = (version_info.dwFileVersionMS) & 0xffff;
    let patch = (version_info.dwFileVersionLS >> 16) & 0xffff;

    info!("Version {} {} {}", major, minor, patch);
    Version::from((major, minor, patch))
}

/*
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
*/
