use dll_syringe::process::OwnedProcess;
use dll_syringe::Syringe;

fn err_to_string<T: std::fmt::Display>(e: T) -> String {
    format!("Error: {}", e)
}

fn main() -> Result<(), String> {
    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.pop();
    dll_path.push("param-tinkerer.dll");

    let dll_path = dll_path.canonicalize().map_err(err_to_string)?;

    let process = OwnedProcess::find_first_by_name("DarkSoulsIII.exe")
        .ok_or_else(|| "Could not find process".to_string())?;
    let syringe = Syringe::for_process(process);
    syringe.inject(dll_path).map_err(|e| format!("{e}"))?;

    Ok(())
}
