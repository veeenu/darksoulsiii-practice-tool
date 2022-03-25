#![feature(array_chunks)]
use std::ffi::c_void;
use std::path::PathBuf;
use std::ptr::{null, null_mut};

use widestring::U16CString;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, BOOL, CHAR, DBG_CONTINUE};
use windows::Win32::System::Diagnostics::Debug::{
    ContinueDebugEvent, ReadProcessMemory,
    WaitForDebugEventEx, DEBUG_EVENT,
};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
    MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{CreateProcessW, OpenProcess, *};

fn szcmp(source: &[CHAR], s: &str) -> bool {
    source.iter().zip(s.chars()).all(|(a, b)| a.0 == b as u8)
}

fn read_base_module_data(proc_name: &str, pid: u32) -> Option<(usize, Vec<u8>)> {
    let module_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) };
    let mut module_entry = MODULEENTRY32::default();
    module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as _;

    unsafe { Module32First(module_snapshot, &mut module_entry) };

    loop {
        if szcmp(&module_entry.szModule, proc_name) {
            let process = unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid) };
            let mut buf = vec![0u8; module_entry.modBaseSize as usize];
            let mut bytes_read = 0usize;
            unsafe {
                ReadProcessMemory(
                    process,
                    module_entry.modBaseAddr as *mut c_void,
                    buf.as_mut_ptr() as *mut c_void,
                    module_entry.modBaseSize as usize,
                    &mut bytes_read,
                )
            };
            println!(
                "Read {:x} out of {:x} bytes",
                bytes_read, module_entry.modBaseSize
            );
            unsafe { CloseHandle(process) };
            return Some((module_entry.modBaseAddr as usize, buf));
        }
        if !unsafe { Module32Next(module_snapshot, &mut module_entry).as_bool() } {
            break;
        }
    }
    None
}

fn enum_modules(pid: u32) {
    let module_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) };
    let mut module_entry = MODULEENTRY32::default();
    module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as _;

    if unsafe { Module32First(module_snapshot, &mut module_entry) } == BOOL::from(false) {
        println!("Module32First failed: {:x}", unsafe { GetLastError() }.0);
    }

    loop {
        println!(
            "{}",
            module_entry
                .szModule
                .iter()
                .map(|&c| c.0 as char)
                .collect::<String>()
        );
        if !unsafe { Module32Next(module_snapshot, &mut module_entry).as_bool() } {
            break;
        }
    }
}

pub(crate) fn run_exe_and_get_base_module_bytes(exe_path: &PathBuf) -> Option<(usize, Vec<u8>)> {
    let mut process_info = PROCESS_INFORMATION::default();
    let mut startup_info = STARTUPINFOW::default();
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as _;

    let mut exe = U16CString::from_str(exe_path.to_str().unwrap())
        .unwrap()
        .into_vec();
    exe.push(0);

    let process = unsafe {
        CreateProcessW(
            PCWSTR(exe.as_ptr()),
            PWSTR(null_mut()),
            null(),
            null(),
            BOOL::from(false),
            DEBUG_PROCESS | DETACHED_PROCESS,
            null(),
            PCWSTR(null()),
            &mut startup_info,
            &mut process_info,
        )
    };

    if !process.as_bool() {
        eprintln!(
            "Could not create process: {:x}",
            unsafe { GetLastError() }.0
        );
        return None;
    }

    println!(
        "Process handle={:x} pid={}",
        process_info.hProcess.0, process_info.dwProcessId
    );

    let mut debug_event = DEBUG_EVENT::default();

    loop {
        unsafe { WaitForDebugEventEx(&mut debug_event, 1000) };
        unsafe {
            ContinueDebugEvent(
                process_info.dwProcessId,
                process_info.dwThreadId,
                DBG_CONTINUE.0 as _,
            )
        };
        if debug_event.dwDebugEventCode.0 == 2 {
            break;
        }
    }

    let ret = read_base_module_data(
        exe_path.file_name().unwrap().to_str().unwrap(),
        process_info.dwProcessId,
    );

    unsafe { TerminateProcess(process_info.hProcess, 0) };

    ret
}

pub(crate) fn get_base_module_bytes(proc_name: &str) -> Option<(usize, Vec<u8>)> {
    let process_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let mut process_entry = PROCESSENTRY32::default();
    process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as _;

    unsafe { Process32First(process_snapshot, &mut process_entry) };

    loop {
        if szcmp(&process_entry.szExeFile, proc_name) {
            println!(
                "{} is {} {}",
                proc_name, process_entry.th32ProcessID, process_entry.th32ModuleID
            );
            return read_base_module_data(proc_name, process_entry.th32ProcessID);
        }
        if !unsafe { Process32Next(process_snapshot, &mut process_entry).as_bool() } {
            break None;
        }
    }
}
