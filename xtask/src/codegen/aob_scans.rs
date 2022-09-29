use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::env;
use std::ffi::c_void;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};
use std::sync::LazyLock;

use heck::AsSnakeCase;
use rayon::prelude::*;
use textwrap::dedent;
use widestring::U16CString;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, GetLastError, BOOL, CHAR, DBG_CONTINUE};
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
};
use windows::Win32::System::Diagnostics::Debug::{
    ContinueDebugEvent, ReadProcessMemory, WaitForDebugEventEx, DEBUG_EVENT,
};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE,
};
use windows::Win32::System::Threading::{CreateProcessW, OpenProcess, *};

const AOBS: &[(&str, &str, usize, usize)] = &[
    ("WorldChrMan", "48 8B 1D ?? ?? ?? 04 48 8B F9 48 85 DB ?? ?? 8B 11 85 D2 ?? ?? 8D", 3, 7),
    ("WorldChrManDbg", "48 8B 05 ?? ?? ?? ?? 66 0F 7F 44 24 40 48 85 C0", 3, 7),
    ("MenuMan", "48 89 15 ?? ?? ?? ?? 44 8b 82 ?? ?? ?? ?? 44 8b 8a ?? ?? ?? ?? 48 8b c3", 3, 7),
    ("BaseA", "48 8B 05 ?? ?? ?? ?? 48 85 C0 ?? ?? 48 8b 40 ?? C3", 3, 7),
    ("BaseD", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 26 44 8B", 3, 7),
    ("SprjDebugEvent", "48 8B 05 ?? ?? ?? ?? 41 0F B6 D8 8B EA", 3, 7),
    (
        "Debug",
        "C6 05 ?? ?? ?? ?? 01 48 8B 8C 24 ?? ?? ?? ?? 48 33 CC E8 ?? ?? ?? ?? 4C 8D 9C 24",
        2,
        7,
    ),
    (
        "Grend",
        "C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 C6 05 ?? ?? ?? ?? 00 4C \
         8B 05 ?? ?? ?? ?? 4C 89 44 24 58",
        2,
        7,
    ),
    ("BaseHBD", "48 8B 0D ?? ?? ?? ?? 41 B0 01 E8 ?? ?? ?? ?? 48 8B D3 48 8B CF", 3, 7),
    ("MapItemMan", "48 8B 0D ?? ?? ?? ?? 48 8B 89 ?? ?? ?? ?? E8 ?? ?? ?? ?? E9", 3, 7),
    (
        "SpawnItemFuncPtr",
        // "E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8 2C 00 00 00 48 8B D3",
        "E8 ?? ?? ?? ?? C7 44 24 20 00 01 00 00 4C 8D 4C 24 40 41 B8",
        1,
        5,
    ),
    ("Param", "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 0B 4C 8B C0 48 8B D7", 3, 7)
];

static AOBS_README: LazyLock<Vec<(&str, usize, Vec<&str>)>> =
    LazyLock::new(|| vec![("XA", 3, vec!["48 8B 83 ?? ?? ?? ?? 48 8B 10 48 85 D2 ?? ?? 8B"])]);

static AOBS_DIRECT: LazyLock<Vec<(&str, Vec<&str>)>> = LazyLock::new(|| {
    vec![("FormatString", vec![
        "3C 00 54 00 45 00 58 00 54 00 46 00 4F 00 52 00 4D 00 41 00 54 00",
    ])]
});

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Version(u32, u32, u32);

impl Version {
    fn to_fromsoft_string(self) -> String {
        format!("{}.{:02}.{}", self.0, self.1, self.2)
    }
}

struct VersionData {
    version: Version,
    aobs: Vec<(&'static str, usize)>,
}

fn szcmp(source: &[CHAR], s: &str) -> bool {
    source.iter().zip(s.chars()).all(|(a, b)| a.0 == b as u8)
}

fn into_needle(pattern: &str) -> Vec<Option<u8>> {
    pattern
        .split(' ')
        .map(|byte| match byte {
            "?" | "??" => None,
            x => u8::from_str_radix(x, 16).ok(),
        })
        .collect::<Vec<_>>()
}

fn naive_search(bytes: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    bytes.windows(pattern.len()).position(|wnd| {
        wnd.iter().zip(pattern.iter()).all(|(byte, pattern)| match pattern {
            Some(x) => byte == x,
            None => true,
        })
    })
}

fn read_base_module_data(proc_name: &str, pid: u32) -> Option<(usize, Vec<u8>)> {
    let module_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid) };
    let mut module_entry =
        MODULEENTRY32 { dwSize: std::mem::size_of::<MODULEENTRY32>() as _, ..Default::default() };

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
            println!("Read {:x} out of {:x} bytes", bytes_read, module_entry.modBaseSize);
            unsafe { CloseHandle(process) };
            return Some((module_entry.modBaseAddr as usize, buf));
        }
        if !unsafe { Module32Next(module_snapshot, &mut module_entry).as_bool() } {
            break;
        }
    }
    None
}

fn get_base_module_bytes(exe_path: &Path) -> Option<(usize, Vec<u8>)> {
    let mut process_info = PROCESS_INFORMATION::default();
    let startup_info =
        STARTUPINFOW { cb: std::mem::size_of::<STARTUPINFOW>() as _, ..Default::default() };

    let mut exe = U16CString::from_str(exe_path.to_str().unwrap()).unwrap().into_vec();
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
            &startup_info,
            &mut process_info,
        )
    };

    if !process.as_bool() {
        eprintln!("Could not create process: {:x}", unsafe { GetLastError() }.0);
        return None;
    }

    println!("Process handle={:x} pid={}", process_info.hProcess.0, process_info.dwProcessId);

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

fn find_aobs(bytes: Vec<u8>) -> Vec<(&'static str, usize)> {
    let mut aob_offsets = AOBS
        .into_par_iter()
        .filter_map(|(name, aob, offs_read, offs_final)| {
            if let Some(r) = naive_search(&bytes, &into_needle(aob)) {
                Some((name, r, offs_read, offs_final))
            } else {
                eprintln!("{name:24} not found");
                None
            }
        })
        .map(|(name, offset, c, f)| {
            (
                name,
                offset,
                *f,
                u32::from_le_bytes(bytes[offset + c..offset + c + 4].try_into().unwrap()),
            )
        })
        .map(|(name, offset, f, val)| (*name, (val + f as u32) as usize + offset))
        .collect::<Vec<_>>();

    aob_offsets.sort_by(|a, b| a.0.cmp(b.0));

    //
    let mut aob_offsets_direct = AOBS_DIRECT
        .iter()
        .filter_map(|(name, aob)| {
            if let Some(r) = aob.iter().find_map(|aob| naive_search(&bytes, &into_needle(aob))) {
                Some((*name, r))
            } else {
                eprintln!("{name:24} not found");
                None
            }
        })
        .collect::<Vec<_>>();

    aob_offsets_direct.sort_by(|a, b| a.0.cmp(b.0));

    aob_offsets.extend(aob_offsets_direct.into_iter());

    //
    let mut aob_offsets_readme = AOBS_README
        .iter()
        .filter_map(|(name, offs, aob)| {
            if let Some(r) = aob.iter().find_map(|aob| naive_search(&bytes, &into_needle(aob))) {
                let r = u32::from_le_bytes((&bytes[r + offs..r + offs + 4]).try_into().unwrap());
                Some((*name, r as usize))
            } else {
                eprintln!("{name:24} not found");
                None
            }
        })
        .collect::<Vec<_>>();

    aob_offsets_readme.sort_by(|a, b| a.0.cmp(b.0));

    aob_offsets.extend(aob_offsets_readme.into_iter());

    aob_offsets
}

fn get_file_version(file: &Path) -> Version {
    let mut file_path = file.to_string_lossy().to_string();
    file_path.push(0 as char);
    let file_path = widestring::U16CString::from_str(file_path).unwrap();
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

    Version(major, minor, patch)
}

// Codegen routine
//

/// Generate the `BaseAddresses` struct.
fn codegen_base_addresses_struct() -> String {
    let mut generated = String::new();

    generated.push_str("// **********************************\n");
    generated.push_str("// *** AUTOGENERATED, DO NOT EDIT ***\n");
    generated.push_str("// **********************************\n");

    generated.push_str("#[derive(Debug)]\n");
    generated.push_str("pub struct BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|(name, ..)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str({
        &AOBS_DIRECT
            .iter()
            .map(|(name, _)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join("")
    });
    generated.push_str({
        &AOBS_README
            .iter()
            .map(|(name, ..)| format!("    pub {}: usize,\n", AsSnakeCase(name)))
            .collect::<Vec<_>>()
            .join("")
    });
    generated.push_str("}\n\n");
    generated.push_str("impl BaseAddresses {\n");
    generated.push_str("    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {\n");
    generated.push_str("        BaseAddresses {\n");
    generated.push_str(
        &AOBS
            .iter()
            .map(|(name, ..)| {
                format!("            {}: self.{} + base,\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str(
        &AOBS_DIRECT
            .iter()
            .map(|(name, _)| {
                format!("            {}: self.{} + base,\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str(
        &AOBS_README
            .iter()
            .map(|(name, ..)| {
                format!("            {}: self.{},\n", AsSnakeCase(name), AsSnakeCase(name))
            })
            .collect::<Vec<_>>()
            .join(""),
    );
    generated.push_str("        }\n    }\n}\n\n");
    generated
}

/// Generate `BaseAddresses` instances.
fn codegen_base_addresses_instances(ver: &Version, aobs: &[(&str, usize)]) -> String {
    use std::fmt::Write;
    let mut string = aobs.iter().fold(
        format!(
            "pub const BASE_ADDRESSES_{}_{:02}_{}: BaseAddresses = BaseAddresses {{\n",
            ver.0, ver.1, ver.2
        ),
        |mut o, (name, offset)| {
            writeln!(o, "    {}: 0x{:x},", AsSnakeCase(name), offset).unwrap();
            o
        },
    );
    string.push_str("};\n\n");
    string
}

/// Generate the `Version` enum and `From<Version> for BaseAddresses`.
fn codegen_version_enum(ver: &[VersionData]) -> String {
    use std::fmt::Write;
    let mut string = String::new();

    // pub enum Version

    string.push_str("#[derive(Clone, Copy)]\n");
    string.push_str("pub enum Version {\n");

    for v in ver {
        writeln!(string, "    V{}_{:02}_{},", v.version.0, v.version.1, v.version.2).unwrap();
    }

    string.push_str("}\n\n");

    // impl From<(u32, u32, u32)> for Version

    string.push_str("impl From<(u32, u32, u32)> for Version {\n");
    string.push_str("    fn from(v: (u32, u32, u32)) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            ({maj}, {min}, {patch}) => Version::V{maj}_{min:02}_{patch},"
        )
        .unwrap();
    }

    string.push_str("            (maj, min, patch) => {\n");
    string.push_str(
        "                log::error!(\"Unrecognized version {maj}.{min:02}.{patch}\");\n",
    );
    string.push_str("                panic!()\n");
    string.push_str("            }\n");
    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl From<Version> for (u32, u32, u32)

    string.push_str("impl From<Version> for (u32, u32, u32) {\n");
    string.push_str("    fn from(v: Version) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        writeln!(
            string,
            "            Version::V{maj}_{min:02}_{patch} => ({maj}, {min}, {patch}),"
        )
        .unwrap();
    }

    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    // impl From<Version> for BaseAddresses

    string.push_str("impl From<Version> for BaseAddresses {\n");
    string.push_str("    fn from(v: Version) -> Self {\n");
    string.push_str("        match v {\n");

    for v in ver {
        let Version(maj, min, patch) = v.version;
        let stem = format!("{maj}_{min:02}_{patch}");
        writeln!(string, "            Version::V{stem} => BASE_ADDRESSES_{stem},").unwrap();
    }

    string.push_str("        }\n");
    string.push_str("    }\n");
    string.push_str("}\n\n");

    string
}

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(
        env::var("DSIIIPT_PATCHES_PATH").unwrap_or_else(|_| panic!("{}", dedent(r#"
            DSIIIPT_PATCHES_PATH environment variable undefined.
            Check the documentation: https://github.com/veeenu/darksoulsiii-practice-tool/README.md#building
        "#))),
    );
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("Game").join("DarkSoulsIII.exe"))
}

fn codegen_base_addresses_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
        .join("lib")
        .join("libds3")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub(crate) fn codegen_base_addresses() {
    let mut processed_versions: HashSet<Version> = HashSet::new();

    let mut version_data = patches_paths()
        .filter(|p| p.exists())
        .filter_map(|exe| {
            let version = get_file_version(&exe);
            if processed_versions.contains(&version) {
                None
            } else {
                let exe = exe.canonicalize().unwrap();
                println!("\nVERSION {}: {:?}", version.to_fromsoft_string(), exe);

                let (_base_addr, bytes) = get_base_module_bytes(&exe).unwrap();
                let aobs = find_aobs(bytes);
                processed_versions.insert(version);
                Some(VersionData { version, aobs })
            }
        })
        .collect::<Vec<_>>();

    version_data.sort_by_key(|vd| vd.version);

    let mut codegen = codegen_base_addresses_struct();
    codegen.push_str(&codegen_version_enum(&version_data));

    let codegen = version_data.iter().fold(codegen, |mut o, i| {
        o.push_str(&codegen_base_addresses_instances(&i.version, &i.aobs));
        o
    });

    File::create(codegen_base_addresses_path()).unwrap().write_all(codegen.as_bytes()).unwrap();
}

