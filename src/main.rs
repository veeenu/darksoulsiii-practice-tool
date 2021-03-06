use std::path::PathBuf;

use hudhook::inject;
use pkg_version::*;
use semver::Version;
use simplelog::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

fn err_to_string<T: std::fmt::Display>(e: T) -> String {
  format!("Error: {}", e)
}

fn get_current_version() -> Version {
  Version {
    major: pkg_version_major!(),
    minor: pkg_version_minor!(),
    patch: pkg_version_patch!(),
    pre: vec![],
    build: vec![],
  }
}

fn get_latest_version() -> Result<(Version, String, String), String> {
  #[derive(serde::Deserialize)]
  struct GithubRelease {
    tag_name: String,
    html_url: String,
    body: String,
  }

  let release =
    ureq::get("https://api.github.com/repos/veeenu/darksoulsiii-practice-tool/releases/latest")
      .call()
      .into_json_deserialize::<GithubRelease>()
      .map_err(|e| format!("Couldn't check version: {:?}", e))?;

  let version = Version::parse(&release.tag_name).map_err(err_to_string)?;

  Ok((version, release.html_url, release.body))
}

fn perform_injection() -> Result<(), String> {
  let mut cur_exe = PathBuf::new();
  cur_exe.push(std::env::current_exe().map_err(err_to_string)?);
  cur_exe.push("..");
  cur_exe.push("jdsd_dsiii_practice_tool");
  cur_exe.set_extension("dll");

  if !cur_exe.exists() {
    cur_exe.push("..");
    cur_exe.push("libjdsd_dsiii_practice_tool");
    cur_exe.set_extension("dll");
  }

  let cur_dll = cur_exe.canonicalize().map_err(err_to_string)?;
  log::trace!("Injecting {:?}", cur_dll);

  inject("DarkSoulsIII.exe", &cur_dll).map_err(err_to_string)?;

  Ok(())
}

fn main() {
  CombinedLogger::init(vec![TermLogger::new(
    LevelFilter::Trace,
    Config::default(),
    TerminalMode::Mixed,
  )])
  .ok();
  let current_version = get_current_version();

  match get_latest_version() {
    Ok((latest_version, download_url, release_notes)) => {
      if latest_version > current_version {
        let update_msg = format!(
          "A new version of the practice tool is available!\n\nLatest version: {}\nInstalled version: {}\n\nRelease notes:\n{}\n\nDo you want to download the update?\0",
          latest_version, current_version, release_notes
        );

        let msgbox_response = unsafe {
          MessageBoxA(
            0 as HWND,
            update_msg.as_str().as_ptr() as _,
            "Update available\0".as_ptr() as _,
            MB_YESNO | MB_ICONINFORMATION,
          )
        };

        if IDYES == msgbox_response {
          open::that(download_url).ok();
        }
      }
    }
    Err(e) => {
      let error_msg = format!("Unexpected error checking for new version: {}\0", e);
      unsafe {
        MessageBoxA(
          0 as HWND,
          error_msg.as_str().as_ptr() as _,
          "Error\0".as_ptr() as _,
          MB_OK | MB_ICONERROR,
        );
      }
    }
  }

  if let Err(e) = perform_injection() {
    let error_msg = format!("{}\0", e);
    unsafe {
      MessageBoxA(
        0 as HWND,
        error_msg.as_str().as_ptr() as _,
        "Error\0".as_ptr() as _,
        MB_OK | MB_ICONERROR,
      );
    }
  }
}
