mod codegen;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, iter};

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
    let status = cargo_command("build")
        .args(["--lib", "--package", "darksoulsiii-practice-tool"])
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

    inject(iter::once(dll_path))?;

    Ok(())
}

fn run_param_tinkerer() -> Result<()> {
    let status = cargo_command("build")
        .args(["--release", "--lib", "--package", "param-tinkerer"])
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

    inject(iter::once(dll_path))?;

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

fn cargo_command(cmd: &'static str) -> Command {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let mut command = Command::new(cargo);
    command.current_dir(project_root());
    if cfg!(windows) {
        command.arg(cmd);
    } else {
        command.args(["xwin", cmd, "--target", "x86_64-pc-windows-msvc"]);
    }
    command
}

fn inject<S: AsRef<OsStr>>(args: impl Iterator<Item = S>) -> Result<()> {
    cargo_command("run")
        .args(["--release", "--bin", "inject", "--"])
        .args(args)
        .status()
        .map_err(|e| format!("cargo: {}", e))?;
    Ok(())
}
