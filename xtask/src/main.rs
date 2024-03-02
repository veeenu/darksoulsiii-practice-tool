mod codegen;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

mod dist;

type DynError = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, DynError>;

// Main
//

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist::dist()?,
        Some("dist-parammod") => dist::dist_parammod()?,
        Some("inject") => inject(env::args().skip(1).map(String::from))?,
        Some("run") => run()?,
        Some("run-param-tinkerer") => run_param_tinkerer()?,
        Some("codegen") => codegen::codegen()?,
        Some("help") => print_help(),
        _ => print_help(),
    }
    Ok(())
}

// Tasks
//

fn run() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(["build", "--lib", "--package", "darksoulsiii-practice-tool"])
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    let mut buf = String::new();
    File::open(project_root().join("jdsd_dsiii_practice_tool.toml"))?.read_to_string(&mut buf)?;
    File::create(
        project_root().join("target").join("debug").join("jdsd_dsiii_practice_tool.toml"),
    )?
    .write_all(buf.as_bytes())?;

    let dll_path = project_root()
        .join("target")
        .join("debug")
        .join("libjdsd_dsiii_practice_tool.dll")
        .canonicalize()?;

    do_inject("DarkSoulsIII.exe", dll_path)?;

    Ok(())
}

fn inject(mut args: impl Iterator<Item = String>) -> Result<()> {
    let dll = args.next().unwrap();
    let exe = args.next().unwrap();

    do_inject(exe, dll)?;

    Ok(())
}

fn run_param_tinkerer() -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(["build", "--release", "--lib", "--package", "param-tinkerer"])
        .status()
        .map_err(|e| format!("cargo: {}", e))?;

    if !status.success() {
        return Err("cargo build failed".into());
    }

    let dll_path = project_root()
        .join("target")
        .join("release")
        .join("jdsd_dsiii_param_tinkerer.dll")
        .canonicalize()?;

    do_inject("DarkSoulsIII.exe", dll_path)?;

    Ok(())
}

fn print_help() {
    eprintln!(
        r#"
Tasks:

run ........... compile and start the practice tool
dist .......... build distribution artifacts
codegen ....... generate Rust code: parameters, base addresses, ...
help .......... print this help
"#
    );
}

// Utilities
//

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}

#[cfg(windows)]
fn do_inject<S: AsRef<str>, P: AsRef<Path>>(exe: S, dll_path: P) -> Result<()> {
    hudhook::inject::Process::by_name(exe.as_ref())
        .map_err(|e| format!("Could not find process: {e:?}"))?
        .inject(dll_path.as_ref().to_path_buf())?;
    Ok(())
}

#[cfg(not(windows))]
fn do_inject<S: AsRef<str>, P: AsRef<Path>>(_exe: S, _dll_path: P) -> Result<()> {
    unimplemented!();
}
