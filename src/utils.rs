use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use glutin::platform::windows::EventLoopExtWindows;
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

mod clipboard {
  use clipboard::{ClipboardContext, ClipboardProvider};
  use imgui::{ClipboardBackend, ImStr, ImString};

  pub struct ClipboardSupport(ClipboardContext);

  pub fn init() -> Option<ClipboardSupport> {
    ClipboardContext::new().ok().map(ClipboardSupport)
  }

  impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<ImString> {
      self.0.get_contents().ok().map(|text| text.into())
    }
    fn set(&mut self, text: &ImStr) {
      let _ = self.0.set_contents(text.to_str().to_owned());
    }
  }
}

pub struct System {
  pub event_loop: EventLoop<()>,
  pub display: glium::Display,
  pub imgui: Context,
  pub platform: WinitPlatform,
  pub renderer: Renderer,
  pub font_size: f32,
}

pub fn init(title: &str) -> System {
  let title = match Path::new(&title).file_name() {
    Some(file_name) => file_name.to_str().unwrap(),
    None => title,
  };
  let event_loop = EventLoopExtWindows::new_any_thread();
  let context = glutin::ContextBuilder::new().with_vsync(true);
  let builder = WindowBuilder::new()
    .with_title(title.to_owned())
    .with_inner_size(glutin::dpi::LogicalSize::new(320., 380.));
  let display = Display::new(builder, context, &event_loop).expect("Failed to initialize display");

  let mut imgui = Context::create();
  imgui.set_ini_filename(None);

  if let Some(backend) = clipboard::init() {
    imgui.set_clipboard_backend(Box::new(backend));
  } else {
    eprintln!("Failed to initialize clipboard");
  }

  let mut platform = WinitPlatform::init(&mut imgui);
  {
    let gl_window = display.gl_window();
    let window = gl_window.window();
    platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
  }

  let hidpi_factor = platform.hidpi_factor();
  let font_size = (13.0 * hidpi_factor) as f32;
  imgui.fonts().add_font(&[
    FontSource::DefaultFontData {
      config: Some(FontConfig {
        size_pixels: font_size,
        ..FontConfig::default()
      }),
    },
    /*FontSource::TtfData {
        data: include_bytes!("../../../resources/mplus-1p-regular.ttf"),
        size_pixels: font_size,
        config: Some(FontConfig {
            rasterizer_multiply: 1.75,
            glyph_ranges: FontGlyphRanges::japanese(),
            ..FontConfig::default()
        }),
    },*/
  ]);

  imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

  let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

  System {
    event_loop,
    display,
    imgui,
    platform,
    renderer,
    font_size,
  }
}

impl System {
  pub fn main_loop<F: FnMut(&mut bool, &mut Ui, &Display) + 'static>(self, mut run_ui: F) {
    let System {
      event_loop,
      display,
      mut imgui,
      mut platform,
      mut renderer,
      ..
    } = self;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
      Event::NewEvents(_) => {
        let now = Instant::now();
        imgui.io_mut().update_delta_time(now - last_frame);
        last_frame = now;
      }
      Event::MainEventsCleared => {
        let gl_window = display.gl_window();
        platform
          .prepare_frame(imgui.io_mut(), gl_window.window())
          .expect("Failed to prepare frame");
        gl_window.window().request_redraw();
      }
      Event::RedrawRequested(_) => {
        let mut ui = imgui.frame();

        let mut run = true;
        run_ui(&mut run, &mut ui, &display);
        if !run {
          *control_flow = ControlFlow::Exit;
        }

        let gl_window = display.gl_window();
        let mut target = display.draw();
        target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
        platform.prepare_render(&ui, gl_window.window());
        let draw_data = ui.render();
        renderer
          .render(&mut target, draw_data)
          .expect("Rendering failed");
        target.finish().expect("Failed to swap buffers");
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      event => {
        let gl_window = display.gl_window();
        platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
      }
    })
  }
}

pub fn imgui_loop<F: FnMut(&mut bool, &mut Ui, &Display) + 'static>(title: &str, run_ui: F) {
  let system = init(title);
  system.main_loop(run_ui)
}

#[test]
fn test_entry_point() {
  let system = init(file!());
  system.main_loop(move |_, ui, _| {
    Window::new(im_str!("Hello world"))
      .size([300.0, 110.0], Condition::FirstUseEver)
      .build(ui, || {
        ui.text(im_str!("Hello world!"));
        ui.text(im_str!("こんにちは世界！"));
        ui.text(im_str!("This...is...imgui-rs!"));
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
          "Mouse Position: ({:.1},{:.1})",
          mouse_pos[0], mouse_pos[1]
        ));
      });
  });
}
