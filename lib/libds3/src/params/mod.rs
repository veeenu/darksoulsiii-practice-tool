mod param_data;
use imgui::{ChildWindow, ListBox, Selectable};
pub use param_data::*;

use std::collections::BTreeMap;
use std::ffi::c_void;
use std::fmt::Write;
use std::lazy::SyncLazy;
use std::sync::{Mutex, RwLock};

use log::{error, info};
use widestring::U16CStr;

use crate::version::{Version, VERSION};
use crate::wait_option;

pub static PARAMS: SyncLazy<Params> = SyncLazy::new(|| unsafe {
    wait_option(|| match Params::new() {
        Ok(p) => Some(p),
        Err(e) => {
            info!("Waiting on memory: {}", e);
            None
        }
    })
});

trait RenderableParam {
    fn render_imgui(&mut self, ui: &imgui::Ui);
}

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

const fn param_ptr(v: Version) -> usize {
    match v {
        Version::Ver104 => 0x1446E2A80,
        Version::Ver108 => 0x144749DD0,
        Version::Ver112 => 0x144780660,
        Version::Ver115 => 0x144785FE0,
    }
}

#[derive(Default)]
struct ParamUIState {
    selected_param: usize,
    selected_id: usize,
}

pub struct Params(
    RwLock<BTreeMap<String, (*const c_void, isize)>>,
    Mutex<ParamUIState>,
);
unsafe impl Send for Params {}
unsafe impl Sync for Params {}

impl Params {
    unsafe fn new() -> Result<Params, String> {
        let p = Params(RwLock::new(BTreeMap::new()), Mutex::new(Default::default()));
        p.refresh()?;

        Ok(p)
    }

    pub unsafe fn refresh(&self) -> Result<(), String> {
        let version = VERSION.ok_or_else(|| String::from("Couldn't detect version"))?;

        let base: &ParamMaster = std::ptr::read(param_ptr(version) as *const *const ParamMaster)
            .as_ref()
            .ok_or_else(|| format!("Invalid param base address"))?;

        let m = Params::param_entries_from_master(base)?;
        *self.0.write().unwrap() = m;
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

        Ok(m)
    }

    fn get_param_ptr(&self, s: &str) -> Option<(*const c_void, isize)> {
        self.0.read().unwrap().get(s).cloned()
    }

    pub unsafe fn iter_param_ids(&self, s: &str) -> Option<impl Iterator<Item = u64>> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_entries.iter().map(|ent| ent.param_id))
    }

    unsafe fn iter_param<T: 'static>(&self, s: &str) -> Option<impl Iterator<Item = Param<T>>> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_entries.iter().map(move |ent| Param {
            id: ent.param_id,
            param: (param_ptr.offset(ent.param_offset) as *mut T).as_mut(),
        }))
    }

    unsafe fn get_param_idx_ptr(&self, s: &str, i: usize) -> Option<*const c_void> {
        let (param_ptr, count) = self.get_param_ptr(s)?;

        if i >= (count as usize) {
            return None;
        }

        let vec_ptr = param_ptr.offset(0x40) as *const ParamEntryOffset;
        let param_entries = std::slice::from_raw_parts(vec_ptr, count as usize);

        Some(param_ptr.offset(param_entries[i].param_offset) as *const c_void)
    }

    pub unsafe fn get_param_idx<T: 'static>(&self, s: &str, i: usize) -> Option<Param<T>> {
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

    pub fn render_imgui(&self, ui: &imgui::Ui) {
        let mut state = self.1.lock().unwrap();
        let params = self.0.read().unwrap();

        ChildWindow::new("##param_child_wnd")
            .size([250. * 3., 250.])
            .build(ui, || {
                ui.columns(3, "##param_columns", false);

                ListBox::new("##param_names")
                    .size([240., 240.])
                    .build(ui, || {
                        for (idx, k) in params.keys().enumerate() {
                            if Selectable::new(k)
                                .selected(idx == state.selected_param)
                                .build(ui)
                            {
                                state.selected_param = idx;
                            }
                        }
                    });

                ui.next_column();
                ui.set_current_column_width(130.);
                let selected_key = params.keys().nth(state.selected_param);

                if let Some(param_entries) =
                    selected_key.and_then(|k| unsafe { self.iter_param_ids(&k) })
                {
                    let mut buf = String::new();
                    ListBox::new("##param_ids")
                        .size([120., 240.])
                        .build(ui, || {
                            for (idx, id) in param_entries.enumerate() {
                                write!(buf, "{}", id).ok();
                                if Selectable::new(&buf)
                                    .selected(idx == state.selected_id)
                                    .build(ui)
                                {
                                    state.selected_id = idx;
                                }
                                buf.clear();
                            }
                        });
                }

                ui.next_column();
                ui.set_current_column_width(370.);

                if let Some(key) = selected_key {
                    if let Some(param_item) =
                        unsafe { self.get_param_idx_ptr(key, state.selected_id) }
                    {
                        if let Some(lambda) = RENDER_VTABLE.get(key) {
                            ListBox::new("##param_detail")
                                .size([360., 240.])
                                .build(ui, || {
                                    lambda(param_item, ui);
                                });
                        }
                    }
                };
            });

        drop(params);
    }
}

unsafe fn get_render_lambda<T: 'static + RenderableParam>(
) -> Box<dyn Fn(*const c_void, &imgui::Ui) + Send + Sync> {
    Box::new(|ptr, ui| {
        if let Some(r) = (ptr as *mut T).as_mut() {
            r.render_imgui(ui);
        }
    })
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
