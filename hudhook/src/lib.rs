pub mod hooks;
pub mod mh;

/// Entry point for the library.
///
/// Example usage:
/// ```
/// pub struct MyRenderLoop;
/// impl RenderLoop for MyRenderLoop {
///   fn render(&self, frame: imgui::Ui) { ... }
/// }
///
/// hudhook!(Box::new(MyRenderLoop::new(...)));
/// ```
#[macro_export]
macro_rules! hudhook {
    ($e:expr) => {
        use hudhook::*;

        use simplelog::*;
        use log::*;
        use log_panics;

        /// Entry point created by the `hudhook` library.
        #[no_mangle]
        pub unsafe extern "stdcall" fn DllMain(
            _: winapi::shared::minwindef::HINSTANCE,
            reason: u32,
            _: *mut winapi::ctypes::c_void,
        ) {
            if reason == 1 {
                simplelog::TermLogger::init(
                    LevelFilter::Trace,
                    Config::default(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto
                ).ok();
                trace!("DllMain()");
                std::thread::spawn(move || {
                    debug!("Started thread, enabling hook...");

                    let status = mh::MH_Initialize();
                    debug!("MH_Initialize: {:?}", status);

                    let mut hook = hooks::dx11::hook_imgui($e);

                    mh::Hook::apply_queue(&mut [&mut hook]);
                });
            }
        }
    };
}
