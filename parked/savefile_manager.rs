#![windows_subsystem = "windows"]
use libjdsd_dsiii_practice_tool::utils::imgui_loop;

use chrono::prelude::*;
use imgui::*;
use winapi::um::winuser::GetAsyncKeyState;

const TITLE: &str = "Practice Tool Savefiles Manager";

mod database {
  use std::cell::RefCell;
  use std::collections::VecDeque;
  use std::path::PathBuf;

  use chrono::prelude::*;

  pub(super) struct Database {
    savefile_path: PathBuf,
    counter: RefCell<u32>,
    buffer: RefCell<VecDeque<(u32, DateTime<Local>, Vec<u8>)>>,
  }

  impl Database {
    pub(super) fn new() -> Result<Database, String> {
      let re = regex::Regex::new(r"^[a-f0-9]+$").unwrap();
      let savefile_path: PathBuf = [std::env::var("APPDATA").unwrap().as_str(), "DarkSoulsIII"]
        .iter()
        .collect();
      let savefile_path = std::fs::read_dir(&savefile_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| re.is_match(&e.file_name().to_string_lossy()) && e.path().is_dir())
        .map(|e| e.path())
        .map(|mut e| {
          e.push("DS30000.sl2");
          e
        })
        .map(PathBuf::from)
        .ok_or_else(|| "Could not find savefile position".to_string())?;

      Ok(Database {
        savefile_path,
        counter: RefCell::new(0),
        buffer: RefCell::new(VecDeque::new()),
      })
    }

    pub(super) fn save(&self) -> Result<(), String> {
      let savefile_content = std::fs::read(&self.savefile_path)
        .map_err(|e| format!("Could not read savefile: {}", e))?;

      let mut buffer = self.buffer.borrow_mut();
      let mut next_id = self.counter.borrow_mut();
      *next_id += 1;
      buffer.push_back((*next_id, Local::now(), savefile_content));
      while buffer.len() > 10 {
        buffer.pop_front();
      }
      Ok(())
    }

    pub(super) fn load(&self) -> Result<(), String> {
      match self.buffer.borrow().iter().last() {
        Some((_, _, savefile_content)) => std::fs::write(&self.savefile_path, &savefile_content)
          .map_err(|e| format!("Could not write savefile: {}", e)),
        None => Err("Savefile list empty".to_string()),
      }
    }

    pub(super) fn pop(&self) -> Result<(), String> {
      if let None = self.buffer.borrow_mut().pop_back() {
        Err("Could not pop savefile".to_string())
      } else {
        Ok(())
      }
    }

    pub(super) fn retrieve(&self) -> Result<Vec<(u32, DateTime<Local>)>, String> {
      Ok(
        self
          .buffer
          .borrow()
          .iter()
          .map(|(id, time, _)| (*id, time.clone()))
          .collect(),
      )
    }
  }
}

fn main() {
  let db = database::Database::new();

  let mut was_pop_pressed: bool = false;
  let mut was_save_pressed: bool = false;
  let mut was_load_pressed: bool = false;
  let mut events: Vec<(ImString, DateTime<Local>)> = Vec::new();
  let mut state: Vec<(ImString, DateTime<Local>)> = Vec::new();

  fn retrieve(db: &database::Database) -> Vec<(ImString, DateTime<Local>)> {
    let mut s = match db.retrieve() {
      Ok(s) => s
        .into_iter()
        .map(|(id, time)| {
          (
            ImString::from(format!("{:>8} {}", id, time.format("%T").to_string())),
            time,
          )
        })
        .collect(),
      Err(e) => {
        vec![(ImString::from(e), Local::now())]
      }
    };

    let epoch = Local.timestamp(0, 0);
    while s.len() < 10 {
      s.push((ImString::new(""), epoch));
    }

    s
  }

  match db {
    Ok(db) => imgui_loop(TITLE, move |_, ui, display| {
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

      let pop_key_state = unsafe { GetAsyncKeyState('1' as _) & 0x01 } != 0;
      let save_key_state = unsafe { GetAsyncKeyState('5' as _) & 0x01 } != 0;
      let load_key_state = unsafe { GetAsyncKeyState('9' as _) & 0x01 } != 0;

      if save_key_state && !was_save_pressed {
        match db.save() {
          Ok(_) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} State saved",
              dt.format("%T").to_string()
            )), Local::now()));
          }
          Err(e) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} {}",
              dt.format("%T").to_string(),
              e
            )), Local::now()));
          }
        }

        state = retrieve(&db);
      }

      if load_key_state && !was_load_pressed {
        match db.load() {
          Ok(_) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} State loaded",
              dt.format("%T").to_string()
            )), Local::now()));
          }
          Err(e) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} {}",
              dt.format("%T").to_string(),
              e
            )), Local::now()));
          }
        }

        state = retrieve(&db);
      }

      if pop_key_state && !was_pop_pressed {
        match db.pop() {
          Ok(_) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} State popped",
              dt.format("%T").to_string()
            )), Local::now()));
          }
          Err(e) => {
            let dt = Local::now();
            events.push((ImString::from(format!(
              "{} {}",
              dt.format("%T").to_string(),
              e
            )), Local::now()));
          }
        }

        state = retrieve(&db);
      }

      was_save_pressed = save_key_state;
      was_load_pressed = load_key_state;
      was_pop_pressed = pop_key_state;

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
          let messages = events.iter().rev().take(10);
          ui.text(im_str!("Save: 5 / Load: 9 / Pop: 1"));

          let tok = ui.push_item_width(-1.);

          ui.separator();

          let now = Local::now();
          let state = state.iter().take(10);
          for (label, time) in state {
            let elapsed = (now - *time).to_std().unwrap();
            let pct = 1. - (elapsed.as_millis() as f32 / 500.0).clamp(0., 1.);
            let [x, y] = ui.cursor_screen_pos();
            let th = ui.calc_text_size(&label, false, 10000.0)[1];
            ui.text(&label);
            let dl = ui.get_window_draw_list();
            dl.add_rect_filled_multicolor(
              [x, y],
              [x + ui.window_size()[0], y + th],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
            );
          }

          ui.separator();

          for (label, time) in messages {
            let elapsed = (now - *time).to_std().unwrap();
            let pct = 1. - (elapsed.as_millis() as f32 / 500.0).clamp(0., 1.);
            let [x, y] = ui.cursor_screen_pos();
            let th = ui.calc_text_size(&label, false, 10000.0)[1];
            ui.text(&label);
            let dl = ui.get_window_draw_list();
            dl.add_rect_filled_multicolor(
              [x, y],
              [x + ui.window_size()[0], y + th],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
              [1., 1., 1., pct],
            );
          }

          tok.pop(ui);
        });

      stack_token.pop(ui);
    }),
    Err(e) => imgui_loop(TITLE, move |_, ui, display| {
      let (_width, _height) = {
        let s = display.gl_window().window().inner_size();
        (s.width as f32, s.height as f32)
      };

      ui.text(im_str!("{}", e));
    }),
  };
}
