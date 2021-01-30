#![windows_subsystem = "windows"]
use libjdsd_dsiii_practice_tool::config::*;
use libjdsd_dsiii_practice_tool::utils::imgui_loop;

use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;

use clap::{App, Arg};
use imgui::*;
use pkg_version::*;

const TITLE: &str = "Practice Tool Config Editor";

fn show_error(s: String) {
  let s = imgui::ImString::new(s);
  imgui_loop(TITLE, move |_, ui, _| {
    ui.text(&s);
  });
}

fn editor(config: Config, path: String) {
  let editor = Editor::new(config, path);

  let mut vkeycodes = VK_SYMBOL_MAP.iter().map(|(_, v)| *v).collect::<Vec<_>>();
  vkeycodes.sort_unstable();
  let vkeycodes = vkeycodes;
  let vkstrings = VK_SYMBOL_MAP
    .iter()
    .map(|(k, v)| (*v, Cow::from(ImString::new(k))))
    .collect::<HashMap<_, _>>();

  let flags = [
    "all_no_damage",
    "no_death",
    "one_shot",
    "inf_stamina",
    "inf_focus",
    "inf_consumables",
    "deathcam",
    "evt_draw",
    "evt_disable",
    "ai_disable",
    "rend_chr",
    "rend_obj",
    "rend_map",
    "rend_mesh_hi",
    "rend_mesh_lo",
    "gravity",
  ]
  .iter()
  .map(|&s| Cow::from(ImString::new(s)))
  .collect::<Vec<_>>();

  let commands = [
    "Toggle",
    "Quitout",
    "Position",
    "Souls",
    "Cycle Speed", // "Spawn Item"
  ]
  .iter()
  .map(|&s| Cow::from(ImString::new(s)))
  .collect::<Vec<_>>();

  let command_factories = [
    || CommandSettings::Toggle {
      flag: String::from("all_no_damage"),
      hotkey: String::from("VK_F1"),
    },
    || CommandSettings::Quitout {
      hotkey: String::from("P"),
    },
    || CommandSettings::Position {
      hotkey_save: String::from("N"),
      hotkey_load: String::from("M"),
    },
    || CommandSettings::Souls {
      quantity: 0,
      hotkey: String::from("VK_F1"),
    },
    || CommandSettings::CycleSpeed {
      values: vec![],
      hotkey: String::from("VK_F1"),
    },
  ];

  let mut cur_cmd = 0usize;

  imgui_loop(TITLE, move |_, ui, display| {
    let (width, height) = {
      let s = display.gl_window().window().inner_size();
      (s.width as f32, s.height as f32)
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
        let mut ididx = 0i32;

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

        let hotkey_combo = |ui: &Ui, label: &ImStr, cur_value: i32| -> Option<i32> {
          let combo = ComboBox::new(label);
          let mut hk = vkeycodes.iter().position(|v| cur_value == *v).unwrap_or(0);
          let combo_changed = combo.build_simple(ui, &mut hk, &vkeycodes, &|i| {
            vkstrings.get(i).unwrap().clone()
          });
          if combo_changed {
            Some(vkeycodes[hk])
          } else {
            None
          }
        };

        {
          let a = hotkey_combo(ui, im_str!("Down"), editor.borrow().settings.down);
          if let Some(a) = a {
            editor.set_down(a);
          }

          let a = hotkey_combo(ui, im_str!("Up"), editor.borrow().settings.up);
          if let Some(a) = a {
            editor.set_up(a);
          }

          let a = hotkey_combo(ui, im_str!("Left"), editor.borrow().settings.left);
          if let Some(a) = a {
            editor.set_left(a);
          }

          let a = hotkey_combo(ui, im_str!("Right"), editor.borrow().settings.right);
          if let Some(a) = a {
            editor.set_right(a);
          }

          let a = hotkey_combo(ui, im_str!("Display"), editor.borrow().settings.display);
          if let Some(a) = a {
            editor.set_display(a);
          }
        }

        ui.dummy([0., 19.]);
        ui.text(im_str!("Commands"));

        editor.iter_commands_mut(|(idx, cmd)| {
          let idtok = ui.push_id(ididx);
          ididx += 1;

          ui.separator();
          ui.columns(4, im_str!("##col0"), false);

          ui.set_column_width(0, f32::max(0., width - 28. * 3. - 3.));
          ui.set_column_width(1, 28.);
          ui.set_column_width(2, 28.);
          ui.set_column_width(3, 28.);

          ui.text(ImString::new(format!("{}", cmd)));
          ui.next_column();

          if ui.arrow_button(im_str!("UP"), Direction::Up) {
            editor.move_command_up(idx);
          }
          ui.next_column();

          if ui.arrow_button(im_str!("DN"), Direction::Down) {
            editor.move_command_down(idx);
          }
          ui.next_column();

          if ui.button(im_str!("x"), [18., 19.]) {
            editor.remove_command(idx);
          }

          ui.columns(1, im_str!("##col1"), false);

          match cmd {
            CommandSettings::Toggle { flag, hotkey } => {
              ui.next_column();
              ui.columns(2, im_str!("##col_toggle"), false);
              ui.set_column_width(0, width * 0.5);
              ui.set_column_width(1, width * 0.5);

              // Flag
              let iwt = ui.push_item_width(-1.);
              {
                let combo = ComboBox::new(im_str!("##Flag"));
                let mut cur_flag = flags.iter().position(|v| flag == v.to_str()).unwrap_or(0);
                if combo.build_simple(ui, &mut cur_flag, &flags, &|i| i.clone()) {
                  *flag = flags[cur_flag].to_string();
                }
              }
              iwt.pop(ui);
              ui.next_column();

              // Hotkey
              let iwt = ui.push_item_width(-1.);
              if let Some(a) = hotkey_combo(
                ui,
                im_str!("##Hotkey"),
                *VK_SYMBOL_MAP.get(hotkey).unwrap_or(&0),
              ) {
                *hotkey = vkstrings[&a].to_string();
              }

              iwt.pop(ui);
              ui.columns(1, im_str!("##col1"), false);
            }
            CommandSettings::Position {
              hotkey_save,
              hotkey_load,
            } => {
              ui.next_column();
              ui.columns(2, im_str!("##col_position"), false);
              ui.set_column_width(0, width * 0.5);
              ui.set_column_width(1, width * 0.5);

              if let Some(a) = hotkey_combo(
                ui,
                im_str!("Save"),
                *VK_SYMBOL_MAP.get(hotkey_save).unwrap_or(&0),
              ) {
                *hotkey_save = vkstrings[&a].to_string();
              }

              ui.next_column();

              if let Some(a) = hotkey_combo(
                ui,
                im_str!("Load"),
                *VK_SYMBOL_MAP.get(hotkey_load).unwrap_or(&0),
              ) {
                *hotkey_load = vkstrings[&a].to_string();
              }

              ui.columns(1, im_str!("##col1"), false);
            }
            CommandSettings::Souls { quantity, hotkey } => {
              ui.input_int(im_str!("Quantity"), quantity).build();

              if let Some(a) = hotkey_combo(
                ui,
                im_str!("Hotkey"),
                *VK_SYMBOL_MAP.get(hotkey).unwrap_or(&0),
              ) {
                *hotkey = vkstrings[&a].to_string();
              }
            }
            CommandSettings::CycleSpeed { values: _, hotkey } => {
              if let Some(a) = hotkey_combo(
                ui,
                im_str!("Hotkey"),
                *VK_SYMBOL_MAP.get(hotkey).unwrap_or(&0),
              ) {
                *hotkey = vkstrings[&a].to_string();
              }
            }
            CommandSettings::SpawnItem { .. } => {}
            CommandSettings::Quitout { hotkey } => {
              if let Some(a) = hotkey_combo(
                ui,
                im_str!("Hotkey"),
                *VK_SYMBOL_MAP.get(hotkey).unwrap_or(&0),
              ) {
                *hotkey = vkstrings[&a].to_string();
              }
            }
          }

          idtok.pop(ui);
        });

        editor.process_operation();

        ui.separator();
        {
          ui.columns(2, im_str!(""), false);
          ui.set_column_width(1, 176.);
          ui.set_column_width(0, 96.);

          if ui.button(im_str!("Add command"), [0., 0.]) {
            // *flag = flags[cur_flag].to_string();
            editor.add_command(command_factories[cur_cmd]());
          }
          ui.next_column();

          let combo = ComboBox::new(im_str!("##"));
          combo.build_simple(ui, &mut cur_cmd, &commands, &|i| Cow::Borrowed(i));
          ui.columns(1, im_str!(""), false);
        }

        ui.separator();
        if ui.button(im_str!("Save"), [0., 0.]) {
          editor.save();
        }
      });

    stack_token.pop(ui);
  });
}

enum Operation {
  SwapCommand(usize, usize),
  RemoveCommand(usize),
}

struct Editor(RefCell<Config>, RefCell<Option<Operation>>, String);

impl Editor {
  fn new(c: Config, f: String) -> Editor {
    Editor(RefCell::new(c), RefCell::new(None), f)
  }

  fn borrow(&self) -> Ref<Config> {
    self.0.borrow()
  }

  fn iter_commands_mut<F: FnMut((usize, &mut CommandSettings))>(&self, f: F) {
    self
      .0
      .borrow_mut()
      .command
      .iter_mut()
      .enumerate()
      .for_each(f);
  }

  fn set_log_level(&self, level: log::Level) {
    self.0.borrow_mut().settings.log_level = level;
  }

  fn set_down(&self, hk: i32) {
    self.0.borrow_mut().settings.down = hk;
  }

  fn set_up(&self, hk: i32) {
    self.0.borrow_mut().settings.up = hk;
  }

  fn set_left(&self, hk: i32) {
    self.0.borrow_mut().settings.left = hk;
  }

  fn set_right(&self, hk: i32) {
    self.0.borrow_mut().settings.right = hk;
  }

  fn set_display(&self, hk: i32) {
    self.0.borrow_mut().settings.display = hk;
  }

  fn add_command(&self, cs: CommandSettings) {
    let mut config = self.0.borrow_mut();
    config.command.push(cs);
  }

  fn remove_command(&self, index: usize) {
    *self.1.borrow_mut() = Some(Operation::RemoveCommand(index));
  }

  fn move_command_up(&self, index: usize) {
    let b = index.saturating_sub(1);
    *self.1.borrow_mut() = Some(Operation::SwapCommand(index, b));
  }

  fn move_command_down(&self, index: usize) {
    *self.1.borrow_mut() = Some(Operation::SwapCommand(index, index + 1));
  }

  fn process_operation(&self) {
    let mut op = self.1.borrow_mut();

    match *op {
      Some(Operation::SwapCommand(a, b)) => {
        let mut config = self.0.borrow_mut();
        let a = usize::min(a, config.command.len() - 1);
        let b = usize::min(b, config.command.len() - 1);
        config.command.swap(a, b);
      }
      Some(Operation::RemoveCommand(a)) => {
        let mut config = self.0.borrow_mut();
        config.command.remove(a);
      }
      None => {}
    }

    *op = None;
  }

  fn save(&self) {
    let r = self.0.borrow().save_to_file(&std::path::Path::new(&self.2));
    println!("{:#?}", r);
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

  let fname = String::from(
    matches
      .value_of("file")
      .unwrap_or("jdsd_dsiii_practice_tool.toml"),
  );
  let conf = Config::load_from_file(&std::path::Path::new(&fname));

  match conf {
    Ok(conf) => editor(conf, fname),
    Err(e) => show_error(e),
  };
}
