#![feature(once_cell)]

pub mod hooks;
pub mod inject;
pub mod mh;

pub mod utils {
    /// Allocate a Windows console.
    pub fn alloc_console() {
        unsafe {
            winapi::um::consoleapi::AllocConsole();
        }
    }

    /// Initialize `simplelog` with sane defaults.
    pub fn simplelog() {
        use log::*;
        use simplelog::*;

        TermLogger::init(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .ok();
    }

    /// Free the previously allocated Windows console.
    pub fn free_console() {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
    }
}

pub use log;
pub use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

/// Entry point for the library.
///
/// Example usage:
/// ```
/// pub struct MyRenderLoop;
/// impl RenderLoop for MyRenderLoop {
///   fn render(&self, frame: imgui::Ui, flags: &ImguiRenderLoopFlags) { ... }
/// }
///
/// hudhook!(|| {
///     [hudhook::hooks::dx11::hook_imgui(RenderLoop {}),]
/// });
/// ```
#[macro_export]
macro_rules! hudhook {
    ($hooks:expr) => {
        use hudhook::*;
        use hudhook::log::*;

        use std::lazy::OnceCell;

        /// Entry point created by the `hudhook` library.
        #[no_mangle]
        pub unsafe extern "stdcall" fn DllMain(
            _: winapi::shared::minwindef::HINSTANCE,
            reason: u32,
            _: *mut winapi::ctypes::c_void,
        ) {
            static mut HOOKS: OnceCell<mh::Hooks> = OnceCell::new();

            if reason == DLL_PROCESS_ATTACH {
                trace!("DllMain()");
                std::thread::spawn(move || {
                    let hooks = hudhook::mh::Hooks::new($hooks);
                    HOOKS.set(hooks).ok();
                });
            } else if reason == DLL_PROCESS_DETACH {
                // TODO figure out a way to trigger drops on exit
            }
        }
    };
}
