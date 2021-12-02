// An attempt at hooking DirectInput8 for redirecting input
use std::ffi::c_void;
use std::ptr::null_mut;

use log::*;
use winapi::shared::minwindef::{DWORD, LPDWORD};
use winapi::shared::ntdef::HRESULT;
use windows::core::GUID;
use windows::Win32::Devices::HumanInterfaceDevice::{DIDEVICEOBJECTDATA, DirectInput8Create, GUID_SysKeyboard, IDirectInput8W, IDirectInputDevice8W};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

type GetDeviceData_Type = unsafe extern "system" fn(
    cbObjectData: DWORD,
    rgdod: *mut DIDEVICEOBJECTDATA,
    pwdInOut: LPDWORD,
    dwFlags: DWORD,
) -> HRESULT;

#[allow(non_snake_case)]
#[repr(C)]
struct IDirectInput8WVtbl {
    QueryInterface: *const c_void,
    AddRef: *const c_void,
    Release: *const c_void,
    GetCapabilities: *const c_void,
    EnumObjects: *const c_void,
    GetProperty: *const c_void,
    SetProperty: *const c_void,
    Acquire: *const c_void,
    Unacquire: *const c_void,
    GetDeviceState: *const c_void,
    GetDeviceData: GetDeviceData_Type,
    SetDataFormat: *const c_void,
    SetEventNotification: *const c_void,
    SetCooperativeLevel: *const c_void,
    GetObjectInfo: *const c_void,
    GetDeviceInfo: *const c_void,
    RunControlPanel: *const c_void,
    Initialize: *const c_void,
}

unsafe extern "system" fn get_device_data_impl(
    cbObjectData: DWORD,
    rgdod: *mut DIDEVICEOBJECTDATA,
    pwdInOut: LPDWORD,
    dwFlags: DWORD,
) -> HRESULT {
    debug!("Get device data!");
    0
}

pub unsafe fn get_getdevicedata_addr() -> Option<GetDeviceData_Type> {
    #[allow(non_upper_case_globals)]
    const IID_IDirectInput8W: GUID = GUID {
        data1: winapi::um::dinput::IID_IDirectInput8W.Data1,
        data2: winapi::um::dinput::IID_IDirectInput8W.Data2,
        data3: winapi::um::dinput::IID_IDirectInput8W.Data3,
        data4: winapi::um::dinput::IID_IDirectInput8W.Data4,
    };

    let mut ppvout: *mut c_void = null_mut();

    if let Err(e) = DirectInput8Create(
        GetModuleHandleW(None),
        0x800,
        &IID_IDirectInput8W as *const _,
        &mut ppvout as *mut _,
        None,
    ) {
        debug!("{:?}", e);
        return None;
    }

    let di = (ppvout as *const IDirectInput8W).as_ref();
    if di.is_none() {
        debug!("Null IDirectInput8W");
    }
    let di = di?;
    return None;

    let mut device: Option<IDirectInputDevice8W> = None;
    if let Err(e) = di.CreateDevice(&GUID_SysKeyboard as *const _, &mut device as *mut _, None) {
        debug!("{:?}", e);
        return None;
    }

    let device = (&device.unwrap()) as *const _ as *const c_void;
    debug!("Device = {:p}", device);

    let vtbl = (*(device as *const *const c_void)) as *const IDirectInput8WVtbl;

    match vtbl.as_ref() {
        None => {
            debug!("Null vtable");
            None
        }
        Some(ptr) => Some(ptr.GetDeviceData),
    }
}
