use libjdsd_dsiii_practice_tool::config::*;
use libjdsd_dsiii_practice_tool::utils::imgui_loop;

use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

use clap::{App, Arg};
use imgui::*;
use pkg_version::*;

fn show_error(s: String) {
  let s = imgui::ImString::new(s);
  imgui_loop(move |_, ui, display| {
    ui.text(&s);
  });
}

fn editor(config: Config) {
  // let s = imgui::ImString::new(format!("{:#?}", s));
  let editor = Editor::new(config);

  let mut vkeycodes = VK_SYMBOL_MAP.iter().map(|(_, v)| *v).collect::<Vec<_>>();
  vkeycodes.sort_unstable();
  let vkeycodes = vkeycodes;
  let vkstrings = VK_SYMBOL_MAP
    .iter()
    .map(|(k, v)| (*v, Cow::from(ImString::new(k))))
    .collect::<HashMap<_, _>>();

  imgui_loop(move |_, ui, display| {
    let (ww, wh) = {
      let s = display.gl_window().window().inner_size();
      (s.width as f32, s.height as f32)
    };

    let (width, height, x, y) = if ww > wh {
      (ww * 0.5, wh, ww * 0.5, 0.)
    } else {
      (ww, wh * 0.5, 0., wh * 0.5)
    };

    let stack_token = ui.push_style_vars({
      &[
        StyleVar::WindowRounding(0.),
        StyleVar::FrameBorderSize(0.),
        StyleVar::WindowBorderSize(0.),
      ]
    });

    Window::new(im_str!("window1"))
      .position([0., 0.], Condition::Always)
      .size([width, height], Condition::Always)
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
      })
      .build(ui, || {
        {
          let combo = ComboBox::new(im_str!("Log level"));
          let levels = &[
            log::Level::Warn,
            log::Level::Error,
            log::Level::Info,
            log::Level::Debug,
            log::Level::Trace,
          ];
          let mut level = levels
            .iter()
            .position(|&x| editor.borrow().settings.log_level == x)
            .unwrap();
          combo.build_simple(ui, &mut level, levels, &|i| {
            Cow::from(ImString::new(format!("{}", i)))
          });
          editor.set_log_level(levels[level]);
        }

        {
          let combo = ComboBox::new(im_str!("Interact hotkey"));
          let mut hk = vkeycodes
            .iter()
            .position(|v| editor.borrow().settings.interact == *v)
            .unwrap_or(0);
          combo.build_simple(ui, &mut hk, &vkeycodes, &|i| {
            Cow::from(ImString::new(
              VK_INV_SYMBOL_MAP
                .get(&i)
                .map(|s| s.clone())
                .unwrap_or_else(String::new),
            ))
          });
          // editor.set_log_level(levels[level]);
        }
      });
    Window::new(im_str!("window2"))
      .position([x, y], Condition::Always)
      .size([width, height], Condition::Always)
      .flags({
        WindowFlags::NO_DECORATION
          | WindowFlags::NO_COLLAPSE
          | WindowFlags::NO_RESIZE
          | WindowFlags::NO_MOVE
      })
      .build(ui, || {
        ui.text(im_str!("Window 2"));
      });

    stack_token.pop(ui);
  });
}

struct Editor(RefCell<Config>);

impl Editor {
  fn new(c: Config) -> Editor {
    Editor(RefCell::new(c))
  }

  fn borrow(&self) -> Ref<Config> {
    self.0.borrow()
  }

  fn set_log_level(&self, level: log::Level) {
    self.0.borrow_mut().settings.log_level = level;
  }

  fn add_command(&self, cs: CommandSettings) {
    let mut config = self.0.borrow_mut();
    config.command.push(cs);
  }

  fn remove_command(&self, index: usize) {
    let mut config = self.0.borrow_mut();
    config.command.remove(index);
  }
}

fn main() {
  let matches = App::new("Dark Souls III Practice Tool config editor")
    .version(
      format!(
        "{}.{}.{}",
        pkg_version_major!(),
        pkg_version_minor!(),
        pkg_version_patch!()
      )
      .as_str(),
    )
    .author("johndisandonato")
    .about("Validate the correctness of your configuration file")
    .arg(
      Arg::with_name("file")
        .required(true)
        .help("The path of the config file"),
    )
    .get_matches();

  let conf = Config::load_from_file(&std::path::Path::new(matches.value_of("file").unwrap()));

  let text = match conf {
    Ok(conf) => editor(conf),
    Err(e) => show_error(e),
  };
}
