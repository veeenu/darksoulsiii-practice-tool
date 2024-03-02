use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::{cargo_command, project_root, Result};

pub(crate) fn dist() -> Result<()> {
    let status = cargo_command("build")
        .args(["--locked", "--release"])
        .env("CARGO_XTASK_DIST", "true")
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    let status = cargo_command("build")
        .args(["--locked", "--release", "--package", "no-logo"])
        .env("CARGO_XTASK_DIST", "true")
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

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
    let status = cargo_command("build")
        .args(["--locked", "--release"])
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

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
