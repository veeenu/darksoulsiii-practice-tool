pub(crate) mod param_data;
pub(crate) use param_data::*;

use std::collections::HashMap;
use std::ffi::c_void;

use log::error;
use widestring::U16CStr;

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

impl std::fmt::Debug for ParamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{:p}", self.indirect) }
    }
}

#[derive(Debug)]
#[repr(C)]
struct ParamEntry {
    address: *const c_void,
    _unk1: u64,
    param_name: ParamName,
    param_length: u64,
}

#[derive(Debug)]
pub(crate) struct Param<T: 'static> {
    pub(crate) id: u64,
    pub(crate) param: Option<&'static mut T>,
}

pub(crate) struct Params(HashMap<String, (*const c_void, isize)>);

impl Params {
    pub(crate) unsafe fn new() -> Result<Params, String> {
        let base: &ParamMaster = (*(0x144785FE0 as *const *const ParamMaster))
            .as_ref()
            .ok_or_else(|| format!("ParamMaster invalid"))?;
        let count = base.end.offset_from(base.start);

        let param_entries: &[*const ParamEntry] =
            std::slice::from_raw_parts(base.start, count as usize);

        let m = param_entries
            .iter()
            .map(|&param_ptr| {
                let e = (param_ptr as *const ParamEntry)
                    .as_ref()
                    .ok_or_else(|| format!("Wrong ptr {:p}", param_ptr))?;
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

        Ok(Params(m))
    }

    fn get_param_ptr(&self, s: &str) -> Option<(*const c_void, isize)> {
        self.0.get(s).cloned()
    }

    unsafe fn iter_param<T: 'static>(&self, s: &str) -> Option<impl Iterator<Item = Param<T>>> {
        #[derive(Debug)]
        #[repr(C)]
        struct ParamEntry {
            param_id: u64,
            param_offset: isize,
            _unk1: u64,
        }
        let (param_ptr, count) = self.get_param_ptr(s)?;

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntry;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_entries.iter().map(move |ent| Param {
            id: ent.param_id,
            param: (param_ptr.offset(ent.param_offset) as *mut T).as_mut(),
        }))
    }
}

fn print_hex<T: Sized>(ptr: *const T) {
    let ptr = ptr as *const u8;

    let bytes: Vec<u8> = (0..std::mem::size_of::<T>())
        .map(|i| unsafe { *ptr.offset(i as _) })
        .collect();

    bytes.chunks(16).for_each(|bs| {
        for i in bs {
            print!("{:02x} ", i);
        }

        print!("  ");

        for _ in bs.len()..16 {
            print!("  ");
        }

        for i in bs {
            let c = *i as char;
            print!("{}", if c.is_ascii() { c } else { '.' });
        }

        println!("");
    });
}
