use std::ffi::CStr;

use hudhook::inject::Process;
use hudhook::tracing::{error, trace};
use libjdsd_dsiii_practice_tool::update::Update;
use libjdsd_dsiii_practice_tool::{RUNNING_EVENT, START_EVENT};
use tracing_subscriber::filter::LevelFilter;
use windows::core::PCSTR;
use windows::Win32::System::Threading::{
    OpenEventW, SetEvent, EVENT_MODIFY_STATE, SYNCHRONIZATION_SYNCHRONIZE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_YESNO, MESSAGEBOX_RESULT,
    MESSAGEBOX_STYLE,
};

fn message_box(caption: &CStr, text: &str, style: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
    let text = format!("{text}\0");
    unsafe { MessageBoxA(None, PCSTR(text.as_ptr()), PCSTR(caption.as_ptr() as _), style) }
}

fn is_running() -> bool {
    unsafe { OpenEventW(SYNCHRONIZATION_SYNCHRONIZE, false, RUNNING_EVENT) }.is_ok()
}

fn load_error(e: impl std::fmt::Display) -> String {
    format!(
        "Could not load the practice tool into Dark Souls III: {e}\n\nMake sure your antivirus \
         isn't blocking it."
    )
}

fn perform_injection() -> Result<(), String> {
    let process = Process::by_name("DarkSoulsIII.exe").map_err(|e| {
        error!("Could not find process: {e:?}");
        "Dark Souls III is not running. Start the game first, then run the practice tool \
         again.\n\nIf the game is running as administrator, run the practice tool as administrator \
         too."
            .to_string()
    })?;

    if is_running() {
        return Err("The practice tool is already running.\n\nTo start a different copy, restart \
                    the game first."
            .to_string());
    }

    let mut dll_path = std::env::current_exe().map_err(load_error)?;
    dll_path.set_file_name("dinput8.dll");

    if !dll_path.exists() {
        return Err(format!(
            "Could not find {}.\n\nExtract all the files from the zip archive before running the \
             practice tool, and make sure your antivirus didn't delete it.",
            dll_path.display()
        ));
    }

    trace!("Injecting {:?}", dll_path);
    process.inject(dll_path).map_err(load_error)?;

    // A freshly loaded DLL is running by now. If it isn't, the game had already
    // loaded this very file at startup, so injecting it did nothing: ask
    // that copy to start instead.
    if !is_running() {
        unsafe { OpenEventW(EVENT_MODIFY_STATE, false, START_EVENT).and_then(|e| SetEvent(e)) }
            .map_err(|_| load_error("the practice tool did not start."))?;
    }

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

    match Update::check() {
        Update::Available { url, notes } => {
            let text =
                format!("{notes}\nDo you want to download it? The practice tool will not start.");
            if message_box(c"Update available", &text, MB_YESNO | MB_ICONINFORMATION) == IDYES {
                open::that(url).ok();
                return;
            }
        },
        // Shown in the overlay.
        Update::Error(e) => error!("Could not check for updates: {e}"),
        Update::UpToDate => {},
    }

    if let Err(e) = perform_injection() {
        message_box(c"Error", &e, MB_OK | MB_ICONERROR);
    }
}
