// mod codegen;

use std::ffi::OsStr;
use std::fs;
use std::{env, iter};

use anyhow::{bail, Context, Result};

use practice_tool_tasks::{
    cargo_command, project_root, steam_command, target_path, Distribution, FileInstall,
};

const APPID: u32 = 374320;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        Some("dist-param-mod") => dist_param_mod()?,
        // Some("codegen") => codegen::codegen()?,
        Some("inject") => inject(env::args().skip(1).map(String::from))?,
        Some("run") => run()?,
        Some("run-param-tinkerer") => run_param_tinkerer()?,
        Some("install") => install()?,
        Some("uninstall") => uninstall()?,
        Some("help") => print_help(),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"
Tasks:

run ............. compile and start the practice tool
dist ............ build distribution artifacts
codegen ......... generate Rust code: parameters, base addresses, ...
inject <args> ... standalone dll inject
install ......... install standalone dll to $DSIII_PATH
uninstall ....... uninstall standalone dll from $DSIII_PATH
help ............ print this help
"#
    );
}

fn run() -> Result<()> {
    let status = cargo_command("build")
        .args(["--lib", "--package", "darksoulsiii-practice-tool"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    fs::copy(
        project_root().join("jdsd_dsiii_practice_tool.toml"),
        target_path("debug").join("jdsd_dsiii_practice_tool.toml"),
    )?;

    let dll_path = target_path("debug").join("libjdsd_dsiii_practice_tool.dll").canonicalize()?;

    inject(iter::once(dll_path))?;

    Ok(())
}

fn run_param_tinkerer() -> Result<()> {
    let status = cargo_command("build")
        .args(["--release", "--lib", "--package", "param-tinkerer"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    let dll_path = project_root()
        .join("target")
        .join("release")
        .join("jdsd_dsiii_param_tinkerer.dll")
        .canonicalize()?;

    inject(iter::once(dll_path))?;

    Ok(())
}

fn dist() -> Result<()> {
    Distribution::new("jdsd_dsiii_practice_tool.zip")
        .with_artifact("libjdsd_dsiii_practice_tool.dll", "jdsd_dsiii_practice_tool.dll")
        .with_artifact("jdsd_dsiii_practice_tool.exe", "jdsd_dsiii_practice_tool.exe")
        .with_artifact("dinput8nologo.dll", "dinput8.dll")
        .with_file("lib/data/RELEASE-README.txt", "README.txt")
        .with_file("jdsd_dsiii_practice_tool.toml", "jdsd_dsiii_practice_tool.toml")
        .build()
}

fn dist_param_mod() -> Result<()> {
    Distribution::new("jdsd_dsiii_param_tinkerer.zip")
        .with_artifact("jdsd_dsiii_param_tinkerer.dll", "param_tinkerer.dll")
        .with_artifact("param_tinkerer.exe", "param_tinkerer.exe")
        .with_artifact("dinput8parammod.dll", "dinput8.dll")
        .with_file("lib/data/PARAM-TINKERER.txt", "README.txt")
        .with_file("lib/param-mod/param-mod.toml", "param-mod.toml")
        .build()
}

fn install() -> Result<()> {
    let status = cargo_command("build")
        .args(["--lib", "--release", "--package", "darksoulsiii-practice-tool"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    FileInstall::new()
        .with_file(target_path("release").join("libjdsd_dsiii_practice_tool.dll"), "dinput8.dll")
        .with_file(
            project_root().join("jdsd_dsiii_practice_tool.toml"),
            "jdsd_dsiii_practice_tool.toml",
        )
        .install("DSIII_PATH")?;

    Ok(())
}

fn uninstall() -> Result<()> {
    FileInstall::new()
        .with_file(target_path("release").join("libjdsd_dsiii_practice_tool.dll"), "dinput8.dll")
        .with_file(
            project_root().join("jdsd_dsiii_practice_tool.toml"),
            "jdsd_dsiii_practice_tool.toml",
        )
        .uninstall("DSIII_PATH")?;

    Ok(())
}

fn inject<S: AsRef<OsStr>>(args: impl Iterator<Item = S>) -> Result<()> {
    cargo_command("build").args(["--release", "--bin", "inject"]).status().context("cargo")?;

    steam_command(target_path("release").join("inject"), APPID)?
        .args(args)
        .status()
        .context("inject")?;

    Ok(())
}
