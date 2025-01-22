use hudhook::inject::Process;
use hudhook::tracing::trace;
use pkg_version::*;
use semver::Version;
use tracing_subscriber::filter::LevelFilter;
use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_YESNO,
};

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
            .map_err(|e| format!("Couldn't check version: {:?}", e))?
            .into_json::<GithubRelease>()
            .map_err(|e| format!("Couldn't check version: {:?}", e))?;

    let version = Version::parse(&release.tag_name).map_err(err_to_string)?;

    Ok((version, release.html_url, release.body))
}

fn perform_injection() -> Result<(), String> {
    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.pop();
    dll_path.push("jdsd_dsiii_practice_tool.dll");

    if !dll_path.exists() {
        dll_path.pop();
        dll_path.push("libjdsd_dsiii_practice_tool");
        dll_path.set_extension("dll");
    }

    let dll_path = dll_path.canonicalize().map_err(err_to_string)?;
    trace!("Injecting {:?}", dll_path);

    Process::by_name("DarkSoulsIII.exe")
        .map_err(|e| format!("Could not find process: {e:?}"))?
        .inject(dll_path)
        .map_err(|e| format!("Could not inject DLL: {e:?}"))?;

    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .init();

    let current_version = get_current_version();

    match get_latest_version() {
        Ok((latest_version, download_url, release_notes)) => {
            if latest_version > current_version {
                let release_notes = match release_notes.find("## What's Changed") {
                    Some(i) => release_notes[..i].trim(),
                    None => &release_notes,
                };
                let update_msg = format!(
                    "A new version of the practice tool is available!\n\nLatest version: \
                     {}\nInstalled version: {}\n\nRelease notes:\n{}\n\nDo you want to download \
                     the update?\0",
                    latest_version, current_version, release_notes
                );

                let msgbox_response = unsafe {
                    MessageBoxA(
                        HWND(0),
                        PCSTR(update_msg.as_str().as_ptr()),
                        PCSTR(c"Update available".as_ptr() as _),
                        MB_YESNO | MB_ICONINFORMATION,
                    )
                };

                if IDYES == msgbox_response {
                    open::that(download_url).ok();
                }
            }
        },
        Err(e) => {
            let error_msg = format!("Unexpected error checking for new version: {}\0", e);
            unsafe {
                MessageBoxA(
                    HWND(0),
                    PCSTR(error_msg.as_str().as_ptr()),
                    PCSTR(c"Error".as_ptr() as _),
                    MB_OK | MB_ICONERROR,
                );
            }
        },
    }

    if let Err(e) = perform_injection() {
        let error_msg = format!("{}\0", e);
        unsafe {
            MessageBoxA(
                HWND(0),
                PCSTR(error_msg.as_str().as_ptr()),
                PCSTR(c"Error".as_ptr() as _),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}
