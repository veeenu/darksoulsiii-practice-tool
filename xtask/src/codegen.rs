use std::{env, io::Write};
use std::fs::File;
use std::process::Command;

use crate::{project_root, Result};

pub(crate) fn codegen() -> Result<()> {
    checkout_paramdex()?;
    run_python_script()?;

    Ok(())
}

fn run_python_script() -> Result<()> {
    let python = env::var("PYTHON").unwrap_or_else(|_| "python".to_string());
    let cmd = Command::new(python)
        .current_dir(project_root().join("target"))
        .args(&[
            project_root().join("xtask/src/codegen/codegen.py"),
            project_root().join("target/Paramdex"),
            project_root().join("xtask"),
        ])
        .output()
        .map_err(|e| format!("python: {}", e))?;

    if !cmd.status.success() {
        return Err("python codegen failed".into());
    }

    File::create(project_root().join("lib/libds3/src/params/param_data.rs"))?
        .write_all(&cmd.stdout)?;

    Ok(())
}

fn checkout_paramdex() -> Result<()> {
    let git = env::var("GIT").unwrap_or_else(|_| "git".to_string());

    if project_root().join("target/Paramdex").exists() {
        let status = Command::new(&git)
            .current_dir(project_root().join("target"))
            .args(&["fetch"])
            .status()
            .map_err(|e| format!("git: {}", e))?;

        if !status.success() {
            return Err("git fetch failed".into());
        }

        let status = Command::new(&git)
            .current_dir(project_root().join("target"))
            .args(&["pull"])
            .status()
            .map_err(|e| format!("git: {}", e))?;

        if !status.success() {
            return Err("git pull failed".into());
        }
    } else {
        let status = Command::new(&git)
            .current_dir(project_root().join("target"))
            .args(&["clone", "https://github.com/soulsmods/Paramdex.git"])
            .status()
            .map_err(|e| format!("git: {}", e))?;

        if !status.success() {
            return Err("git clone failed".into());
        }
    }

    Ok(())
}
