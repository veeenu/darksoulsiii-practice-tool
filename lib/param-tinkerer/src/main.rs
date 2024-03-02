use hudhook::inject::Process;

fn err_to_string<T: std::fmt::Display>(e: T) -> String {
    format!("Error: {}", e)
}

fn main() -> Result<(), String> {
    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.pop();
    dll_path.push("param-tinkerer.dll");

    let dll_path = dll_path.canonicalize().map_err(err_to_string)?;

    Process::by_name("DarkSoulsIII.exe")
        .map_err(|e| format!("Could not find process: {e:?}"))?
        .inject(dll_path)
        .map_err(|e| format!("Could not inject DLL: {e:?}"))?;

    Ok(())
}
