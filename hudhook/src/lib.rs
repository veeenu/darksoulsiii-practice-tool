#![feature(once_cell)]

pub mod hooks;
pub mod inject;
pub mod mh;

pub mod init {
    pub fn alloc_console() {
        unsafe {
            winapi::um::consoleapi::AllocConsole();
        }
    }

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
}

pub mod deinit {
    pub fn free_console() {
        unsafe {
            winapi::um::wincon::FreeConsole();
        }
    }
}

pub use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
pub use {log, log_panics};

/// Entry point for the library.
///
/// Example usage:
/// ```
/// pub struct MyRenderLoop;
/// impl RenderLoop for MyRenderLoop {
///   fn render(&self, frame: imgui::Ui) { ... }
/// }
///
/// hudhook!(
///     {
///         println!("Initialization code (logging, ...) in this block!");
///         hudhook::init::alloc_console();
///         hudhook::init::simplelog();
///     },
///     // {
///     //     hudhook::deinit::free_console();
///     // }
///     [
///         hudhook::hooks::dx11::hook_imgui(RenderLoop {}),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! hudhook {
    // ($init:block, $deinit:block, $hooks:expr) => {
    ($init:block, $hooks:expr) => {
        use hudhook::*;

        use hudhook::log::*;
        use hudhook::log_panics;

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
                $init

                trace!("DllMain()");
                std::thread::spawn(move || {
                    let hooks = hudhook::mh::Hooks::new(|| { $hooks });
                    HOOKS.set(hooks).ok();
                });
            } else if reason == DLL_PROCESS_DETACH {
                // TODO figure out a way to trigger drops on exit
                // In this branch, logging panics

                // $deinit

                // drop(HOOKS.take());
            }
        }
    };
}
