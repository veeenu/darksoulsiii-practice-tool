use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::{project_root, Result};

pub(crate) fn dist() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .env("CARGO_XTASK_DIST", "true")
        .args(["build", "--locked", "--release"])
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .env("CARGO_XTASK_DIST", "true")
        .args(["build", "--locked", "--release", "--package", "no-logo"])
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    update_icon(
        project_root().join("target/release/jdsd_dsiii_practice_tool.exe"),
        project_root().join("practice-tool/src/sidherald.ico"),
    )
    .map_err(|e| format!("Update icon: {}", e))?;

    std::fs::remove_dir_all(dist_dir()).ok();
    std::fs::create_dir_all(dist_dir())?;

    let mut zip = ZipWriter::new(File::create(dist_dir().join("jdsd_dsiii_practice_tool.zip"))?);
    let file_options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut buf: Vec<u8> = Vec::new();

    let mut add_zip = |src: PathBuf, dst: &str| -> Result<()> {
        File::open(src)
            .map_err(|e| format!("{}: Couldn't open file: {}", dst, e))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("{}: Couldn't read file: {}", dst, e))?;
        zip.start_file(dst, file_options)
            .map_err(|e| format!("{}: Couldn't start zip file: {}", dst, e))?;
        zip.write_all(&buf).map_err(|e| format!("{}: Couldn't write zip: {}", dst, e))?;
        buf.clear();
        Ok(())
    };

    add_zip(
        project_root().join("target/release/jdsd_dsiii_practice_tool.exe"),
        "jdsd_dsiii_practice_tool.exe",
    )?;
    add_zip(
        project_root().join("target/release/libjdsd_dsiii_practice_tool.dll"),
        "jdsd_dsiii_practice_tool.dll",
    )?;
    add_zip(project_root().join("target/release/dinput8nologo.dll"), "dinput8.dll")?;
    add_zip(project_root().join("lib/data/RELEASE-README.txt"), "README.txt")?;
    add_zip(project_root().join("jdsd_dsiii_practice_tool.toml"), "jdsd_dsiii_practice_tool.toml")?;

    Ok(())
}

pub(crate) fn dist_parammod() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(cargo)
        .current_dir(project_root())
        .env("CARGO_XTASK_DIST", "true")
        .args(["build", "--locked", "--release"])
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    std::fs::remove_dir_all(dist_dir()).ok();
    std::fs::create_dir_all(dist_dir())?;

    let mut zip = ZipWriter::new(File::create(dist_dir().join("jdsd_dsiii_param_tinkerer.zip"))?);
    let file_options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut buf: Vec<u8> = Vec::new();

    let mut add_zip = |src: PathBuf, dst: &str| -> Result<()> {
        File::open(src)
            .map_err(|e| format!("{}: Couldn't open file: {}", dst, e))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("{}: Couldn't read file: {}", dst, e))?;
        zip.start_file(dst, file_options)
            .map_err(|e| format!("{}: Couldn't start zip file: {}", dst, e))?;
        zip.write_all(&buf).map_err(|e| format!("{}: Couldn't write zip: {}", dst, e))?;
        buf.clear();
        Ok(())
    };

    add_zip(project_root().join("target/release/param-tinkerer.exe"), "param-tinkerer.exe")?;
    add_zip(
        project_root().join("target/release/jdsd_dsiii_param_tinkerer.dll"),
        "param-tinkerer.dll",
    )?;
    add_zip(project_root().join("target/release/dinput8parammod.dll"), "dinput8.dll")?;
    add_zip(project_root().join("lib/data/PARAM-TINKERER.txt"), "README.txt")?;
    add_zip(project_root().join("lib/param-mod/param-mod.toml"), "param-mod.toml")?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn update_icon(_path: PathBuf, _icon: PathBuf) -> Result<()> {
    unimplemented!("Use a Windows target for this");
}

#[cfg(target_os = "windows")]
fn update_icon(path: PathBuf, icon: PathBuf) -> Result<()> {
    use std::ffi::c_void;
    use std::os::windows::ffi::OsStrExt;

    use windows::core::{w, PCWSTR};
    use windows::Win32::System::LibraryLoader::{
        BeginUpdateResourceW, EndUpdateResourceW, UpdateResourceW,
    };
    use windows::Win32::System::SystemServices::{LANG_ENGLISH, SUBLANG_DEFAULT};
    use windows::Win32::UI::WindowsAndMessaging::RT_ICON;

    #[repr(C, packed)]
    struct GroupHeader {
        reserved: u16,
        r#type: u16,
        count: u16,
        width: u8,
        height: u8,
        ccount: u8,
        reserved1: u8,
        planes: u16,
        bcount: u16,
        bytes: u32,
        offset: u32,
    }

    let mut buf: Vec<u8> = Vec::new();
    File::open(icon)?.read_to_end(&mut buf)?;

    let group_header: &mut GroupHeader =
        unsafe { (buf.as_ptr() as *mut GroupHeader).as_mut().ok_or("Invalid pointer")? };

    let start: usize = group_header.offset as usize;
    let count: usize = group_header.bytes as usize;
    let end: usize = start + count;
    let icon_data = &buf[start..end];

    group_header.offset = 1;

    unsafe {
        let path = path.as_os_str().encode_wide().collect::<Vec<_>>();
        let handle = BeginUpdateResourceW(PCWSTR(path.as_ptr()), false)?;

        let lang = (LANG_ENGLISH | (SUBLANG_DEFAULT << 8)) as u16;
        UpdateResourceW(
            handle,
            RT_ICON,
            PCWSTR(1 as *const u16),
            // MAKELANGID(LANG_ENGLISH, SUBLANG_DEFAULT),
            lang,
            Some(icon_data.as_ptr() as *const c_void),
            count as u32,
        )?;

        UpdateResourceW(
            handle,
            PCWSTR((RT_ICON.0 as usize + 11) as *const u16),
            w!("IDI_ICON"),
            lang,
            Some(buf.as_ptr() as *const c_void),
            std::mem::size_of::<GroupHeader>() as u32,
        )?;

        EndUpdateResourceW(handle, false)?;
    }

    Ok(())
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
