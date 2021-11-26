// Mandatory reference:
// https://www.codeslow.com/2019/12/tiny-windows-executable-in-rust.html

#![no_main]

use std::{
    ffi::c_void,
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
};

use imgui::{Condition, Context, Window};
use imgui_dx11::{check_hresult, RenderEngine};
use winapi::{
    shared::{
        guiddef::REFIID,
        minwindef::{LPARAM, LPVOID, LRESULT, UINT, WPARAM},
        ntdef::HRESULT,
        windef::{HBRUSH, HICON, HMENU, HWND},
    },
    um::{
        dxgidebug::{IDXGIInfoQueue, DXGI_DEBUG_ALL, DXGI_INFO_QUEUE_MESSAGE},
        libloaderapi::{
            GetModuleHandleA, GetProcAddress, LoadLibraryExA, LOAD_LIBRARY_SEARCH_SYSTEM32,
        },
        winuser::{
            BeginPaint, CreateWindowExA, DefWindowProcA, DispatchMessageA, DrawTextA, EndPaint,
            GetClientRect, GetMessageA, PostQuitMessage, RegisterClassA, TranslateMessage,
            CS_HREDRAW, CS_OWNDC, CS_VREDRAW, DT_CENTER, DT_SINGLELINE, DT_VCENTER, WM_QUIT,
            WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    Interface,
};

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    let hinstance = unsafe { GetModuleHandleA(0 as *const i8) };
    let wnd_class = WNDCLASSA {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: hinstance,
        lpszClassName: "MyClass\0".as_ptr() as *const i8,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: 0 as HICON,
        hCursor: 0 as HICON,
        hbrBackground: 0 as HBRUSH,
        lpszMenuName: 0 as *const i8,
    };
    unsafe { RegisterClassA(&wnd_class) };
    let handle = unsafe {
        CreateWindowExA(
            0,                                 // dwExStyle
            "MyClass\0".as_ptr() as *const i8, // class we registered.
            "MiniWIN\0".as_ptr() as *const i8, // title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,  // dwStyle
            // size and position
            100,
            100,
            640,
            480,
            0 as HWND,  // hWndParent
            0 as HMENU, // hMenu
            hinstance,  // hInstance
            0 as LPVOID,
        )
    }; // lpParam

    let mut ctx = Context::create();
    ctx.set_ini_filename(None);
    ctx.io_mut().display_size = [640., 480.];

    let mut renderer = RenderEngine::new(handle, ctx);

    let mut diq: *mut IDXGIInfoQueue = null_mut();

    #[allow(non_snake_case)]
    let DXGIGetDebugInterface: unsafe extern "system" fn(REFIID, *mut *mut c_void) -> HRESULT = unsafe {
        let module = LoadLibraryExA(
            "dxgidebug.dll\0".as_ptr() as _,
            null_mut(),
            LOAD_LIBRARY_SEARCH_SYSTEM32,
        );
        std::mem::transmute(GetProcAddress(
            module,
            "DXGIGetDebugInterface\0".as_ptr() as _,
        ))
    };

    check_hresult(unsafe {
        DXGIGetDebugInterface(&IDXGIInfoQueue::uuidof(), &mut diq as *mut _ as _)
    });

    let diq = NonNull::new(diq).expect("Null Debug info queue");
    let diq = unsafe { diq.as_ref() };

    loop {
        unsafe {
            for i in 0..diq.GetNumStoredMessages(DXGI_DEBUG_ALL) {
                let mut msg_len: usize = 0;
                check_hresult(diq.GetMessage(DXGI_DEBUG_ALL, i, null_mut(), &mut msg_len as _));
                let diqm = vec![0u8; msg_len];
                let pdiqm = diqm.as_ptr() as *mut DXGI_INFO_QUEUE_MESSAGE;
                check_hresult(diq.GetMessage(DXGI_DEBUG_ALL, i, pdiqm, &mut msg_len as _));
                let diqm = pdiqm.as_ref().unwrap();
                println!(
                    "{}",
                    String::from_utf8_lossy(std::slice::from_raw_parts(
                        diqm.pDescription as *const u8,
                        diqm.DescriptionByteLength
                    ))
                );
            }
            diq.ClearStoredMessages(DXGI_DEBUG_ALL);
        }

        if let Err(e) = renderer.render(|ui| {
            Window::new("Hello world")
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text("Hello world!");
                    ui.text("こんにちは世界！");
                    ui.text("This...is...imgui-rs!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        }) {
            eprintln!("{}", e);
        }

        if !handle_message(handle) {
            break;
        }
    }
}

//
// Winapi things
//

fn handle_message(window: HWND) -> bool {
    unsafe {
        let mut msg = MaybeUninit::uninit();
        if GetMessageA(msg.as_mut_ptr(), window, 0, 0) > 0 {
            TranslateMessage(msg.as_ptr());
            DispatchMessageA(msg.as_ptr());
            msg.as_ptr()
                .as_ref()
                .map(|m| m.message != WM_QUIT)
                .unwrap_or(true)
        } else {
            false
        }
    }
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    match msg {
        winapi::um::winuser::WM_PAINT => {
            let mut paint_struct = MaybeUninit::uninit();
            let mut rect = MaybeUninit::uninit();
            let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
            GetClientRect(hwnd, rect.as_mut_ptr());
            DrawTextA(
                hdc,
                "Test\0".as_ptr() as *const i8,
                -1,
                rect.as_mut_ptr(),
                DT_SINGLELINE | DT_CENTER | DT_VCENTER,
            );
            EndPaint(hwnd, paint_struct.as_mut_ptr());
        }
        winapi::um::winuser::WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcA(hwnd, msg, wParam, lParam);
        }
    }
    return 0;
}