use std::path::PathBuf;

use practice_tool_core::{
    key::Key,
    widgets::{savefile_manager::SavefileManager, Widget},
};

pub(crate) fn savefile_manager(key_load: Key, key_close: Key) -> Box<dyn Widget> {
    // TODO
    Box::new(SavefileManager::new(Some(key_load), Some(key_close), get_savefile_path().unwrap()))
}

fn get_savefile_path() -> Result<PathBuf, String> {
    let re = regex::Regex::new(r"^[a-f0-9]+$").unwrap();
    let savefile_path: PathBuf =
        [std::env::var("APPDATA").map_err(|e| format!("{}", e))?.as_str(), "DarkSoulsIII"]
            .iter()
            .collect();
    std::fs::read_dir(savefile_path)
        .map_err(|e| format!("{}", e))?
        .filter_map(|e| e.ok())
        .find(|e| re.is_match(&e.file_name().to_string_lossy()) && e.path().is_dir())
        .map(|e| e.path())
        .map(|p| p.join("DS30000.sl2"))
        .ok_or_else(|| String::from("Couldn't find savefile path"))
}
