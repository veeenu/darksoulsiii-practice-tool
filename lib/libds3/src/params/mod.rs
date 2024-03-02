mod param_data;
use std::collections::{BTreeMap, HashMap};
use std::ffi::c_void;
use std::time::Duration;
use std::{mem, thread};

use log::{error, info};
use once_cell::sync::Lazy;
pub use param_data::*;
use parking_lot::RwLock;
use widestring::U16CStr;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{VirtualQuery, MEMORY_BASIC_INFORMATION, PAGE_READWRITE};

use crate::prelude::base_addresses::*;
use crate::version::VERSION;
use crate::{wait_option, ParamVisitor};

pub static PARAMS: Lazy<RwLock<Params>> = Lazy::new(|| unsafe {
    wait_option(|| match Params::new() {
        Ok(p) => Some(RwLock::new(p)),
        Err(e) => {
            info!("Waiting on memory: {}", e);
            None
        },
    })
});

pub static PARAM_NAMES: Lazy<HashMap<String, HashMap<usize, String>>> =
    Lazy::new(|| serde_json::from_str(include_str!("param_names.json")).unwrap());

#[repr(C)]
struct ParamMaster {
    unk1: [u64; 2],
    start: *const *const ParamEntry,
    end: *const *const ParamEntry,
}

#[repr(C)]
union ParamName {
    indirect: *const [u16; 90], // size: 8
    direct: [u16; 8],           // size: 16
}

#[repr(C)]
struct ParamEntry {
    address: *const c_void,
    _unk1: u64,
    param_name: ParamName,
    param_length: u64,
}

#[derive(Debug)]
#[repr(C)]
struct ParamEntryOffset {
    param_id: u64,
    param_offset: isize,
    _unk1: u64,
}

#[derive(Debug)]
pub struct Param<T: 'static> {
    pub id: u64,
    pub param: Option<&'static mut T>,
}

pub struct Params(BTreeMap<String, (*const c_void, isize)>);
unsafe impl Send for Params {}
unsafe impl Sync for Params {}

impl Params {
    unsafe fn new() -> Result<Params, String> {
        let mut p = Params(BTreeMap::new());
        p.refresh()?;

        Ok(p)
    }

    /// # Safety
    ///
    /// Accesses raw pointers. Should never crash as the param pointers are
    /// static.
    pub unsafe fn refresh(&mut self) -> Result<(), String> {
        let addresses: BaseAddresses = (*VERSION).into();
        let module_base_addr = GetModuleHandleA(None).map_err(|e| e.to_string())?.0 as usize;
        let base_ptr = addresses.param + module_base_addr;
        let base_ptr = loop {
            let base_ptr = *(base_ptr as *const *const c_void) as usize;
            let mut memory_basic_info = MEMORY_BASIC_INFORMATION::default();

            VirtualQuery(
                Some(base_ptr as _),
                &mut memory_basic_info,
                mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            if memory_basic_info.Protect.contains(PAGE_READWRITE) {
                break base_ptr;
            }
            thread::sleep(Duration::from_millis(500));
        };

        let base: &ParamMaster = (base_ptr as *const ParamMaster) // std::ptr::read(base_ptr as *const *const ParamMaster)
            .as_ref()
            .ok_or_else(|| "Invalid param base address".to_string())?;

        let m = Params::param_entries_from_master(base)?;
        self.0 = m;
        Ok(())
    }

    unsafe fn param_entries_from_master(
        base: &ParamMaster,
    ) -> Result<BTreeMap<String, (*const c_void, isize)>, String> {
        let count = base.end.offset_from(base.start);

        let param_entries: &[*const ParamEntry] =
            std::slice::from_raw_parts(base.start, count as usize);

        let m = param_entries
            .iter()
            .map(|&param_ptr| {
                let e = param_ptr.as_ref().ok_or_else(|| format!("Wrong ptr {:p}", param_ptr))?;
                let ustr = U16CStr::from_slice_truncate(if e.param_length <= 7 {
                    &e.param_name.direct
                } else {
                    e.param_name
                        .indirect
                        .as_ref()
                        .ok_or_else(|| format!("Wrong string ptr: {:p}", e.param_name.indirect))?
                });
                let name = ustr
                    .map_err(|e| format!("{}", e))?
                    .to_string()
                    .map_err(|e| format!("{}", e))?;

                let ptr = param_ptr as *const c_void;
                let ptr = *(ptr.offset(0x68) as *const *const c_void);
                let ptr = *(ptr.offset(0x68) as *const *const c_void);
                let count = *(ptr.offset(0x0a) as *const u16);

                Ok((name, (ptr as _, count as isize)))
            })
            .filter_map(|e: Result<_, String>| {
                if let Err(ref e) = e {
                    error!("{}", e);
                }

                e.ok()
            })
            .collect();

        Ok(m)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    fn get_param_ptr(&self, s: &str) -> Option<(*const c_void, isize)> {
        self.0.get(s).cloned()
    }

    pub fn visit_param_item<T: ParamVisitor>(
        &self,
        param: &str,
        param_idx: usize,
        visitor: &mut T,
    ) {
        if let Some((lambda, ptr)) = PARAM_VTABLE.get(param).and_then(|lambda| {
            unsafe { self.get_param_idx_ptr(param, param_idx) }.map(|v| (lambda, v))
        }) {
            lambda(ptr, visitor);
        }
    }

    /// # Safety
    ///
    /// Accesses raw pointers. Ensure that the param is properly initialized
    /// (e.g. with the params well-formed and loaded into memory) before
    /// invoking.
    pub unsafe fn iter_param_ids(&self, s: &str) -> Option<impl Iterator<Item = u64>> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_entries.iter().map(|ent| ent.param_id))
    }

    /// # Safety
    ///
    /// Accesses raw pointers. Ensure that the param is properly initialized
    /// (e.g. with the params well-formed and loaded into memory) before
    /// invoking.
    ///
    /// This is somewhat expensive as it calculates each param's offset at every
    /// iteration. If you only need the param IDs, use `iter_param_ids`.
    pub unsafe fn iter_param<T: 'static>(&self, s: &str) -> Option<impl Iterator<Item = Param<T>>> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_entries.iter().map(move |ent| Param {
            id: ent.param_id,
            param: (param_ptr.offset(ent.param_offset) as *mut T).as_mut(),
        }))
    }

    /// # Safety
    ///
    /// Accesses raw pointers. Ensure that the param is properly initialized
    /// (e.g. with the params well-formed and loaded into memory) before
    /// invoking.
    unsafe fn get_param_idx_ptr(&self, s: &str, i: usize) -> Option<*const c_void> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        if i >= (count as usize) {
            return None;
        }

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_ptr.offset(param_entries[i].param_offset))
    }

    /// # Safety
    ///
    /// Accesses raw pointers. Ensure that the param is properly initialized
    /// (e.g. with the params well-formed and loaded into memory) before
    /// invoking.
    unsafe fn _get_param_idx<T: 'static>(&self, s: &str, i: usize) -> Option<Param<T>> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        if i >= (count as usize) {
            return None;
        }

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(Param {
            id: param_entries[i].param_id,
            param: (param_ptr.offset(param_entries[i].param_offset) as *mut T).as_mut(),
        })
    }
}
