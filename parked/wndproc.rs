      let mut sd: DXGI_SWAP_CHAIN_DESC = std::mem::zeroed();
      this.GetDesc(&mut sd as *mut _)
      #[allow(clippy::fn_to_numeric_cast)]
      let default_wnd_proc = std::mem::transmute(SetWindowLongPtrA(
        sd.OutputWindow,
        GWLP_WNDPROC,
        wnd_proc as _,
      ));


unsafe extern "system" fn wnd_proc(
  hwnd: HWND,
  umsg: UINT,
  wparam: WPARAM,
  lparam: LPARAM,
) -> isize {
  if let Some(hook) = DXGI_HOOK_STATE.get() {
    let mut hook = hook.lock().unwrap();

    let set_capture = |mouse_down: &[bool], hwnd| {
      let any_down = mouse_down.iter().any(|i| *i);
      if !any_down && GetCapture() == 0 as HWND {
        SetCapture(hwnd);
      }
    };

    let release_capture = |mouse_down: &[bool], hwnd| {
      let any_down = mouse_down.iter().any(|i| *i);
      if !any_down && GetCapture() == hwnd {
        ReleaseCapture();
      }
    };

    match umsg {
      WM_KEYDOWN | WM_SYSKEYDOWN => {
        if wparam < 256 {
          hook.imgui_ctx.io_mut().keys_down[wparam] = true;
        }
      }
      WM_KEYUP | WM_SYSKEYUP => {
        if wparam < 256 {
          hook.imgui_ctx.io_mut().keys_down[wparam] = false;
        }
      }
      WM_LBUTTONDOWN | WM_LBUTTONDBLCLK => {
        // set_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        hook.imgui_ctx.io_mut().mouse_down[0] = true;
        // return 1;
      }
      WM_RBUTTONDOWN | WM_RBUTTONDBLCLK => {
        // set_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        hook.imgui_ctx.io_mut().mouse_down[1] = true;
        // return 1;
      }
      WM_MBUTTONDOWN | WM_MBUTTONDBLCLK => {
        // set_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        hook.imgui_ctx.io_mut().mouse_down[2] = true;
        // return 1;
      }
      WM_XBUTTONDOWN | WM_XBUTTONDBLCLK => {
        let btn = if GET_XBUTTON_WPARAM(wparam) == XBUTTON1 {
          3
        } else {
          4
        };
        // set_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        hook.imgui_ctx.io_mut().mouse_down[btn] = true;
        // return 1;
      }
      WM_LBUTTONUP => {
        hook.imgui_ctx.io_mut().mouse_down[0] = false;
        // release_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        // return 1;
      }
      WM_RBUTTONUP => {
        hook.imgui_ctx.io_mut().mouse_down[1] = false;
        // release_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        // return 1;
      }
      WM_MBUTTONUP => {
        hook.imgui_ctx.io_mut().mouse_down[2] = false;
        // release_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
        // return 1;
      }
      WM_XBUTTONUP => {
        let btn = if GET_XBUTTON_WPARAM(wparam) == XBUTTON1 {
          3
        } else {
          4
        };
        hook.imgui_ctx.io_mut().mouse_down[btn] = false;
        // release_capture(&hook.imgui_ctx.io().mouse_down, hwnd);
      }
      WM_MOUSEWHEEL => {
        hook.imgui_ctx.io_mut().mouse_wheel +=
          (GET_WHEEL_DELTA_WPARAM(wparam) as f32) / (WHEEL_DELTA as f32);
      }
      WM_MOUSEHWHEEL => {
        hook.imgui_ctx.io_mut().mouse_wheel_h +=
          (GET_WHEEL_DELTA_WPARAM(wparam) as f32) / (WHEEL_DELTA as f32);
      }
      WM_CHAR => hook
        .imgui_ctx
        .io_mut()
        .add_input_character(wparam as u8 as char),
      _ => {}
    }

    let wnd_proc = hook.default_wnd_proc;
    drop(hook);

    CallWindowProcW(Some(wnd_proc), hwnd, umsg, wparam, lparam)
  } else {
    debug!("WndProc called before hook was set");
    0
  }
}
