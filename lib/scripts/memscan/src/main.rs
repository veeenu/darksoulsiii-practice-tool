#![feature(array_chunks)]
use std::ffi::c_void;
use std::mem::size_of;

use memchr::memmem;
use windows::Win32::Foundation::{CloseHandle, CHAR};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

fn szcmp(source: &[CHAR], s: &str) -> bool {
    source.iter().zip(s.chars()).all(|(a, b)| a.0 == b as u8)
}

fn read_base_module_data(pid: u32) -> Option<(usize, Vec<u8>)> {
    let module_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) }.ok()?;
    let mut module_entry =
        MODULEENTRY32 { dwSize: std::mem::size_of::<MODULEENTRY32>() as _, ..Default::default() };

    unsafe { Module32First(module_snapshot, &mut module_entry) };

    loop {
        if szcmp(&module_entry.szModule, "eldenring.exe") {
            let process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid) }.ok()?;
            let mut buf = vec![0u8; module_entry.modBaseSize as usize];
            let mut bytes_read = 0usize;
            unsafe {
                ReadProcessMemory(
                    process,
                    module_entry.modBaseAddr as *mut c_void,
                    buf.as_mut_ptr() as *mut c_void,
                    module_entry.modBaseSize as usize,
                    Some(&mut bytes_read),
                )
            };
            println!("Read {} out of {} bytes", bytes_read, module_entry.modBaseSize);
            unsafe { CloseHandle(process) };
            return Some((module_entry.modBaseAddr as usize, buf));
        }
        if !unsafe { Module32Next(module_snapshot, &mut module_entry).as_bool() } {
            break;
        }
    }
    None
}

fn get_base_module_bytes(proc_name: &str) -> Option<(usize, Vec<u8>)> {
    let process_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.ok()?;
    let mut process_entry =
        PROCESSENTRY32 { dwSize: std::mem::size_of::<PROCESSENTRY32>() as _, ..Default::default() };

    unsafe { Process32First(process_snapshot, &mut process_entry) };

    loop {
        if szcmp(&process_entry.szExeFile, proc_name) {
            println!(
                "{} is {} {}",
                proc_name, process_entry.th32ProcessID, process_entry.th32ModuleID
            );
            return read_base_module_data(process_entry.th32ProcessID);
        }
        if !unsafe { Process32Next(process_snapshot, &mut process_entry).as_bool() } {
            break None;
        }
    }
}

fn first_cstr(v: &[u8]) -> String {
    v.iter().take_while(|&c| c != &0u8).take(256).map(|&c| c as char).collect()
}

struct Module {
    base_addr: usize,
    bytes: Vec<u8>,
}

impl Module {
    fn find_u32(&self, needle: u32) -> impl Iterator<Item = usize> + '_ {
        const N: usize = size_of::<u32>();
        self.bytes.array_chunks::<N>().copied().map(u32::from_le_bytes).enumerate().filter_map(
            move |(pos, haystack_val)| {
                if haystack_val == needle {
                    Some(pos * N)
                } else {
                    None
                }
            },
        )
    }

    fn find_u64(&self, needle: u64) -> impl Iterator<Item = usize> + '_ {
        const N: usize = size_of::<u64>();
        self.bytes.array_chunks::<N>().copied().map(u64::from_le_bytes).enumerate().filter_map(
            move |(pos, haystack_val)| {
                if haystack_val == needle {
                    Some(pos * N)
                } else {
                    None
                }
            },
        )
    }

    fn get_u64(&self, addr: usize) -> Option<u64> {
        let slice: [u8; size_of::<u64>()] =
            self.bytes[addr..addr + size_of::<u64>()].try_into().ok()?;
        Some(u64::from_le_bytes(slice))
    }

    fn find_rtti_desc(&self, addr: usize) -> Option<()> {
        println!("rtti string at {:x} ({:x})", addr, addr + self.base_addr);
        let rtti_type_desc = ((addr - 0x10) & 0xFFFFFFFF) as u32;
        println!(
            "rtti type desc   {:08x} ({:08x})",
            rtti_type_desc,
            rtti_type_desc as usize + self.base_addr
        );

        let rtti_object_locator = self.find_u32(rtti_type_desc).next()? - 0xC;
        let rtti_object_locator_value = self.get_u64(rtti_object_locator).unwrap();
        println!(
            "rtti obj locator {:08x} ({:08x}) value {:x}",
            rtti_object_locator,
            (rtti_object_locator + self.base_addr),
            rtti_object_locator_value
        );

        let rtti_vtable_ptr =
            self.find_u64((rtti_object_locator + self.base_addr) as u64).next()? + 0x8;
        let rtti_vtable_ptr_abs = rtti_vtable_ptr + self.base_addr;
        println!("rtti vtable ptr  {:08x} ({:08x})", rtti_vtable_ptr, rtti_vtable_ptr_abs,);

        // let rtti_vtable_base = self.get_u64(rtti_vtable_ptr + 0x8)?;
        // println!("rtti vtable base {:x}", rtti_vtable_base);

        for instance_addr in self.find_u64(rtti_vtable_ptr_abs as _) {
            println!("Instance at {:x}", instance_addr);
        }

        None
    }

    fn find_all(&self, class_name: &str) {
        let mut pattern = b".?AV".to_vec();
        pattern.append(&mut class_name.as_bytes().to_vec());

        for i in memmem::find_iter(&self.bytes, &pattern) {
            println!("\n{}", first_cstr(&self.bytes[i..]));
            self.find_rtti_desc(i);
        }
    }
}

fn main() {
    let (addr, buf) = get_base_module_bytes("eldenring.exe").unwrap();

    println!("addr {:x}", addr);

    let pattern = std::env::args().nth(1).expect("Usage: <class name>");

    let m = Module { base_addr: addr, bytes: buf };
    m.find_all(&pattern);
}
