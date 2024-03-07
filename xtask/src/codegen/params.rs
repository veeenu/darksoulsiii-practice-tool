use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use anyhow::{bail, Context};
use practice_tool_tasks::params::{checkout_paramdex, codegen_param_names};

use crate::{project_root, Result};

pub(crate) fn codegen() -> Result<()> {
    checkout_paramdex()?;
    run_python_script()?;
    codegen_param_names("target/Paramdex/DS3/Names", "lib/libds3/src/params/param_names.json")?;

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
        .context("python")?;

    if !cmd.status.success() {
        eprintln!("{}", std::str::from_utf8(&cmd.stderr).unwrap());
        bail!("python codegen failed");
    }

    File::create(project_root().join("lib/libds3/src/params/param_data.rs"))?
        .write_all(&cmd.stdout)?;

    Ok(())
}
