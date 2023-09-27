use std::fmt::Write;
use std::ptr::null_mut;

use libds3::memedit::PointerChain;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Memory::{
    VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
};
use windows::Win32::System::Threading::GetCurrentProcess;

use super::Widget;
use crate::debug;
use crate::util::KeyState;

#[derive(Debug)]
pub(crate) struct Target {
    ptr: PointerChain<[u8; 7]>,
    hotkey: KeyState,
    patch_data: [u8; 7],
    code_cave_data: [u8; 22],
    data: u64,
    code_cave_addr: *mut u8,
}

unsafe impl Send for Target {}
unsafe impl Sync for Target {}

impl Target {
    pub(crate) fn new(ptr: PointerChain<u64>, hotkey: KeyState) -> Self {
        let ptr = ptr.cast();
        let mut ptr_addr = ptr.eval().unwrap() as usize;
        let mut code_cave_addr = loop {
            debug!("Allocating near {:x}", ptr_addr);
            let c = unsafe {
                VirtualAlloc(
                    Some(ptr_addr as *mut _),
                    0x20,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            debug!("Allocated {c:p}");
            if c.is_null() {
                debug!("{:?}", unsafe { GetLastError() });
                ptr_addr += 65536;
            } else {
                break c as *mut u8;
            }
        };
        Target {
            ptr,
            hotkey,
            patch_data: [0xE9, 0x00, 0x00, 0x00, 0x00, 0x90, 0x90],
            code_cave_data: [
                0x48, 0xa3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8b, 0x80, 0x90,
                0x1f, 0x00, 0x00, 0xe9, 0x00, 0x00, 0x00, 0x00,
            ],
            data: 0,
            code_cave_addr,
        }
    }

    fn get_value(&mut self) {
        let patch_address = self.ptr.eval().unwrap();
        let patch_target = (self.code_cave_addr as isize - patch_address as isize - 5) as i32;
        let ret_target = (patch_address as isize - self.code_cave_addr as isize - 15) as i32;
        let data_ptr = (&self.data as *const u64) as usize;

        for i in 0..4 {
            self.patch_data[i + 1] = ((patch_target >> (i * 8)) & 0xff) as u8;
            self.code_cave_data[i + 18] = ((ret_target >> (i * 8)) & 0xff) as u8;
        }

        for i in 0..8 {
            self.code_cave_data[i + 2] = ((data_ptr >> (i * 8)) & 0xff) as u8;
        }

        unsafe {
            std::ptr::copy(
                self.code_cave_data.as_ptr(),
                self.code_cave_addr,
                self.code_cave_data.len(),
            );
        }

        self.ptr.write(self.patch_data);
    }
}

impl Widget for Target {
    fn render(&mut self, ui: &imgui::Ui) {
        let scale = super::scaling_factor(ui);

        if ui.button_with_size("Target", [super::BUTTON_WIDTH * scale, super::BUTTON_HEIGHT]) {
            self.get_value();
        }

        let mut s = String::new();
        for b in &self.patch_data[..] {
            write!(s, "{b:02x} ");
        }
        ui.text(s);

        let mut s = String::new();
        for b in &self.code_cave_data[0..10] {
            write!(s, "{b:02x} ");
        }
        ui.text(s);

        let mut s = String::new();
        for b in &self.code_cave_data[10..18] {
            write!(s, "{b:02x} ");
        }
        ui.text(s);

        let mut s = String::new();
        for b in &self.code_cave_data[18..] {
            write!(s, "{b:02x} ");
        }
        ui.text(s);

        ui.text(format!("D: {:016x}", self.data));
    }
}
