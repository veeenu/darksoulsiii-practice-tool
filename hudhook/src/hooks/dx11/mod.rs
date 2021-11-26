mod state_backup;

use crate::mh;

use std::ffi::c_void;
use std::ptr::null_mut;

use imgui_dx11::check_hresult;
use log::*;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND};
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;
use winapi::Interface;

type DXGISwapChainPresentType = unsafe extern "system" fn(
    This: *mut IDXGISwapChain,
    SyncInterval: UINT,
    Flags: UINT,
) -> HRESULT;

type WndProcType =
    unsafe extern "system" fn(hwnd: HWND, umsg: UINT, wparam: WPARAM, lparam: LPARAM) -> isize;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data structures and traits
////////////////////////////////////////////////////////////////////////////////////////////////////

trait Renderer {
    /// Invoked once per frame.
    fn render(&mut self);
}

pub trait ImguiRenderLoop {
    fn render(&mut self, ui: &mut imgui_dx11::imgui::Ui);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Global singletons
////////////////////////////////////////////////////////////////////////////////////////////////////

static TRAMPOLINE: OnceCell<DXGISwapChainPresentType> = OnceCell::new();

////////////////////////////////////////////////////////////////////////////////////////////////////
// Hook entry points
////////////////////////////////////////////////////////////////////////////////////////////////////

static mut IMGUI_RENDER_LOOP: OnceCell<Mutex<Box<dyn ImguiRenderLoop + Send + Sync>>> =
    OnceCell::new();
static IMGUI_RENDERER: OnceCell<Mutex<Box<ImguiRenderer>>> = OnceCell::new();

unsafe extern "system" fn imgui_dxgi_swap_chain_present_impl(
    p_this: *mut IDXGISwapChain,
    sync_interval: UINT,
    flags: UINT,
) -> HRESULT {
    let trampoline = TRAMPOLINE
        .get()
        .expect("IDXGISwapChain::Present trampoline uninitialized");

    let mut renderer = IMGUI_RENDERER
        .get_or_init(|| {
            let mut dev: *mut ID3D11Device = null_mut();
            let mut dev_ctx: *mut ID3D11DeviceContext = null_mut();

            check_hresult((*p_this).GetDevice(&ID3D11Device::uuidof(), &mut dev as *mut _ as _));
            (*dev).GetImmediateContext(&mut dev_ctx as _);

            let engine = imgui_dx11::RenderEngine::new_with_ptrs(dev, dev_ctx, &mut *p_this);
            let render_loop = IMGUI_RENDER_LOOP.take().unwrap().into_inner();
            Mutex::new(Box::new(ImguiRenderer {
                engine,
                render_loop,
            }))
        })
        .lock();

    debug!("Hook renderer doing its thing");
    (*renderer).render();

    let m = trampoline(p_this, sync_interval, flags);
    m
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Render loops
////////////////////////////////////////////////////////////////////////////////////////////////////

struct ImguiRenderer {
    engine: imgui_dx11::RenderEngine,
    render_loop: Box<dyn ImguiRenderLoop>,
}

impl ImguiRenderer {
    fn render(&mut self) {
        let state = state_backup::StateBackup::backup(self.engine.dev_ctx());
        println!("ImguiRenderer::render start");

        if let Err(e) = self.engine.render(|ui| self.render_loop.render(ui)) {
            println!("ImGui renderer error: {:?}", e);
        }

        println!("ImguiRenderer::render end");
        state.restore(self.engine.dev_ctx());
    }
}

unsafe impl Send for ImguiRenderer {}
unsafe impl Sync for ImguiRenderer {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Function address finders
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Get the `IDXGISwapChain::Present` function address.
///
/// Creates a swap chain + device instance and looks up its
/// vtable to find the address.
fn get_present_addr() -> LPVOID {
    let hwnd = {
        let hinstance =
            unsafe { winapi::um::libloaderapi::GetModuleHandleA(std::ptr::null::<i8>()) };
        let wnd_class = WNDCLASSA {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcA),
            hInstance: hinstance,
            lpszClassName: "HUDHOOK_DUMMY\0".as_ptr() as *const i8,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0 as HICON,
            hCursor: 0 as HICON,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: std::ptr::null::<i8>(),
        };
        unsafe {
            RegisterClassA(&wnd_class);
            CreateWindowExA(
                0,
                "HUDHOOK_DUMMY\0".as_ptr() as _,
                "HUDHOOK_DUMMY\0".as_ptr() as _,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                0,
                0,
                16,
                16,
                0 as HWND,
                0 as HMENU,
                std::mem::transmute(hinstance),
                0 as LPVOID,
            )
        }
    };

    let mut feature_level = D3D_FEATURE_LEVEL_11_0;
    let mut swap_chain_desc: DXGI_SWAP_CHAIN_DESC = unsafe { std::mem::zeroed() };
    let mut p_device: *mut ID3D11Device = null_mut();
    let mut p_context: *mut ID3D11DeviceContext = null_mut();
    let mut p_swap_chain: *mut IDXGISwapChain = null_mut();

    swap_chain_desc.BufferCount = 1;
    swap_chain_desc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    swap_chain_desc.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
    swap_chain_desc.BufferDesc.Scaling = DXGI_MODE_SCALING_UNSPECIFIED;
    swap_chain_desc.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
    swap_chain_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    swap_chain_desc.OutputWindow = hwnd;
    swap_chain_desc.SampleDesc.Count = 1;
    swap_chain_desc.Windowed = 1;

    let result = unsafe {
        D3D11CreateDeviceAndSwapChain(
            std::ptr::null_mut::<IDXGIAdapter>(),
            D3D_DRIVER_TYPE_HARDWARE,
            0 as HMODULE,
            0u32,
            &mut feature_level as *mut D3D_FEATURE_LEVEL,
            1,
            D3D11_SDK_VERSION,
            &mut swap_chain_desc as *mut DXGI_SWAP_CHAIN_DESC,
            &mut p_swap_chain as *mut *mut IDXGISwapChain,
            &mut p_device as *mut *mut ID3D11Device,
            null_mut(),
            &mut p_context as *mut *mut ID3D11DeviceContext,
        )
    };

    if result < 0 {
        panic!("D3D11CreateDeviceAndSwapChain failed {:x}", result);
    }

    let ret = unsafe { (*(*p_swap_chain).lpVtbl).Present };

    unsafe {
        (*p_device).Release();
        (*p_context).Release();
        (*p_swap_chain).Release();
        DestroyWindow(hwnd);
    }

    ret as LPVOID
}

pub unsafe fn hook_imgui<T: 'static>(t: T) -> mh::Hook
where
    T: ImguiRenderLoop + Send + Sync,
{
    let dxgi_swap_chain_present_addr = get_present_addr();
    debug!(
        "IDXGISwapChain::Present = {:p}",
        dxgi_swap_chain_present_addr
    );

    let mut trampoline = null_mut();

    debug!(
        "MH_CreateHook: {:?}",
        mh::MH_CreateHook(
            dxgi_swap_chain_present_addr,
            imgui_dxgi_swap_chain_present_impl as *mut c_void,
            &mut trampoline as *mut _ as _
        )
    );

    IMGUI_RENDER_LOOP.get_or_init(|| Mutex::new(Box::new(t)));
    TRAMPOLINE.get_or_init(|| std::mem::transmute(trampoline));

    mh::Hook::new(
        dxgi_swap_chain_present_addr,
        imgui_dxgi_swap_chain_present_impl as *mut c_void,
    )
}
